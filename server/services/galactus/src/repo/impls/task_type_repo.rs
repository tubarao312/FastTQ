use async_trait::async_trait;
use common::models::TaskType;
use sqlx::PgPool;
use uuid::Uuid;

use crate::repo::{PgRepositoryCore, TaskTypeRepository};

#[derive(Clone)]

pub struct PgTaskTypeRepository {
    core: PgRepositoryCore,
}

impl PgTaskTypeRepository {
    pub fn new(core: PgRepositoryCore) -> Self {
        Self { core }
    }
}

#[async_trait]
impl TaskTypeRepository for PgTaskTypeRepository {
    async fn put_task_type(&self, task_type: &TaskType) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO task_types (id, name)
            VALUES ($1, $2)
            ON CONFLICT (id) DO UPDATE SET name = $2
            "#,
            task_type.id,
            task_type.name,
        )
        .execute(&self.core.pool)
        .await?;

        Ok(())
    }

    async fn get_all_task_types(&self) -> Result<Vec<TaskType>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name FROM task_types
            "#
        )
        .fetch_all(&self.core.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| TaskType {
                id: row.id,
                name: row.name,
            })
            .collect())
    }

    async fn get_task_type_by_id(&self, id: &Uuid) -> Result<TaskType, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT id, name FROM task_types WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.core.pool)
        .await?;

        Ok(TaskType {
            id: row.id,
            name: row.name,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    /// Creates a task type and then retrieves it by id
    #[sqlx::test(migrator = "db_common::MIGRATOR")]
    async fn create_and_retrieve_task_type(pool: PgPool) {
        let repo = PgTaskTypeRepository::new(PgRepositoryCore::new(pool));

        let task_type = TaskType {
            id: Uuid::new_v4(),
            name: "Test Task".to_string(),
        };

        repo.put_task_type(&task_type).await.unwrap();
        let retrieved = repo.get_task_type_by_id(&task_type.id).await.unwrap();

        assert_eq!(task_type.id, retrieved.id);
        assert_eq!(task_type.name, retrieved.name);
    }

    /// Creates two task types and then retrieves all task types
    #[sqlx::test(migrator = "db_common::MIGRATOR")]
    async fn get_all_task_types(pool: PgPool) {
        let repo = PgTaskTypeRepository::new(PgRepositoryCore::new(pool));

        let task_type1 = TaskType {
            id: Uuid::new_v4(),
            name: "Test Task 1".to_string(),
        };

        let task_type2 = TaskType {
            id: Uuid::new_v4(),
            name: "Test Task 2".to_string(),
        };

        repo.put_task_type(&task_type1).await.unwrap();
        repo.put_task_type(&task_type2).await.unwrap();

        let all_types = repo.get_all_task_types().await.unwrap();

        assert_eq!(all_types.len(), 2);
        assert!(all_types.iter().any(|t| t.id == task_type1.id));
        assert!(all_types.iter().any(|t| t.id == task_type2.id));
    }

    /// Creates a task type and then updates its name
    #[sqlx::test(migrator = "db_common::MIGRATOR")]
    async fn update_task_type(pool: PgPool) {
        let repo = PgTaskTypeRepository::new(PgRepositoryCore::new(pool));

        let id = Uuid::new_v4();
        let task_type = TaskType {
            id,
            name: "Original Name".to_string(),
        };

        repo.put_task_type(&task_type).await.unwrap();

        let updated = TaskType {
            id,
            name: "Updated Name".to_string(),
        };

        repo.put_task_type(&updated).await.unwrap();
        let retrieved = repo.get_task_type_by_id(&id).await.unwrap();

        assert_eq!(retrieved.name, "Updated Name");
    }
}
