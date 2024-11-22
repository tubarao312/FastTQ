use uuid::Uuid;
use serde::{Serialize, Deserialize};

/// A task that is ready to be executed
/// 
/// Tasks are sent to workers to be executed with a specific payload.
/// Workers are eligble for receiving certain tasks depending on their
/// list of capabilities.
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub data: serde_json::Value,
}

impl Task {
    pub fn new(name: String, data: serde_json::Value, id: Option<Uuid>) -> Self {
        Self { 
            id: id.unwrap_or_else(|| Uuid::new_v4()), 
            name, 
            data 
        }
    }
}

