use server::{ServerRunner, ServerRunnerConfig};
use tokio;

#[tokio::main]
async fn main() {
    let server = ServerRunner::new(ServerRunnerConfig { port: 1337 });
    server.run().await;
}
