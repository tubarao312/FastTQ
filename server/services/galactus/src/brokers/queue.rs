use std::sync::Arc;

pub trait QueueBroker<T> {
    async fn connect(&self) -> Result<Arc<T>, Box<dyn std::error::Error>>;
    async fn register_queue(
        &self,
        conn: Arc<T>,
        queue_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
    async fn publish_message(
        &self,
        conn: Arc<T>,
        exchange: &str,
        routing_key: &str,
        payload: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>>;
}
