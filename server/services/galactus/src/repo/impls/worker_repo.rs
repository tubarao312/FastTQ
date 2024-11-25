use async_trait::async_trait;
use common::models::{TaskType, Worker};
use sqlx::PgPool;
use std::time::SystemTime;
use uuid::Uuid;

use crate::repo::{PgRepositoryCore, WorkerRepository};

#[derive(Clone)]
pub struct PgWorkerRepository {
    core: PgRepositoryCore,
}

impl PgWorkerRepository {
    pub fn new(core: PgRepositoryCore) -> Self {
        Self { core }
    }
}

#[async_trait]
impl WorkerRepository for PgWorkerRepository {
    async fn register_worker(
        &self,
        id: Uuid,
        name: String,
        task_types: Vec<TaskType>,
    ) -> Result<Worker, sqlx::Error> {
        let mut txn = self.core.pool.begin().await?;

        sqlx::query!(
            r#"
            INSERT INTO workers (id, name, registered_at)
            VALUES ($1, $2, NOW())
            ON CONFLICT (id) DO UPDATE SET name = $2
            "#,
            id,
            name,
        )
        .execute(&mut *txn)
        .await?;

        // Clear existing task types
        sqlx::query!(
            r#"
            DELETE FROM worker_task_types WHERE worker_id = $1
            "#,
            id
        )
        .execute(&mut *txn)
        .await?;

        // Insert new task types
        for task_type in &task_types {
            sqlx::query!(
                r#"
                INSERT INTO worker_task_types (worker_id, task_type_id)
                VALUES ($1, $2)
                "#,
                id,
                task_type.id
            )
            .execute(&mut *txn)
            .await?;
        }

        txn.commit().await?;

        let row = sqlx::query!(
            r#"
            SELECT registered_at, active FROM workers WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.core.pool)
        .await?;

        Ok(Worker {
            id,
            name,
            registered_at: row.registered_at.into(),
            task_types,
            active: row.active,
        })
    }

    async fn get_worker_by_id(&self, id: &Uuid) -> Result<Worker, sqlx::Error> {
        let worker = sqlx::query!(
            r#"
            SELECT name, registered_at, active FROM workers WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.core.pool)
        .await?;

        let task_types = sqlx::query!(
            r#"
            SELECT tt.id, tt.name 
            FROM task_types tt
            JOIN worker_task_types wtt ON wtt.task_type_id = tt.id
            WHERE wtt.worker_id = $1
            "#,
            id
        )
        .fetch_all(&self.core.pool)
        .await?;

        Ok(Worker {
            id: *id,
            name: worker.name,
            registered_at: worker.registered_at.into(),
            task_types: task_types
                .into_iter()
                .map(|tt| TaskType {
                    id: tt.id,
                    name: tt.name,
                })
                .collect(),
            active: worker.active,
        })
    }

    async fn get_all_workers(&self) -> Result<Vec<Worker>, sqlx::Error> {
        let workers = sqlx::query!(
            r#"
            SELECT id, name, registered_at, active FROM workers
            "#
        )
        .fetch_all(&self.core.pool)
        .await?;

        let mut result = Vec::new();

        for worker in workers {
            let task_types = sqlx::query!(
                r#"
                SELECT tt.id, tt.name 
                FROM task_types tt
                JOIN worker_task_types wtt ON wtt.task_type_id = tt.id
                WHERE wtt.worker_id = $1
                "#,
                worker.id
            )
            .fetch_all(&self.core.pool)
            .await?;

            result.push(Worker {
                id: worker.id,
                name: worker.name,
                registered_at: worker.registered_at.into(),
                task_types: task_types
                    .into_iter()
                    .map(|tt| TaskType {
                        id: tt.id,
                        name: tt.name,
                    })
                    .collect(),
                active: worker.active,
            });
        }

        Ok(result)
    }

    async fn set_worker_active(&self, worker_id: &Uuid, active: bool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE workers SET active = $1 WHERE id = $2
            "#,
            active,
            worker_id
        )
        .execute(&self.core.pool)
        .await?;

