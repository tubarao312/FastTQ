use crate::brokers::base::BaseBroker;
use redis::{AsyncCommands, Client, RedisResult};
use std::sync::Arc;

pub struct RedisBroker;

impl BaseBroker<Client> for RedisBroker {
    async fn connect(&self, uri: String) -> Result<Arc<Client>, Box<dyn std::error::Error>> {
        let client = Client::open(uri)?;
        Ok(Arc::new(client))
    }

    async fn register_queue(
        &self,
        _: Arc<Client>,
        _: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Redis does not have the concept of queues, so we don't need to do anything here
        Ok(())
    }

    async fn publish_message(
        &self,
        client: Arc<Client>,
        exchange: &str,
        _routing_key: &str,
        payload: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = client.get_multiplexed_async_connection().await?;

        let result: RedisResult<()> = conn.publish(exchange, payload).await;

        // Convert the RedisResult to a Result with the error type Box<dyn std::error::Error>
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
