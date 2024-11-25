use std::time::SystemTime;

use async_trait::async_trait;
use sqlx::PgPool;
use tracing::info;
use uuid::Uuid;

use crate::repo::{PgRepositoryCore, TaskRepository};
use common::{
    models::{Task, TaskResult},
    TaskStatus, TaskType,
};
pub struct PgTaskRepository {
    core: PgRepositoryCore,
}

impl PgTaskRepository {
    pub fn new(core: PgRepositoryCore) -> Self {
        Self { core }
    }
}

#[async_trait]
impl TaskRepository for PgTaskRepository {
    async fn create_task(
        &self,
        task_type_id: Uuid,
        input_data: Option<serde_json::Value>,
    ) -> Result<Task, sqlx::Error> {
        let task_id = Uuid::new_v4();
        let row = sqlx::query!(
            r#"
            INSERT INTO tasks (id, task_type_id, input_data, status, assigned_to)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, task_type_id, input_data, status, assigned_to, created_at
            "#,
            task_id,
            task_type_id,
            input_data,
            "pending",
            None::<Uuid>,
        )
        .fetch_one(&self.core.pool)
        .await?;

        let task_type_row = sqlx::query!(
            r#"
            SELECT id, name FROM task_types WHERE id = $1
            "#,
            row.task_type_id
        )
        .fetch_one(&self.core.pool)
        .await?;

        let task = Task {
            id: row.id,
            task_type: TaskType {
                id: row.task_type_id,
                name: task_type_row.name,
            },
            input_data: row.input_data,
            status: row.status.into(),
            assigned_to: row.assigned_to,
            created_at: row.created_at.into(),
        };

        info!("Created task: {:?}", task);

        Ok(task)
    }

