mod brokers;
mod config;

use brokers::create_broker_connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    config::load_env();

    let addr = std::env::var("FASTTQ_BROKER_ADDR")?;

    let broker = create_broker_connection(addr).await?;

    broker.register_queue("test_queue").await?;
    broker
        .publish_message("", "test_queue", b"Hello, RabbitMQ!")
        .await?;

    Ok(())
}
