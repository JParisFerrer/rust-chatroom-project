use anyhow::Context;
use proto_chatroom::chatroom as proto;
use shared::{into_proto_stream, read_next_message, write_message, Shutdown};
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use tokio;
use tokio::net::{tcp, TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio_stream::StreamExt;

#[derive(Debug, Clone)]
pub struct ChatClientConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
}

// Ops that tasks can send to the main task to run
#[derive(Debug, Clone)]
enum MainOp {}

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
        .await.context("Failed to write join request").unwrap();

        let join_success = match read_next_message::<_, proto::ServerMessageWrapper>(&mut tcp_stream).await.context("failed to read join response").unwrap() {
            proto::ServerMessageWrapper {
                inner_message:
                    proto::ServerMessageWrapper_InnerMessage::JoinRespMsg(join_response),
            } => {

                match join_response.response {
                    proto::JoinResponse_Response::Success(succ) => {
                        succ
                    },

                    proto::JoinResponse_Response::Failure(fail) => {
                        panic!("Server rejected join: {}", fail.error_message);
                    },
                }
            }
            other => panic!("Received unexpected join response: {:?}", other),
        };

        println!("Joined {:?} as {}", addr_str, self.config.username);

        (tcp_stream, join_success)
    }

    // Public API
    pub fn new(config: ChatClientConfig) -> Self {
        ChatClient { config }
    }

    /// Begin running the client. Connects to the server, and also begins reading & writing to the terminal.
    pub async fn run(mut self) {
        let (ops_sender, mut ops_receiver) = mpsc::channel::<MainOp>(1024);
        let (network_sender, network_receiver) = mpsc::channel::<proto::ClientMessageWrapper>(1024);

        let (tcp_stream, join_success) = self.initiate_connection().await;

        // Split the TCP connection and spawn a reading & writing task
        let (tcp_reader, tcp_writer) = tcp_stream.into_split();
        // TODO

        // Spawn a task to read input from the CLI
        // TODO

        // Main loop where we receive & handle requests from the spawnd tasks
        loop {
            // TODO
        }
    }
}
