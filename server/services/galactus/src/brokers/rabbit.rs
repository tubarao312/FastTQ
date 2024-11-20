use lapin::{options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties};
use std::sync::Arc;

pub async fn create_connection() -> Result<Arc<Connection>, Box<dyn std::error::Error>> {
    let addr = std::env::var("AMQP_ADDR")?;
    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;

    // Change to loggin in the future
    println!("Connected to RabbitMQ!");

    Ok(Arc::new(conn))
}

pub async fn register_queue(
    conn: Arc<Connection>,
    queue_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let channel = conn.create_channel().await?;
    channel
        .queue_declare(
            queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    Ok(())
}

pub async fn publish_message(
    conn: Arc<Connection>,
    exchange: &str,
    routing_key: &str,
    payload: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let channel = conn.create_channel().await?;
    channel
        .basic_publish(
            exchange,
            routing_key,
            BasicPublishOptions::default(),
            payload,
            BasicProperties::default(),
        )
        .await?
        .await?;

    Ok(())
}
