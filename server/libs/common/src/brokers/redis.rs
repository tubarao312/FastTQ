use crate::brokers::base::BaseBroker;
use async_trait::async_trait;
use redis::{AsyncCommands, Client, RedisResult};
use std::sync::Arc;

#[derive(Clone)]
pub struct RedisBroker {
    client: Arc<Client>,
}

impl RedisBroker {
    pub async fn new(uri: &str) -> Result<RedisBroker, Box<dyn std::error::Error>> {
        let client = Client::open(uri)?;

        Ok(RedisBroker {
            client: Arc::new(client),
        })
    }
}

#[async_trait]
impl BaseBroker for RedisBroker {
    async fn register_queue(&self, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Redis does not have the concept of queues, so we don't need to do anything here
        Ok(())
    }

    async fn publish_message(
        &self,
        exchange: &str,
        _routing_key: &str,
        payload: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let result: RedisResult<()> = conn.publish(exchange, payload).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
