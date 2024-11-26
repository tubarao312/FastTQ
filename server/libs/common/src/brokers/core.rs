use async_trait::async_trait;

#[async_trait]
pub trait BrokerCore {
    async fn register_queue(&self, queue_name: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn publish_message(
        &self,
        exchange: &str,
        routing_key: &str,
        payload: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>>;
}