    async fn get_task_by_id(&self, id: &Uuid) -> Result<Task, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT id, task_type_id, input_data, status::text, assigned_to, created_at 
            FROM tasks 
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.core.pool)
        .await?;

        let task_type_row = sqlx::query!(
            r#"
            SELECT id, name FROM task_types WHERE id = $1
            "#,
            row.task_type_id
        )
        .fetch_one(&self.core.pool)
        .await?;

        Ok(Task {
            id: row.id,
            task_type: TaskType {
                id: task_type_row.id,
                name: task_type_row.name,
            },
            input_data: row.input_data,
            status: row.status.into(),
            assigned_to: row.assigned_to,
            created_at: row.created_at.into(),
        })
    }

    async fn get_task_results_by_task_id(
        &self,
        task_id: &Uuid,
    ) -> Result<Vec<TaskResult>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
            SELECT id, task_id, worker_id, output_data, created_at, error_data
            FROM task_results
            WHERE task_id = $1
            "#,
            task_id
        )
        .fetch_all(&self.core.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| TaskResult {
                id: row.id,
                task_id: row.task_id,
                worker_id: row.worker_id,
                output_data: row.output_data,
                created_at: row.created_at.into(),
                error_data: row.error_data,
            })
            .collect())
    }

    async fn update_task_status(
        &self,
        task_id: &Uuid,
        status: TaskStatus,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE tasks 
            SET status = $1
            WHERE id = $2
            "#,
            String::from(status),
            task_id
        )
        .execute(&self.core.pool)
        .await?;

        Ok(())
    }

    async fn upload_task_error(
        &self,
        task_id: &Uuid,
        worker_id: &Uuid,
        error: serde_json::Value,
    ) -> Result<TaskResult, sqlx::Error> {
        let now = SystemTime::now();
        let mut txn = self.core.pool.begin().await?;

        sqlx::query!(
            r#"
            UPDATE tasks SET status = $1 WHERE id = $2
            "#,
            String::from(TaskStatus::Failed),
            task_id
        )
        .execute(&mut *txn)
        .await?;

        let result = sqlx::query!(
            r#"
            INSERT INTO task_results (
                id, task_id, worker_id, error_data
            )
            VALUES ($1, $2, $3, $4)
            RETURNING id, task_id, worker_id, output_data, error_data, created_at
            "#,
            Uuid::new_v4(),
            task_id,
            worker_id,
            error
        )
        .fetch_one(&mut *txn)
        .await?;

        txn.commit().await?;

        Ok(TaskResult {
            id: result.id,
            task_id: result.task_id,
            worker_id: result.worker_id,
            output_data: result.output_data,
            error_data: result.error_data,
            created_at: result.created_at.into(),
        })
    }

    async fn upload_task_result(
        &self,
        task_id: &Uuid,
        worker_id: &Uuid,
        output: serde_json::Value,
    ) -> Result<TaskResult, sqlx::Error> {
        let mut txn = self.core.pool.begin().await?;

        sqlx::query!(
            r#"
            UPDATE tasks SET status = $1 WHERE id = $2
            "#,
            String::from(TaskStatus::Completed),
            task_id
        )
        .execute(&mut *txn)
        .await?;

        let result = sqlx::query!(
            r#"
            INSERT INTO task_results (
                id, task_id, worker_id, output_data
            )
            VALUES ($1, $2, $3, $4)
            RETURNING id, task_id, worker_id, output_data, error_data, created_at
            "#,
            Uuid::new_v4(),
            task_id,
            worker_id,
            output
        )
        .fetch_one(&mut *txn)
        .await?;

        txn.commit().await?;

        Ok(TaskResult {
            id: result.id,
            task_id: result.task_id,
            worker_id: result.worker_id,
            output_data: result.output_data,
            error_data: result.error_data,
            created_at: result.created_at.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::repo::{
        PgRepositoryCore, PgTaskTypeRepository, PgWorkerRepository, TaskTypeRepository,
        WorkerRepository,
    };

    use super::*;
    use common::TaskStatus;
    use sqlx::PgPool;
    use std::time::SystemTime;
    use uuid::Uuid;

    /// Creates a task and then retrieves it by id
    #[sqlx::test(migrator = "db_common::MIGRATOR")]
    async fn create_and_get_task(pool: PgPool) {
        let repo = PgTaskRepository::new(PgRepositoryCore::new(pool.clone()));
        let task_type_repo = PgTaskTypeRepository::new(PgRepositoryCore::new(pool));

        let task_type = TaskType {
            id: Uuid::new_v4(),
            name: "Test Task".to_string(),
        };
        task_type_repo.put_task_type(&task_type).await.unwrap();

        let input = serde_json::json!({"test": "data"});
        let task = repo
            .create_task(task_type.id, Some(input.clone()))
            .await
            .unwrap();

        assert_eq!(task.task_type.id, task_type.id);
        assert_eq!(task.task_type.name, task_type.name);
        assert_eq!(task.input_data, Some(input));
        assert_eq!(task.status, TaskStatus::Pending);
        assert_eq!(task.assigned_to, None);

        let retrieved = repo.get_task_by_id(&task.id).await.unwrap();
        assert_eq!(task.id, retrieved.id);
    }

    /// Creates a task and then uploads a result and an error
    #[sqlx::test(migrator = "db_common::MIGRATOR")]
    async fn create_task_and_then_upload_error(pool: PgPool) {
        let core = PgRepositoryCore::new(pool.clone());
        let repo = PgTaskRepository::new(core.clone());
        let task_type_repo = PgTaskTypeRepository::new(core.clone());
        let worker_repo = PgWorkerRepository::new(core);

        let task_type = TaskType {
            id: Uuid::new_v4(),
            name: "Test Task".to_string(),
        };
        task_type_repo.put_task_type(&task_type).await.unwrap();

        let task = repo.create_task(task_type.id, None).await.unwrap();
        let worker_id = Uuid::new_v4();
        worker_repo
            .register_worker(
                worker_id,
                "Test Worker".to_string(),
                vec![task_type.clone()],
            )
            .await
            .unwrap();

        // Test successful result
        let output = serde_json::json!({"result": "success"});
        let result = repo
            .upload_task_result(&task.id, &worker_id, output.clone())
            .await
            .unwrap();

        assert_eq!(result.task_id, task.id);
        assert_eq!(result.worker_id, worker_id);
        assert_eq!(result.output_data, Some(output));
        assert!(result.error_data.is_none());

        // Test error result
        let task2 = repo.create_task(task_type.id, None).await.unwrap();
        let error = serde_json::json!({"error": "failed"});
        let error_result = repo
            .upload_task_error(&task2.id, &worker_id, error.clone())
            .await
            .unwrap();

        assert_eq!(error_result.task_id, task2.id);
        assert_eq!(error_result.worker_id, worker_id);
        assert_eq!(error_result.error_data, Some(error));
        assert!(error_result.output_data.is_none());

        // Test getting results
        let results = repo.get_task_results_by_task_id(&task.id).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, result.id);
    }

    /// Tests that a task's status can be updated after creation
    #[sqlx::test(migrator = "db_common::MIGRATOR")]
    async fn test_task_status_update(pool: PgPool) {
        let repo = PgTaskRepository::new(PgRepositoryCore::new(pool.clone()));
        let task_type_repo = PgTaskTypeRepository::new(PgRepositoryCore::new(pool));

        let task_type = TaskType {
            id: Uuid::new_v4(),
            name: "Test Task".to_string(),
        };
        task_type_repo.put_task_type(&task_type).await.unwrap();

        let task = repo.create_task(task_type.id, None).await.unwrap();
        assert_eq!(task.status, TaskStatus::Pending);

        repo.update_task_status(&task.id, TaskStatus::Running)
            .await
            .unwrap();
        let updated = repo.get_task_by_id(&task.id).await.unwrap();
        assert_eq!(updated.status, TaskStatus::Running);
    }

    /// Creates a task without input data (should be allowed)
    #[sqlx::test(migrator = "db_common::MIGRATOR")]
    async fn create_task_without_input_data(pool: PgPool) {
        let repo = PgTaskRepository::new(PgRepositoryCore::new(pool.clone()));
        let task_type_repo = PgTaskTypeRepository::new(PgRepositoryCore::new(pool));

        let task_type = TaskType {
            id: Uuid::new_v4(),
            name: "Test Task".to_string(),
        };
        task_type_repo.put_task_type(&task_type).await.unwrap();

        let task = repo.create_task(task_type.id, None).await.unwrap();
        assert_eq!(task.input_data, None);
    }

    /// Creates a task and then retrieves its results, which should be empty (no results yet)
    #[sqlx::test(migrator = "db_common::MIGRATOR")]
    async fn get_task_results_empty(pool: PgPool) {
        let repo = PgTaskRepository::new(PgRepositoryCore::new(pool.clone()));
        let task_type_repo = PgTaskTypeRepository::new(PgRepositoryCore::new(pool));

        let task_type = TaskType {
            id: Uuid::new_v4(),
            name: "Test Task".to_string(),
        };
        task_type_repo.put_task_type(&task_type).await.unwrap();

        let task = repo.create_task(task_type.id, None).await.unwrap();
        let results = repo.get_task_results_by_task_id(&task.id).await.unwrap();
        assert!(results.is_empty());
    }

    /// Attempts to retrieve a non-existent task (should fail)
    #[sqlx::test(migrator = "db_common::MIGRATOR")]
    async fn get_nonexistent_task(pool: PgPool) {
        let repo = PgTaskRepository::new(PgRepositoryCore::new(pool));
        let result = repo.get_task_by_id(&Uuid::new_v4()).await;
        assert!(result.is_err());
    }

    /// Creates a task and then updates its status through all possible transitions
    #[sqlx::test(migrator = "db_common::MIGRATOR")]
    async fn status_transitions(pool: PgPool) {
        let repo = PgTaskRepository::new(PgRepositoryCore::new(pool.clone()));
        let task_type_repo = PgTaskTypeRepository::new(PgRepositoryCore::new(pool));

        let task_type = TaskType {
            id: Uuid::new_v4(),
            name: "Test Task".to_string(),
        };
        task_type_repo.put_task_type(&task_type).await.unwrap();

        let task = repo.create_task(task_type.id, None).await.unwrap();

        // Test full lifecycle
        assert_eq!(task.status, TaskStatus::Pending);

        repo.update_task_status(&task.id, TaskStatus::Running)
            .await
            .unwrap();
        let task = repo.get_task_by_id(&task.id).await.unwrap();
        assert_eq!(task.status, TaskStatus::Running);

        repo.update_task_status(&task.id, TaskStatus::Completed)
            .await
            .unwrap();
        let task = repo.get_task_by_id(&task.id).await.unwrap();
        assert_eq!(task.status, TaskStatus::Completed);
    }
}
