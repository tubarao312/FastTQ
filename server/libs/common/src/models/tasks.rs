use serde::{Deserialize, Serialize};
use strum_macros::Display;
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::TaskKind;

// Task status enum

/// # Possible Status:
/// * `Pending`: Task is created but not yet assigned
/// * `Queued`: Task has been assigned to a worker and sent to a queue
/// * `Running`: Worker has started processing
/// * `Completed`: Task completed successfully
/// * `Failed`: Task failed to complete
/// * `Cancelled`: Task was cancelled before completion
#[derive(Display, Debug, Serialize, Deserialize, PartialEq, ToSchema)]
pub enum TaskStatus {
    Pending,   // Task is created but not yet assigned
    Queued,    // Task has been assigned to a worker and sent to a queue
    Running,   // Worker has started processing
    Completed, // Task completed successfully
    Failed,    // Task failed to complete
    Cancelled, // Task was cancelled before completion
}

impl From<String> for TaskStatus {
    fn from(s: String) -> Self {
        s.to_lowercase()
            .as_str()
            .try_into()
            .unwrap_or_else(|_| panic!("Invalid task status: {}", s))
    }
}

impl TryFrom<&str> for TaskStatus {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "pending" => Ok(Self::Pending),
            "queued" => Ok(Self::Queued),
            "running" => Ok(Self::Running),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(format!("Invalid task status: {}", s)),
        }
    }
}

impl From<TaskStatus> for String {
    fn from(status: TaskStatus) -> Self {
        match status {
            TaskStatus::Pending => "pending",
            TaskStatus::Queued => "queued",
            TaskStatus::Running => "running",
            TaskStatus::Completed => "completed",
            TaskStatus::Failed => "failed",
            TaskStatus::Cancelled => "cancelled",
        }
        .to_string()
    }
}

// Task

/// Tasks are sent to workers to be executed with a specific payload.
/// Workers are eligble for receiving certain tasks depending on their
/// list of capabilities.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TaskInstance {
    pub id: Uuid,
    pub task_kind: TaskKind,
    pub input_data: Option<serde_json::Value>,
    pub status: TaskStatus,
    #[serde(
        serialize_with = "crate::models::serialize_datetime",
        deserialize_with = "crate::models::deserialize_datetime"
    )]
    pub created_at: OffsetDateTime,
    pub assigned_to: Option<Uuid>,
    pub result: Option<TaskResult>,
}

/// Task results contain the output or error data from a completed task
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TaskResult {
    pub task_id: Uuid,
    pub output_data: Option<serde_json::Value>,
    pub error_data: Option<serde_json::Value>,
    pub worker_id: Uuid,
    pub created_at: OffsetDateTime,
}
