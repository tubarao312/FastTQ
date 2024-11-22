use uuid::Uuid;
use serde::{Serialize, Deserialize};

/// A worker that can execute tasks after receiving them. 
/// We know that it can receive those tasks from its list of capabilities.
/// A worker must register itself with its capabilities to be able to receive tasks.
#[derive(Debug, Serialize, Deserialize)]
pub struct Worker {
    id: Uuid,
    capabilities: Vec<String>,
}