pub mod core;
pub mod rabbit;
pub mod redis;

use core::BrokerCore;
use rabbit::RabbitBroker;
use redis::RedisBroker;
use uuid::Uuid;

use std::sync::Arc;

use crate::{TaskInstance, Worker};

async fn create_broker_connection(
    uri: &str,
) -> Result<Arc<dyn BrokerCore>, Box<dyn std::error::Error>> {
    let prefix = uri.split(":").collect::<Vec<&str>>()[0];

    match prefix {
        "redis" => Ok(Arc::new(RedisBroker::new(uri).await?)),
        "amqp" => Ok(Arc::new(RabbitBroker::new(uri).await?)),
        _ => Err("Invalid broker URI".into()),
    }
}

#[derive(Clone)]
pub struct Broker {
    pub uri: String,
    pub broker: Arc<dyn BrokerCore>,
    pub workers: Vec<Worker>,
    pub workers_index: usize,
}

impl Broker {
    pub async fn new(uri: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let broker = create_broker_connection(uri).await?;
        Ok(Broker {
            uri: uri.to_string(),
            broker,
            workers: Vec::new(),
            workers_index: 0,
        })
    }

    pub async fn register_worker(
        &mut self,
        worker: Worker,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for task_kind in worker.task_kind.clone() {
            self.broker
                .register_queue(&task_kind.name, &task_kind.name, &worker.name)
                .await?;
        }

        self.workers.push(worker);
        Ok(())
    }

    pub fn remove_worker(&mut self, worker_id: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
        let index: usize = self
            .workers
            .iter()
            .position(|worker| worker.id == *worker_id)
            .unwrap();
        self.workers.remove(index);

        Ok(())
    }

    pub async fn publish(
        &mut self,
        task: &TaskInstance,
    ) -> Result<Uuid, Box<dyn std::error::Error>> {
        let worker = (0..self.workers.len())
            // Cycle the workers list in a round robin fashion
            .map(|_| {
                let cur_worker = &self.workers[self.workers_index];
                self.workers_index = (self.workers_index + 1) % self.workers.len();
                cur_worker
            })
            // Find the first worker that can handle the task
            .find(|cur_worker| cur_worker.can_handle(task))
            .ok_or("No available worker")?;

        // Convert input data to bytes
        let payload = serde_json::to_vec(&task.input_data)?;

        // Publish the task to the worker
        self.broker
            .publish_message(&task.task_kind.name, &worker.name, &payload)
            .await?;

        Ok(worker.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::TaskKind;
    use crate::TaskStatus;
    use async_trait::async_trait;
    use time::OffsetDateTime;
    use uuid::Uuid;

    // Mock implementations for BaseBroker, RedisBroker, and RabbitBroker
    #[derive(Clone)]
    struct MockBroker;
    #[async_trait]
    impl BrokerCore for MockBroker {
        async fn register_queue(
            &self,
            _: &str,
            _: &str,
            _: &str,
        ) -> Result<(), Box<dyn std::error::Error>> {
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

    fn setup_task_kinds() -> Vec<TaskKind> {
        vec![
            TaskKind::new("task1".to_string()),
            TaskKind::new("task2".to_string()),
        ]
    }

    fn setup_workers(task_kinds: Vec<TaskKind>) -> Vec<Worker> {
        vec![
            Worker {
                id: Uuid::new_v4(),
                name: "worker1".to_string(),
                registered_at: OffsetDateTime::now_utc(),
                task_kind: vec![task_kinds[0].clone()],
                active: true,
            },
            Worker {
                id: Uuid::new_v4(),
                name: "worker2".to_string(),
                registered_at: OffsetDateTime::now_utc(),
                task_kind: vec![task_kinds[1].clone()],
                active: true,
            },
            Worker {
                id: Uuid::new_v4(),
                name: "worker3".to_string(),
                registered_at: OffsetDateTime::now_utc(),
                task_kind: task_kinds,
                active: true,
            },
        ]
    }

    fn setup_tasks(task_kinds: Vec<TaskKind>) -> Vec<TaskInstance> {
        vec![
            TaskInstance {
                id: Uuid::new_v4(),
                task_kind: task_kinds[0].clone(),
                input_data: Some(serde_json::json!({"key": "value"})),
                status: TaskStatus::Pending,
                created_at: OffsetDateTime::now_utc(),
                assigned_to: None,
                result: None,
            },
            TaskInstance {
                id: Uuid::new_v4(),
                task_kind: task_kinds[1].clone(),
                input_data: Some(serde_json::json!({"key": "value"})),
                status: TaskStatus::Pending,
                created_at: OffsetDateTime::now_utc(),
                assigned_to: None,
                result: None,
            },
            TaskInstance {
                id: Uuid::new_v4(),
                task_kind: task_kinds[1].clone(),
                input_data: Some(serde_json::json!({"key": "value"})),
                status: TaskStatus::Pending,
                created_at: OffsetDateTime::now_utc(),
                assigned_to: None,
                result: None,
            },
        ]
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
        assert_eq!(broker.workers_index, 0);
    }

    #[tokio::test]
    async fn test_broker_register_worker() {
        let uri = "redis://localhost".to_string();
        let mut broker = Broker::new(&uri).await.unwrap();
        let workers = setup_workers(setup_task_kinds());

        for worker in workers {
            broker.register_worker(worker).await.unwrap();
        }

        assert_eq!(broker.workers.len(), 3);
    }

    #[tokio::test]
    async fn test_broker_remove_worker() {
        let uri = "redis://localhost".to_string();
        let mut broker = Broker::new(&uri).await.unwrap();
        let workers = setup_workers(setup_task_kinds());

        for worker in workers.clone() {
            broker.register_worker(worker).await.unwrap();
        }

        broker.remove_worker(&workers[0].id).unwrap();
        assert_eq!(broker.workers.len(), 2);
    }

    #[tokio::test]
    async fn test_broker_publish() {
        let uri = "redis://localhost".to_string();
        let task_kinds = setup_task_kinds();
        let workers = setup_workers(task_kinds.clone());
        let tasks = setup_tasks(task_kinds.clone());

        let mut broker = Broker::new(&uri).await.unwrap();
        broker.broker = Arc::new(MockBroker {});

        for worker in workers.clone() {
            broker.register_worker(worker).await.unwrap();
        }

        for task in tasks {
            broker.publish(&task).await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_no_available_worker() {
        let uri = "redis://localhost".to_string();
        let mut broker = Broker::new(&uri).await.unwrap();
        broker.broker = Arc::new(MockBroker {});

        let workers = setup_workers(setup_task_kinds());

        for worker in workers.clone() {
            broker.register_worker(worker).await.unwrap();
        }

        let task = TaskInstance {
            id: Uuid::new_v4(),
            task_kind: TaskKind::new("task3".to_string()),
            input_data: Some(serde_json::json!({"key": "value"})),
            status: TaskStatus::Pending,
            created_at: OffsetDateTime::now_utc(),
            assigned_to: None,
            result: None,
        };

        let result = broker.publish(&task).await;
        assert!(result.is_err());
    }
}
