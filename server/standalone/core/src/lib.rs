use anyhow::Result;
use decistudio_server_shared::init_logging;

/// Core logic for the standalone server.
/// This will later host REST API, WebSockets, Postgres, etc.
pub async fn run_server() -> Result<()> {
    init_logging();
    println!("decistudio standalone server core stub");
    Ok(())
}
