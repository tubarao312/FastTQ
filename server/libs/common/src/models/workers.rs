use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

use crate::models::TaskType;

/// A worker that can execute tasks after receiving them.
/// We know that it can receive those tasks from its list of capabilities.
/// A worker must register itself with its capabilities to be able to receive tasks.
#[derive(Debug, Serialize, Deserialize)]
pub struct Worker {
    pub id: Uuid,
    pub name: String,
    pub registered_at: SystemTime,
    pub task_types: Vec<TaskType>,
    pub active: bool,
}
