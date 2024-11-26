use std::time::SystemTime;

use async_trait::async_trait;
use common::{
    models::{TaskInstance, TaskKind, TaskResult, Worker},
    TaskStatus,
};
use uuid::Uuid;

/// Repository trait for managing task records in the database
///
/// Provides methods for creating new tasks and retrieving existing tasks by their ID.
/// Tasks represent units of work that can be assigned to and processed by workers.
#[async_trait]
pub trait TaskRepository: Clone {
    /// Create a new task in the database
    async fn create_task(
        &self,
        task_type_id: Uuid,
        input_data: Option<serde_json::Value>,
    ) -> Result<TaskInstance, sqlx::Error>;

    /// Get a task by its ID
    async fn get_task_by_id(&self, id: &Uuid) -> Result<TaskInstance, sqlx::Error>;

    /// Get task results by task ID
    async fn get_task_results_by_task_id(
        &self,
        task_id: &Uuid,
    ) -> Result<Vec<TaskResult>, sqlx::Error>;

    /// Update the status of a task
    async fn update_task_status(
        &self,
        task_id: &Uuid,
        status: TaskStatus,
    ) -> Result<(), sqlx::Error>;

    /// Upload an error result for a task, marking it as failed
    async fn upload_task_error(
        &self,
        task_id: &Uuid,
        worker_id: &Uuid,
        error: serde_json::Value,
    ) -> Result<TaskResult, sqlx::Error>;

    /// Upload a successful result for a task, marking it as completed
    async fn upload_task_result(
        &self,
        task_id: &Uuid,
        worker_id: &Uuid,
        output: serde_json::Value,
    ) -> Result<TaskResult, sqlx::Error>;
}

/// Repository trait for managing task type records in the database
///
/// Provides methods for registering and managing task types that workers can process.
/// Task types define the different kinds of work that can be performed in the system.
#[async_trait]
pub trait TaskTypeRepository: Clone {
    /// Register a new task type
    async fn put_task_type(&self, task_type: &TaskKind) -> Result<(), sqlx::Error>;

    /// Get all registered task types
    async fn get_all_task_types(&self) -> Result<Vec<TaskKind>, sqlx::Error>;

    /// Get a task type by its ID
    async fn get_task_type_by_id(&self, id: &Uuid) -> Result<TaskKind, sqlx::Error>;
}

/// Repository trait for managing worker records in the database
///
/// Provides methods for registering and managing workers that can process tasks.
#[async_trait]
pub trait WorkerRepository: Clone {
    /// Register a new worker with its supported task types
    async fn register_worker(
        &self,
        id: Uuid,
        name: String,
        task_types: Vec<TaskKind>,
    ) -> Result<Worker, sqlx::Error>;

    /// Get a worker by ID
    async fn get_worker_by_id(&self, id: &Uuid) -> Result<Worker, sqlx::Error>;

    /// Get all registered workers
    async fn get_all_workers(&self) -> Result<Vec<Worker>, sqlx::Error>;

    /// Update a worker's active status
    async fn set_worker_active(&self, worker_id: &Uuid, active: bool) -> Result<(), sqlx::Error>;

    /// Record a heartbeat for a worker
    async fn record_heartbeat(&self, worker_id: &Uuid) -> Result<(), sqlx::Error>;

    /// Get the latest heartbeat for a worker
    async fn get_latest_heartbeat(&self, worker_id: &Uuid) -> Result<SystemTime, sqlx::Error>;
}
