use async_trait::async_trait;

#[async_trait]
pub trait BrokerCore: Send + Sync {
    async fn register_queue(
        &self,
        queue_name: &str,
        exchange: &str,
        routing_key: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;

    async fn publish_message(
        &self,
        exchange: &str,
        routing_key: &str,
        payload: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>>;
}
