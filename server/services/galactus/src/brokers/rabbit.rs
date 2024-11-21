use crate::brokers::base::BaseBroker;
use lapin::{options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties};
use std::sync::Arc;

pub struct RabbitBroker;

impl BaseBroker<Connection> for RabbitBroker {
    async fn connect(&self, uri: String) -> Result<Arc<Connection>, Box<dyn std::error::Error>> {
        let conn = Connection::connect(&uri, ConnectionProperties::default()).await?;

        // Change to loggin in the future
        println!("Connected to RabbitMQ!");

        Ok(Arc::new(conn))
    }

    async fn register_queue(
        &self,
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

    async fn publish_message(
        &self,
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
}
