use client::{ChatClient, ChatClientConfig};
use tokio;

#[tokio::main]
async fn main() {
    // TODO: get from CLI args
    let host = "127.0.0.1".to_owned();
    let port = 1337;
    let username = "User1".to_owned();

    let client = ChatClient::new(ChatClientConfig {
        host,
        port,
        username,
    });

    client.run().await;
}
