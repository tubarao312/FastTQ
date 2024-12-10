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
/// * `Accepted`: Worker acknowledged receipt
/// * `Paused`: Temporarily suspended
/// * `Retrying`: Failed but attempting again
/// * `Timeout`: Exceeded time limit
/// * `Rejected`: Worker refused task
/// * `Blocked`: Waiting on dependencies
#[derive(Display, Debug, Serialize, Deserialize, PartialEq, ToSchema)]
pub enum TaskStatus {
    Pending,   // Task is created but not yet assigned
    Accepted,  // Worker acknowledged receipt
    Queued,    // Task has been assigned to a worker and sent to a queue
    Running,   // Worker has started processing
    Paused,    // Temporarily suspended
    Retrying,  // Failed but attempting again
    Completed, // Task completed successfully
    Failed,    // Task failed to complete
    Cancelled, // Task was cancelled before completion
    Timeout,   // Exceeded time limit
    Rejected,  // Worker refused task
    Blocked,   // Waiting on dependencies
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
            "accepted" => Ok(Self::Accepted),
            "queued" => Ok(Self::Queued),
            "running" => Ok(Self::Running),
            "paused" => Ok(Self::Paused),
            "retrying" => Ok(Self::Retrying),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            "timeout" => Ok(Self::Timeout),
            "rejected" => Ok(Self::Rejected),
            "blocked" => Ok(Self::Blocked),
            _ => Err(format!("Invalid task status: {}", s)),
        }
    }
}

impl From<TaskStatus> for String {
    fn from(status: TaskStatus) -> Self {
        match status {
            TaskStatus::Pending => "pending",
            TaskStatus::Accepted => "accepted",
            TaskStatus::Queued => "queued",
            TaskStatus::Running => "running",
            TaskStatus::Paused => "paused",
            TaskStatus::Retrying => "retrying",
            TaskStatus::Completed => "completed",
            TaskStatus::Failed => "failed",
            TaskStatus::Cancelled => "cancelled",
            TaskStatus::Timeout => "timeout",
            TaskStatus::Rejected => "rejected",
            TaskStatus::Blocked => "blocked",
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
