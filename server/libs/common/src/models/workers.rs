use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

/// A worker that can execute tasks after receiving them.
/// We know that it can receive those tasks from its list of capabilities.
/// A worker must register itself with its capabilities to be able to receive tasks.
#[derive(Debug, Serialize, Deserialize)]
pub struct Worker {
    id: Uuid,
    name: String,
    registered_at: OffsetDateTime,
    capabilities: Vec<String>,
}
