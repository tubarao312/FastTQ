use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Task Type

/// A task type is a type of task that can be executed by a worker.
/// It is defined by its name and its input data schema.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TaskKind {
    pub id: Uuid,
    pub name: String,
}

impl TaskKind {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
        }
    }
}
