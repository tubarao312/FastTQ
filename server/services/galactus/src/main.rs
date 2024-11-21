mod brokers;
mod config;

use crate::brokers::base::BaseBroker;
use brokers::rabbit;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    config::load_env();

    let addr = std::env::var("FASTTQ_BROKER_ADDR")?;

    let rabbit = rabbit::RabbitBroker;
    let conn = rabbit.connect(addr).await?;

    rabbit.register_queue(conn.clone(), "test_queue").await?;
    rabbit
        .publish_message(conn.clone(), "", "test_queue", b"Hello, RabbitMQ!")
        .await?;

    Ok(())
}
