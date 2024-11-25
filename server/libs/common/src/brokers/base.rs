use async_trait::async_trait;

#[async_trait]
pub trait BaseBroker: BaseBrokerClone {
    async fn register_queue(&self, queue_name: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn publish_message(
        &self,
        exchange: &str,
        routing_key: &str,
        payload: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait BaseBrokerClone {
    fn clone_box(&self) -> Box<dyn BaseBroker>;
}

impl<T> BaseBrokerClone for T
where
    T: 'static + BaseBroker + Clone,
{
    fn clone_box(&self) -> Box<dyn BaseBroker> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn BaseBroker> {
    fn clone(&self) -> Box<dyn BaseBroker> {
        self.clone_box()
    }
}
