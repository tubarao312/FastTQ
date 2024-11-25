pub mod base;
pub mod rabbit;
pub mod redis;

use base::BaseBroker;
use rabbit::RabbitBroker;
use redis::RedisBroker;

use std::collections::HashMap;

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
    pub broker: Box<dyn BaseBroker>,
    pub workers: HashMap<String, Vec<String>>,
    pub wokers_index: HashMap<String, usize>,
}

impl Broker {
    pub async fn new(uri: &String) -> Result<Self, Box<dyn std::error::Error>> {
        let broker = create_broker_connection(uri).await?;
        Ok(Broker {
            uri: uri.clone(),
            broker,
            workers: HashMap::new(),
            wokers_index: HashMap::new(),
        })
    }

    pub fn register_worker(
        &mut self,
        task_name: &str,
        worker_name: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // If worker is not already registered for the task, add it and set the index to 0
        if !self.workers.contains_key(task_name) {
            self.workers
                .insert(task_name.to_string(), vec![worker_name]);
            self.wokers_index.insert(task_name.to_string(), 0);
        // If worker is already registered for the task, add it to the list
        } else {
            let workers = self.workers.get_mut(task_name).unwrap();
            workers.push(worker_name);
        }

        Ok(())
    }

    pub fn remove_worker(
        &mut self,
        task_name: &str,
        worker_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let workers = self.workers.get_mut(task_name).unwrap();
        let index = workers.iter().position(|x| x == worker_name).unwrap();
        workers.remove(index);

        Ok(())
    }

    pub async fn publish(
        &mut self,
        task_name: &str,
        message: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let workers = self.workers.get_mut(task_name).unwrap();
        let index = self.wokers_index.get_mut(task_name).unwrap();
        let size = workers.len();

        self.broker
            .publish_message(task_name, &workers[*index], message.as_bytes())
            .await?;

        // Increment the index to the next worker
        *index = (*index + 1) % size;

        Ok(())
    }
}

impl Clone for Broker {
    fn clone(&self) -> Self {
        Broker {
            uri: self.uri.clone(),
            broker: self.broker.clone(),
            workers: self.workers.clone(),
            wokers_index: self.wokers_index.clone(),
        }
    }
}
// Needs these implementations to be able to be used on the AppState struct
unsafe impl Send for Broker {}
unsafe impl Sync for Broker {}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    // Mock implementations for BaseBroker, RedisBroker, and RabbitBroker
    #[derive(Clone)]
    struct MockBroker;
    #[async_trait]
    impl BaseBroker for MockBroker {
        async fn register_queue(&self, _: &str) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }
        async fn publish_message(
            &self,
            _task_name: &str,
            _worker: &str,
            _message: &[u8],
        ) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_create_broker_connection() {
        let uri = "redis://localhost".to_string();
        let broker = create_broker_connection(&uri).await;
        assert!(broker.is_ok());
    }

    #[tokio::test]
    async fn test_broker_new() {
        let uri = "redis://localhost".to_string();
        let broker = Broker::new(&uri).await;
        assert!(broker.is_ok());
        let broker = broker.unwrap();
        assert_eq!(broker.uri, uri);
        assert_eq!(broker.workers.len(), 0);
        assert_eq!(broker.wokers_index.len(), 0);
    }

    #[test]
    fn test_broker_clone() {
        let uri = "redis://localhost".to_string();
        let broker = Broker {
            uri: uri.clone(),
            broker: Box::new(MockBroker),
            workers: HashMap::new(),
            wokers_index: HashMap::new(),
        };
        let cloned_broker = broker.clone();
        assert_eq!(cloned_broker.uri, uri);
    }

    #[tokio::test]
    async fn test_broker_register_worker() {
        let uri = "redis://localhost".to_string();
        let mut broker = Broker::new(&uri).await.unwrap();
        broker
            .register_worker("task1", "worker1".to_string())
            .unwrap();
        broker
            .register_worker("task1", "worker2".to_string())
            .unwrap();
        broker
            .register_worker("task2", "worker1".to_string())
            .unwrap();
        assert_eq!(broker.workers.len(), 2);
        assert_eq!(broker.workers.get("task1").unwrap().len(), 2);
        assert_eq!(broker.workers.get("task2").unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_broker_remove_worker() {
        let uri = "redis://localhost".to_string();
        let mut broker = Broker::new(&uri).await.unwrap();
        broker
            .register_worker("task1", "worker1".to_string())
            .unwrap();
        broker
            .register_worker("task1", "worker2".to_string())
            .unwrap();
        broker
            .register_worker("task2", "worker1".to_string())
            .unwrap();
        broker.remove_worker("task1", "worker1").unwrap();
        assert_eq!(broker.workers.len(), 2);
        assert_eq!(broker.workers.get("task1").unwrap().len(), 1);
        assert_eq!(broker.workers.get("task2").unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_broker_publish() {
        let uri = "redis://localhost".to_string();
        let mut broker = Broker::new(&uri).await.unwrap();
        broker.broker = Box::new(MockBroker {});

        broker
            .register_worker("task1", "worker1".to_string())
            .unwrap();
        broker
            .register_worker("task1", "worker2".to_string())
            .unwrap();
        broker
            .register_worker("task2", "worker1".to_string())
            .unwrap();

        // Confirm that the message is published to the first worker
        broker.publish("task1", "message1").await.unwrap();
        assert_eq!(broker.wokers_index.get("task1").unwrap(), &1);

        // Confirm that the message is published to the second worker
        broker.publish("task1", "message2").await.unwrap();
        assert_eq!(broker.wokers_index.get("task1").unwrap(), &0);

        // Confirm that the message is published to the only worker
        broker.publish("task2", "message3").await.unwrap();
        assert_eq!(broker.wokers_index.get("task2").unwrap(), &0);
    }
}
