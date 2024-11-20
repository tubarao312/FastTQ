mod brokers;
mod config;

use crate::brokers::queue::QueueBroker;
use brokers::rabbit;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    config::load_env();

    let rabbit = rabbit::RabbitBroker;
    let conn = rabbit.connect().await?;

    rabbit.register_queue(conn.clone(), "test_queue").await?;
    rabbit
        .publish_message(conn.clone(), "", "test_queue", b"Hello, RabbitMQ!")
        .await?;

    Ok(())
}
