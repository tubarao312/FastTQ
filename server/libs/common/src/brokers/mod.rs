pub mod base;
pub mod rabbit;
pub mod redis;

use base::BaseBroker;
use rabbit::RabbitBroker;
use redis::RedisBroker;

async fn create_broker_connection(
    uri: &String,
) -> Result<Box<dyn BaseBroker>, Box<dyn std::error::Error>> {
    let prefix = uri.split(":").collect::<Vec<&str>>()[0];

    match prefix {
        "redis" => Ok(Box::new(RedisBroker::new(&uri).await?)),
        "amqp" => Ok(Box::new(RabbitBroker::new(&uri).await?)),
        _ => Err("Invalid broker URI".into()),
    }
}

pub struct Broker {
    pub uri: String,
    pub brokers: Vec<Box<dyn BaseBroker>>,
}

impl Broker {
    pub async fn new(uri: &String) -> Result<Self, Box<dyn std::error::Error>> {
        let broker = create_broker_connection(uri).await?;
        Ok(Broker {
            uri: uri.clone(),
            brokers: vec![broker],
        })
    }
}

impl Clone for Broker {
    fn clone(&self) -> Self {
        Broker {
            uri: self.uri.clone(),
            brokers: self.brokers.clone(),
        }
    }
}
// Needs these implementations to be able to be used on the AppState struct
unsafe impl Send for Broker {}
unsafe impl Sync for Broker {}
