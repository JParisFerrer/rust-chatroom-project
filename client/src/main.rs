use clap::{Parser};
use client::{ChatClient, ChatClientConfig};
use tokio;

#[tokio::main]
async fn main() {
    let config = ChatClientConfig::parse();

    let client = ChatClient::new(config);

    client.run().await;
}
