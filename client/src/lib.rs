use anyhow::{Context, Error};
use proto_chatroom::chatroom as proto;
use shared::{into_proto_stream, read_next_message, write_message, Shutdown};
use std::collections::HashMap;
use std::io;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::thread;
use tokio;
use tokio::net::{tcp, TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio_stream::StreamExt;
use clap::{Parser};

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct ChatClientConfig {
    #[clap(short, long, default_value_t = String::from("127.0.0.1"))]
    pub host: String,
    #[clap(short, long, value_parser = clap::value_parser!(u16).range(1..), default_value_t = 1337)]
    pub port: u16,
    #[clap(short, long, default_value_t = String::from("User1"))]
    pub username: String,
}

// Ops that tasks can send to the main task to run
#[derive(Debug, Clone)]
enum MainOp {
    CommenceShutdown { reason: String },
    SendChat { msg: String },
    ReceiveChat { msg: proto::RecvChat },
    OtherUserLeft {username: String},
    OtherUserJoined {username: String},
}
//
// Ops that are sent to the network sender
#[derive(Debug, Clone)]
enum WriterOp {
    SendChat { msg: String },
}

// Thick client to the chat server. ie, it handles network communication but also
// reading & writing to Terminal.
pub struct ChatClient {
    config: ChatClientConfig,
}

impl ChatClient {
    async fn initiate_connection(&mut self) -> (TcpStream, proto::JoinResponse_Success) {
        // This is not good.
        let addr_str = format!("{}:{}", self.config.host, self.config.port);

        let mut tcp_stream = match TcpStream::connect(&addr_str).await {
            Ok(tcp_stream) => tcp_stream,
            Err(e) => panic!("Error connecting to {:?}: {:?}", addr_str, e),
        };

        println!("Connected to {:?}", addr_str);

        // Perform the initial client connection protocol
        write_message(
            &mut tcp_stream,
            &proto::ClientMessageWrapper {
                inner_message: proto::ClientMessageWrapper_InnerMessage::JoinMsg(
                    proto::JoinRequest {
                        username: self.config.username.clone(),
                    },
                ),
            },
        )
        .await
        .context("Failed to write join request")
        .unwrap();

        let join_success =
            match read_next_message::<_, proto::ServerMessageWrapper>(&mut tcp_stream)
                .await
                .context("failed to read join response")
                .unwrap()
            {
                proto::ServerMessageWrapper {
                    inner_message:
                        proto::ServerMessageWrapper_InnerMessage::JoinRespMsg(join_response),
                } => match join_response.response {
                    proto::JoinResponse_Response::Success(succ) => succ,

                    proto::JoinResponse_Response::Failure(fail) => {
                        panic!("Server rejected join: {}", fail.error_message);
                    }
                },
                other => panic!("Received unexpected join response: {:?}", other),
            };

        println!("Joined {:?} as {}", addr_str, self.config.username);

        (tcp_stream, join_success)
    }

    fn cli_loop(mut op_sender: mpsc::Sender<MainOp>, shutdown: Shutdown) {
        loop {
            if shutdown.is_shutdown() {
                break;
            }

            let mut line = String::new();
            let n = io::stdin()
                .read_line(&mut line)
                .map_err(anyhow::Error::new)
                .context("Reading stdin")
                .unwrap();
            if n == 0 {
                eprintln!("EOF?");
                break;
            }

            if line.trim() == "!!quit" {
                op_sender.blocking_send(MainOp::CommenceShutdown {
                    reason: "!!quit".to_owned(),
                });
                break;
            } else if line.trim().len() == 0 {
                continue
            }

            op_sender.blocking_send(MainOp::SendChat { msg: line.trim_right().to_owned() });
        }

        println!("CLI input thread shutting down");
    }

    async fn send_loop(
        mut tcp_sender: tcp::OwnedWriteHalf,
        mut shutdown: Shutdown,
        username: String,
        mut write_ops: mpsc::Receiver<WriterOp>,
    ) {
        loop {
            tokio::select! {
                msg = write_ops.recv() => {
                    match msg {
                        Some(WriterOp::SendChat{msg}) => {
                            write_message(&mut tcp_sender, &proto::ClientMessageWrapper{
                                inner_message: proto::ClientMessageWrapper_InnerMessage::ChatMsg(
                                                   proto::SendChat{
                                                       msg
                                                   }
                                                   )

                            }).await.context("send chat").unwrap();
                        }
                        None => break,
                    }

                }

                _ = shutdown.wait_for_shutdown() => {
                    break;
                }
            }
        }

        println!("Shutting down send loop");
    }

    async fn read_loop(
        mut tcp_reader: tcp::OwnedReadHalf,
        mut shutdown: Shutdown,
        mut op_sender: mpsc::Sender<MainOp>,
    ) {

        let mut message_stream = Box::pin(into_proto_stream::<_, proto::ServerMessageWrapper>(
            tcp_reader,
        ));

        loop {
            tokio::select! {
                maybe_msg = message_stream.next() => {
                    if let Some(msg) = maybe_msg {
                        match msg.inner_message {
                            proto::ServerMessageWrapper_InnerMessage::ChatMsg(recv_chat) => {
                                let _ = op_sender.send(MainOp::ReceiveChat{msg: recv_chat}).await;
                            }
                            proto::ServerMessageWrapper_InnerMessage::UserJoinMsg(user_join) => {
                                let _ = op_sender.send(MainOp::OtherUserJoined{username: user_join.username}).await;
                            }
                            proto::ServerMessageWrapper_InnerMessage::UserLeftMsg(user_leave) => {
                                let _ = op_sender.send(MainOp::OtherUserLeft{username: user_leave.username}).await;
                            }
                            o => {
                                panic!("Unhandled server message: {:?}", o);
                            }
                        }
                    } else {
                        // End of stream -> connection has closed
                        op_sender.blocking_send(MainOp::CommenceShutdown {
                            reason: "Server connection closed (read)".to_owned(),
                        });
                        break;
                    }

                }

                _ = shutdown.wait_for_shutdown() => {
                    break;
                }
            }
        }

        println!("Shutting down read loop");
    }

    fn print_chat(chat: &proto::RecvChat) {
        println!("[{}] {}", chat.username, chat.msg);
    }

    // Public API
    pub fn new(config: ChatClientConfig) -> Self {
        ChatClient { config }
    }

    /// Begin running the client. Connects to the server, and also begins reading & writing to the terminal.
    pub async fn run(mut self) {
        let (ops_sender, mut ops_receiver) = mpsc::channel::<MainOp>(1024);
        let (network_sender, network_receiver) = mpsc::channel::<WriterOp>(1024);

        let (shutdown_notify_tx, shutdown_notify_rx) = broadcast::channel::<()>(1024);
        let (shutdown_ack_tx, mut shutdown_ack_rx) = mpsc::channel::<()>(1024);

        let (tcp_stream, join_success) = self.initiate_connection().await;

        for recv_chat in join_success.initial_chats.iter() {
            ChatClient::print_chat(recv_chat);
        }

        // Split the TCP connection and spawn a reading & writing task
        let (tcp_reader, tcp_writer) = tcp_stream.into_split();
        tokio::spawn(ChatClient::send_loop(
            tcp_writer,
            Shutdown::new(shutdown_notify_tx.clone(), shutdown_ack_tx.clone()),
            self.config.username.clone(),
            network_receiver,
        ));
        tokio::spawn(ChatClient::read_loop(
                tcp_reader,
                Shutdown::new(shutdown_notify_tx.clone(), shutdown_ack_tx.clone()),
                ops_sender.clone(),
                ));

        // Spawn a thread to read input from the CLI. We use a thread and not a task as suggested
        // by Tokio, in order to not hang the runtime waiting on Enter
        let ops_sender_clone = ops_sender.clone();
        let shutdown = Shutdown::new(shutdown_notify_tx.clone(), shutdown_ack_tx.clone());
        let t_handle = thread::spawn(move || ChatClient::cli_loop(ops_sender_clone, shutdown));

        // Main loop where we receive & handle requests from the spawnd tasks
        loop {
            tokio::select! {
                maybe_op = ops_receiver.recv() => {
                    if maybe_op.is_none() {
                        eprintln!("all senders dead?");
                        break;
                    }

                    match maybe_op.unwrap() {
                        MainOp::CommenceShutdown{reason} => {
                            println!("Shutdown requested: {}", reason);
                            break
                        }

                        MainOp::SendChat{msg} => {
                            let _ = network_sender.send(WriterOp::SendChat{msg}).await;
                        }

                        MainOp::ReceiveChat{msg} => {
                            ChatClient::print_chat(&msg);
                        }

                        MainOp::OtherUserJoined{username} => {
                            // Text output should be improved
                            ChatClient::print_chat(&proto::RecvChat{username: "Server".to_owned(),
                            msg: format!("User '{}' has joined.", username)});
                        }

                        MainOp::OtherUserLeft{username} => {
                            ChatClient::print_chat(&proto::RecvChat{username: "Server".to_owned(),
                            msg: format!("User '{}' has left.", username)});
                        }
                    };
                }

                _ = tokio::signal::ctrl_c() => {
                    println!("Received ctrl C, quitting");
                    break;
                }
            };
        }

        shutdown_notify_tx
            .send(())
            .expect("failed to send shutdown signal");
        drop(shutdown_ack_tx);
        drop(ops_receiver);

        // This will wait until all senders have dropped, then return an Err
        let _ = shutdown_ack_rx.recv().await;

        // Also, let the thread clean exit. This shouldn't block long, but only because the recv()
        // itself is going to block for the CLI thread to also drop its Shutdown after pressing
        // Enter
        t_handle.join().unwrap();
    }
}
