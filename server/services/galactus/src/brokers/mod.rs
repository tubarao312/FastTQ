pub mod base;
pub mod rabbit;
pub mod redis;

use base::BaseBroker;
use rabbit::RabbitBroker;
use redis::RedisBroker;

pub async fn create_broker_connection(
    uri: String,
) -> Result<Box<dyn BaseBroker>, Box<dyn std::error::Error>> {
    let prefix = uri.split(":").collect::<Vec<&str>>()[0];

    match prefix {
        "redis" => Ok(Box::new(RedisBroker::new(&uri).await?)),
        "amqp" => Ok(Box::new(RabbitBroker::new(&uri).await?)),
        _ => Err("Invalid broker URI".into()),
    }
}
