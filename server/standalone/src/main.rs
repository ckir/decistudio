use anyhow::Result;
use decistudio_server_standalone_core::run_server;

#[tokio::main]
async fn main() -> Result<()> {
    run_server().await
}