        Ok(())
    }

    async fn record_heartbeat(&self, worker_id: &Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO worker_heartbeats (worker_id, heartbeat_time)
            VALUES ($1, NOW())
            "#,
            worker_id
        )
        .execute(&self.core.pool)
        .await?;

        Ok(())
    }

    async fn get_latest_heartbeat(&self, worker_id: &Uuid) -> Result<SystemTime, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT heartbeat_time 
            FROM worker_heartbeats 
            WHERE worker_id = $1 
            ORDER BY heartbeat_time DESC 
            LIMIT 1
            "#,
            worker_id
        )
        .fetch_one(&self.core.pool)
        .await?;

        Ok(row.heartbeat_time.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo::{PgRepositoryCore, PgTaskTypeRepository, TaskTypeRepository};

    /// Registers a worker and then retrieves it by id
    #[sqlx::test(migrator = "db_common::MIGRATOR")]
    async fn register_and_get_worker(pool: PgPool) {
        let repo = PgWorkerRepository::new(PgRepositoryCore::new(pool.clone()));
        let task_type_repo = PgTaskTypeRepository::new(PgRepositoryCore::new(pool));

        let task_type = TaskType {
            id: Uuid::new_v4(),
            name: "Test Task".to_string(),
        };
        task_type_repo.put_task_type(&task_type).await.unwrap();

        let worker_id = Uuid::new_v4();
        let worker = repo
            .register_worker(
                worker_id,
                "Test Worker".to_string(),
                vec![task_type.clone()],
            )
            .await
            .unwrap();

        assert_eq!(worker.id, worker_id);
        assert_eq!(worker.name, "Test Worker");
        assert_eq!(worker.task_types.len(), 1);
        assert_eq!(worker.task_types[0].id, task_type.id);
        assert!(worker.active);

        let retrieved = repo.get_worker_by_id(&worker_id).await.unwrap();
        assert_eq!(worker.id, retrieved.id);
        assert_eq!(worker.name, retrieved.name);
        assert_eq!(worker.task_types, retrieved.task_types);
    }

    /// Registers two workers and then retrieves all workers
    #[sqlx::test(migrator = "db_common::MIGRATOR")]
    async fn get_all_workers(pool: PgPool) {
        let repo = PgWorkerRepository::new(PgRepositoryCore::new(pool.clone()));
        let task_type_repo = PgTaskTypeRepository::new(PgRepositoryCore::new(pool));

        let task_type = TaskType {
            id: Uuid::new_v4(),
            name: "Test Task".to_string(),
        };
        task_type_repo.put_task_type(&task_type).await.unwrap();

        let worker1 = repo
            .register_worker(
                Uuid::new_v4(),
                "Worker 1".to_string(),
                vec![task_type.clone()],
            )
            .await
            .unwrap();

        let worker2 = repo
            .register_worker(
                Uuid::new_v4(),
                "Worker 2".to_string(),
                vec![task_type.clone()],
            )
            .await
            .unwrap();

        let all_workers = repo.get_all_workers().await.unwrap();
        assert_eq!(all_workers.len(), 2);
        assert!(all_workers.iter().any(|w| w.id == worker1.id));
        assert!(all_workers.iter().any(|w| w.id == worker2.id));
    }

    /// Registers a worker and then updates its name and task types
    #[sqlx::test(migrator = "db_common::MIGRATOR")]
    async fn update_worker(pool: PgPool) {
        let repo = PgWorkerRepository::new(PgRepositoryCore::new(pool.clone()));
        let task_type_repo = PgTaskTypeRepository::new(PgRepositoryCore::new(pool));

        let task_type1 = TaskType {
            id: Uuid::new_v4(),
            name: "Task 1".to_string(),
        };
        let task_type2 = TaskType {
            id: Uuid::new_v4(),
            name: "Task 2".to_string(),
        };
        task_type_repo.put_task_type(&task_type1).await.unwrap();
        task_type_repo.put_task_type(&task_type2).await.unwrap();

        let worker_id = Uuid::new_v4();
        let worker = repo
            .register_worker(worker_id, "Original Name".to_string(), vec![task_type1])
            .await
            .unwrap();

        // Update name and task types
        let updated = repo
            .register_worker(
                worker_id,
                "Updated Name".to_string(),
                vec![task_type2.clone()],
            )
            .await
            .unwrap();

        assert_eq!(updated.id, worker_id);
        assert_eq!(updated.name, "Updated Name");
        assert_eq!(updated.task_types.len(), 1);
        assert_eq!(updated.task_types[0].id, task_type2.id);
    }

    /// Registers a worker and then updates its active status
    #[sqlx::test(migrator = "db_common::MIGRATOR")]
    async fn worker_active_status(pool: PgPool) {
        let repo = PgWorkerRepository::new(PgRepositoryCore::new(pool.clone()));
        let task_type_repo = PgTaskTypeRepository::new(PgRepositoryCore::new(pool));

        let task_type = TaskType {
            id: Uuid::new_v4(),
            name: "Test Task".to_string(),
        };
        task_type_repo.put_task_type(&task_type).await.unwrap();

        let worker = repo
            .register_worker(Uuid::new_v4(), "Test Worker".to_string(), vec![task_type])
            .await
            .unwrap();
        assert!(worker.active);

        repo.set_worker_active(&worker.id, false).await.unwrap();
        let updated = repo.get_worker_by_id(&worker.id).await.unwrap();
        assert!(!updated.active);
    }

    /// Registers a worker and then records a heartbeat
    #[sqlx::test(migrator = "db_common::MIGRATOR")]
    async fn worker_heartbeat(pool: PgPool) {
        let repo = PgWorkerRepository::new(PgRepositoryCore::new(pool.clone()));
        let task_type_repo = PgTaskTypeRepository::new(PgRepositoryCore::new(pool));

        let task_type = TaskType {
            id: Uuid::new_v4(),
            name: "Test Task".to_string(),
        };
        task_type_repo.put_task_type(&task_type).await.unwrap();

        let worker = repo
            .register_worker(Uuid::new_v4(), "Test Worker".to_string(), vec![task_type])
            .await
            .unwrap();

        repo.record_heartbeat(&worker.id).await.unwrap();
        let heartbeat = repo.get_latest_heartbeat(&worker.id).await.unwrap();

        // Heartbeat should be recent
        let now = SystemTime::now();
        assert!(now.duration_since(heartbeat).unwrap().as_secs() < 1);
    }

    /// Attempts to retrieve a nonexistent worker by id (should fail)
    #[sqlx::test(migrator = "db_common::MIGRATOR")]
    async fn get_nonexistent_worker(pool: PgPool) {
        let repo = PgWorkerRepository::new(PgRepositoryCore::new(pool));
        let result = repo.get_worker_by_id(&Uuid::new_v4()).await;
        assert!(result.is_err());
    }

    /// Attempts to retrieve a nonexistent heartbeat (should fail)
    #[sqlx::test(migrator = "db_common::MIGRATOR")]
    async fn get_nonexistent_heartbeat(pool: PgPool) {
        let repo = PgWorkerRepository::new(PgRepositoryCore::new(pool));
        let result = repo.get_latest_heartbeat(&Uuid::new_v4()).await;
        assert!(result.is_err());
    }
}
