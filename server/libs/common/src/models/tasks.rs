use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use strum_macros::Display;
use uuid::Uuid;

use crate::models::TaskType;

// Task status enum

/// # Possible Status:
/// * `Pending`: Task is created but not yet assigned
/// * `Queued`: Task has been assigned to a worker and sent to a queue
/// * `Running`: Worker has started processing
/// * `Completed`: Task completed successfully
/// * `Failed`: Task failed to complete
/// * `Cancelled`: Task was cancelled before completion
#[derive(Display, Debug, Serialize, Deserialize, PartialEq)]
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
        match s.to_lowercase().as_str() {
            "pending" => Self::Pending,
            "queued" => Self::Queued,
            "running" => Self::Running,
            "completed" => Self::Completed,
            "failed" => Self::Failed,
            "cancelled" => Self::Cancelled,
            _ => panic!("Invalid task status: {}", s),
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
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub task_type: TaskType,
    pub input_data: Option<serde_json::Value>,
    pub status: TaskStatus,
    pub created_at: SystemTime,
    pub assigned_to: Option<Uuid>,
}

/// Task results contain the output or error data from a completed task
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResult {
    pub id: Uuid,
    pub task_id: Uuid,
    pub output_data: Option<serde_json::Value>,
    pub error_data: Option<serde_json::Value>,
    pub worker_id: Uuid,
    pub created_at: SystemTime,
}

impl Task {
    pub fn new(task_type: TaskType, input_data: Option<serde_json::Value>) -> Self {
        Self {
            id: Uuid::new_v4(),
            task_type,
            input_data,
            status: TaskStatus::Pending,
            created_at: SystemTime::now(),
            assigned_to: None,
        }
    }
}
