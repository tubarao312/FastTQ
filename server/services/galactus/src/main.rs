mod brokers;
mod config;

use brokers::rabbit;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    config::load_env();

    let conn = rabbit::create_connection().await?;

    rabbit::register_queue(Arc::clone(&conn), "test_queue").await?;
    rabbit::publish_message(Arc::clone(&conn), "", "test_queue", b"Hello, world!").await?;

    Ok(())
}
