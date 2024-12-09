use crate::brokers::core::BrokerCore;
use async_trait::async_trait;
use lapin::{
    options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties, ExchangeKind,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct RabbitBroker {
    connection: Arc<Connection>,
}

impl RabbitBroker {
    pub async fn new(uri: &str) -> Result<RabbitBroker, Box<dyn std::error::Error>> {
        let connection = Connection::connect(uri, ConnectionProperties::default()).await?;

        Ok(RabbitBroker {
            connection: Arc::new(connection),
        })
    }
}

#[async_trait]
impl BrokerCore for RabbitBroker {
    async fn register_queue(
        &self,
        queue_name: &str,
        exchange: &str,
        routing_key: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let channel = self.connection.create_channel().await?;
        channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        channel
            .exchange_declare(
                exchange,
                ExchangeKind::Direct,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        channel
            .queue_bind(
                queue_name,
                exchange,
                routing_key,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(())
    }

    async fn publish_message(
        &self,
        exchange: &str,
        routing_key: &str,
        payload: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let channel = self.connection.create_channel().await?;

        channel
            .basic_publish(
                exchange,
                routing_key,
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            )
            .await?;

        Ok(())
    }
}
