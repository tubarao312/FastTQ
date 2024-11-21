mod brokers;
mod config;

use brokers::create_broker_connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::new();
    let broker = create_broker_connection(config.broker_addr().to_string()).await?;

    broker.register_queue("test_queue").await?;
    broker
        .publish_message("", "test_queue", b"Hello, RabbitMQ!")
        .await?;

    Ok(())
}
