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

        // Create durable queue
        channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions {
                    durable: true,
                    auto_delete: false,
                    ..QueueDeclareOptions::default()
                },
                FieldTable::default(),
            )
            .await?;

        // Create durable exchange
        channel
            .exchange_declare(
                exchange,
                ExchangeKind::Direct,
                ExchangeDeclareOptions {
                    durable: true,
                    ..ExchangeDeclareOptions::default()
                },
                FieldTable::default(),
            )
            .await?;

        // Bind using worker name as routing key
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
        message_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let channel = self.connection.create_channel().await?;

        channel
            .basic_publish(
                exchange,
                routing_key,
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default().with_message_id(message_id.into()),
            )
            .await?;

        Ok(())
    }
}
