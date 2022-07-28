use anyhow::{Context, Error};
use proto_chatroom::chatroom as proto;
use shared::{into_proto_stream, write_message, Shutdown};
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use tokio;
use tokio::net::{tcp, TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio_stream::StreamExt;

#[derive(Copy, Clone, Debug)]
pub struct ServerRunnerConfig {
    pub port: u16,
}

pub struct ServerRunner {
    config: ServerRunnerConfig,

    connected_users: HashMap<String, ()>,
    writer_broadcast_sender: broadcast::Sender<WriterOp>,

    // This should be a full log of all events, real type, etc
    message_history: Vec<(String, proto::SendChat)>,
}

// Specific pre-parsed commands that the stdin reader will send to the main thread to run
#[derive(Debug)]
enum StdinCommand {}

// Ops various threads will ask the "main thread" to run
#[derive(Debug)]
enum MainOp {
    // user read-task has parsed an initial connection request message.
    // The main task decides whether to accept it or not, sending the
    // result back using the oneshot channel and starting a write-task if so.
    NewUserConnection {
        username: String,
        sender: tcp::OwnedWriteHalf,
        connection_accepted: oneshot::Sender<bool>,
    },
    NewChat {
        username: String,
        contents: proto::SendChat,
    },
    UserLeft {
        username: String,
    },
    HandleStdin(StdinCommand),
}

// Ops the main thread will ask the writer threads to run
#[derive(Debug, Clone)]
enum WriterOp {
    NewChat {
        username: String,
        contents: proto::SendChat,
    },
    UserLeft {
        username: String,
    },
}

impl ServerRunner {
    // Task closures

    /// accept incoming connections, and spawn new tasks to handle them
    async fn connection_loop(
        config: ServerRunnerConfig,
        main_op_channel: mpsc::Sender<MainOp>,
        mut shutdown: Shutdown,
    ) {
        // Start a listener
        let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), config.port))
            .await
            .expect("failed to listen");

        println!("Server listening on port {}", config.port);

        // Loop until shutdown accepting connections
        loop {
            tokio::select! {
                maybe_conn = listener.accept() => {
                    match maybe_conn {
                        Ok((sock, addr)) => {
                            tokio::spawn(ServerRunner::user_read_loop(sock, addr, shutdown.clone(), main_op_channel.clone()));
                        }
                        Err(err) => {
                            eprintln!("Error on accept! {}", err);
                        }
                    }
                }
                _ = shutdown.wait_for_shutdown() => {
                    println!("Shutting down connection loop");
                    break;
                }
            }
        }
    }

    async fn user_read_loop(
        tcp_stream: TcpStream,
        remote_addr: SocketAddr,
        mut shutdown: Shutdown,
        main_op_sender: mpsc::Sender<MainOp>,
    ) {
        println!("Accepted connection from {}", remote_addr);

        let mut username: Option<String> = None;
        let (tcp_read, tcp_write) = tcp_stream.into_split();
        let mut tcp_write = Some(tcp_write);

        let mut message_stream = Box::pin(into_proto_stream::<_, proto::ClientMessageWrapper>(
            tcp_read,
        ));

        loop {
            tokio::select! {
                maybe_msg = message_stream.next() => {

                    if let Some(msg) = maybe_msg {
                        match (&username, msg.inner_message) {
                            // Expected cases
                            (None, proto::ClientMessageWrapper_InnerMessage::JoinMsg(join_msg)) => {
                                // regular connection request
                                username = Some(join_msg.username);
                                let (accepted_tx, accepted_rx) = oneshot::channel();

                                let tcp_writer = tcp_write.take().expect("accepted two JoinMsg");
                                let _ = main_op_sender.send(MainOp::NewUserConnection{
                                    username: username.as_ref().unwrap().clone(),
                                    sender: tcp_writer,
                                    connection_accepted: accepted_tx,
                                }).await;

                                let accepted = accepted_rx.await.unwrap_or(false);
                                if !accepted {
                                    eprintln!("{:?}: User {:?} join rejected", remote_addr, username);
                                    break;
                                }
                            }
                            (Some(username), proto::ClientMessageWrapper_InnerMessage::ChatMsg(chat_msg)) => {
                                // Regular chat message
                                let _ = main_op_sender.send(MainOp::NewChat{
                                    username: username.clone(),
                                    contents: chat_msg
                                }).await;
                            }
                            // Error scenarios
                            (Some(username), proto::ClientMessageWrapper_InnerMessage::JoinMsg(join_msg)) => {
                                // Duplicate JoinMsg, send a user leave and close their connection
                                let _ = main_op_sender.send(MainOp::UserLeft{username: username.clone()}).await;
                                eprintln!("{:?}: User {:?} received duplicate JoinMsg", remote_addr, username);
                                break;
                            }
                            (None, proto::ClientMessageWrapper_InnerMessage::ChatMsg(chat_msg)) => {
                                // Chat before providing the join message, close their connection
                                eprintln!("{:?}: Received chat before JoinMsg, closing", remote_addr);
                                break;
                            }
                        };
                    } else {
                        // End of stream -> connection has closed
                        if let Some(ref username) = username {
                            let _ = main_op_sender.send(MainOp::UserLeft{username: username.clone()}).await;
                        }
                        break;
                    }
                }

                _ = shutdown.wait_for_shutdown() => {
                    println!("{:?}: User {:?} shutting down readloop", remote_addr, username);
                    break;
                }
            }
        }

        println!("{:?}: User {:?} disconnected", remote_addr, username);
    }

    async fn user_write_loop(
        mut tcp_sender: tcp::OwnedWriteHalf,
        username: String,
        mut broadcast_rx: broadcast::Receiver<WriterOp>,
        mut shutdown: Shutdown,
    ) {
        // This should probably not expect()
        let remote_addr = tcp_sender.peer_addr().expect("failed to get peer addr");
        println!(
            "{:?}: write loop for User {:?} started",
            remote_addr, username
        );

        loop {
            tokio::select! {
                maybe_writer_op = broadcast_rx.recv() => {
                    match maybe_writer_op {
                        Ok(WriterOp::NewChat{username: chat_sender_username, contents}) => {
                            if chat_sender_username == username {
                                continue;
                            }

                            let network_proto = proto::ServerMessageWrapper{inner_message: proto::ServerMessageWrapper_InnerMessage::ChatMsg(proto::RecvChat{username: chat_sender_username, msg:contents.msg})};
                            if let Err(e) = write_message(&mut tcp_sender, &network_proto).await {
                                eprintln!("{:?}: write loop for User {:?}: error sending message {:?}", remote_addr, username, e);
                                break;
                            }
                        }
                        Ok(WriterOp::UserLeft{username: msg_username}) => {
                            if msg_username == username {
                                continue;
                            }

                            let network_proto = proto::ServerMessageWrapper{inner_message: proto::ServerMessageWrapper_InnerMessage::UserLeftMsg(proto::UserLeft{username: msg_username})};
                            if let Err(e) = write_message(&mut tcp_sender, &network_proto).await {
                                eprintln!("{:?}: write loop for User {:?}: error sending message {:?}", remote_addr, username, e);
                            }
                        }
                        Err(broadcast::error::RecvError::Lagged(by)) => {
                            // This would mean dropped chats; for now, simply panic the entire
                            // server as there is no way atm of having this send that the user has
                            // "left"
                            panic!("{:?}: write loop for User {:?} lagged by {} events", remote_addr, username, by);
                        }
                        Err(broadcast::error::RecvError::Closed) => {
                            eprintln!("{:?}: write loop for User {:?}: all senders closed", remote_addr, username);
                            break;
                        }
                    }
                }

                _ = shutdown.wait_for_shutdown() => {
                    break;
                }
            }
        }

        println!(
            "{:?}: write loop for User {:?} shutting down",
            remote_addr, username
        );
    }

    // Server helpers

    // Handles NewUserConnection on the main thread by starting up a thread to read the values, registering any metadata, etc.
    async fn handle_new_connection(
        &mut self,
        username: String,
        mut sender: tcp::OwnedWriteHalf,
        connection_accepted: oneshot::Sender<bool>,
        shutdown: Shutdown,
    ) {
        // Reject duplicate usernames
        if self.connected_users.contains_key(&username) {
            // Since we are rejecting anyways, do not care if client already hung up
            let _ = connection_accepted.send(false);

            // Let the client know why they failed to connect
            if let Err(e) = write_message(
                &mut sender,
                &proto::ServerMessageWrapper {
                    inner_message: proto::ServerMessageWrapper_InnerMessage::JoinRespMsg(
                        proto::JoinResponse {
                            response: proto::JoinResponse_Response::Failure(
                                proto::JoinResponse_Failure {
                                    error_message: "Username taken".to_owned(),
                                },
                            ),
                        },
                    ),
                },
            )
            .await
            {
                eprintln!(
                    "Failed to give {} connection failure reason: {:?}",
                    username, e
                );
            }

            // send half gets dropped automatically
            return;
        }

        let initial_chats: Vec<proto::RecvChat> = std::iter::once(proto::RecvChat {
            username: "FakeUser".to_owned(),
            msg: "some chat".to_owned(),
        })
        .chain(self.message_history.iter().map(|(username, send_chat)| {
            proto::RecvChat {
                username: username.clone(),
                msg: send_chat.msg.clone(),
            }
        }))
        .collect();

        if let Err(e) = write_message(
            &mut sender,
            &proto::ServerMessageWrapper {
                inner_message: proto::ServerMessageWrapper_InnerMessage::JoinRespMsg(
                    proto::JoinResponse {
                        response: proto::JoinResponse_Response::Success(
                            proto::JoinResponse_Success {
                                initial_chats: initial_chats,
                            },
                        ),
                    },
                ),
            },
        )
        .await
        {
            eprintln!("Failed to give {} initial chats: {:?}", username, e);
            // Since we are rejecting anyways, do not care if client already hung up
            let _ = connection_accepted.send(false);
            return;
        }

        // Only after successfully sending the initial chats, accept the username
        // and start a sending task for it.
        self.connected_users.insert(username.clone(), ());

        if let Err(err) = connection_accepted.send(true) {
            eprintln!(
                "user {} failed to accept connection, client disconnected? {}",
                username, err
            );
            return;
        }

        tokio::spawn(ServerRunner::user_write_loop(
            sender,
            username,
            self.writer_broadcast_sender.subscribe(),
            shutdown,
        ));
    }

    // Handler for something notifying main app that a user has left
    async fn handle_user_left(&mut self, username: String) {
        if !self.connected_users.contains_key(&username) {
            eprintln!("UserLeft for {} which is not connected", username);
        }

        // If the last user leaves, this would return Err, which is fine.
        let _ = self.writer_broadcast_sender.send(WriterOp::UserLeft {
            username: username.clone(),
        });

        self.connected_users.remove(&username);
    }

    async fn handle_new_chat(&mut self, username: String, contents: proto::SendChat) {
        // Store the message
        self.message_history
            .push((username.clone(), contents.clone()));

        // broadcast it out to send to clients
        let _ = self
            .writer_broadcast_sender
            .send(WriterOp::NewChat { username, contents });
    }

    // Public API
    pub fn new(config: ServerRunnerConfig) -> ServerRunner {
        ServerRunner {
            config: config,
            connected_users: HashMap::new(),
            writer_broadcast_sender: broadcast::channel(1024).0,
            message_history: vec![],
        }
    }

    /// Run the server. Blocks until exit
    pub async fn run(mut self) {
        let (ops_sender, mut ops_receiver) = mpsc::channel::<MainOp>(1024);
        let (shutdown_notify_tx, shutdown_notify_rx) = broadcast::channel::<()>(1024);
        let (shutdown_ack_tx, mut shutdown_ack_rx) = mpsc::channel::<()>(1024);

        // Start listening for incoming connections
        tokio::spawn(ServerRunner::connection_loop(
            self.config,
            ops_sender.clone(),
            Shutdown::new(shutdown_notify_tx.clone(), shutdown_ack_tx.clone()),
        ));

        // Start parsing CLI
        // TODO

        // Main loop
        loop {
            tokio::select! {
                maybe_op = ops_receiver.recv() => {
                    if maybe_op.is_none() {
                        eprintln!("all senders dead?");
                        break;
                    }

                    match maybe_op.unwrap() {
                        MainOp::NewUserConnection {
                            username,
                            sender,
                            connection_accepted,
                        } => {
                            self.handle_new_connection(username, sender, connection_accepted, Shutdown::new(shutdown_notify_tx.clone(), shutdown_ack_tx.clone())).await
                        }
                        MainOp::UserLeft {username} => {
                            self.handle_user_left(username).await;
                        }
                        MainOp::NewChat{username, contents} => {
                            self.handle_new_chat(username, contents).await;
                        }
                        o => panic!("unhandled op! {:?}", o),
                    };
                }

                _ = tokio::signal::ctrl_c() => {
                    println!("Received ctrl C, quitting");
                    break;
                }
            }
        }

        // Gracefully shutdown by signalling all tasks to shutdown, and wait for them to do so.

        shutdown_notify_tx
            .send(())
            .expect("failed to send shutdown signal");
        drop(shutdown_ack_tx);

        // This will wait until all senders have dropped, then return an Err
        let _ = shutdown_ack_rx.recv().await;
    }
}
