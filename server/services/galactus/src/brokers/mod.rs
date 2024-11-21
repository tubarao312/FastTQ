pub mod base;
pub mod rabbit;
pub mod redis;

use base::BaseBroker;
use rabbit::RabbitBroker;
use redis::RedisBroker;

pub async fn create_broker_connection(
    uri: String,
) -> Result<Box<dyn BaseBroker>, Box<dyn std::error::Error>> {
    if uri.starts_with("amqp") {
        let broker = RabbitBroker::new(&uri).await?;
        Ok(Box::new(broker))
    } else if uri.starts_with("redis") {
        let broker = RedisBroker::new(&uri).await?;
        Ok(Box::new(broker))
    } else {
        Err("Unsupported broker URI".into())
    }
}
