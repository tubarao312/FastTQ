use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    routing::{delete, post, put},
    Router,
};
use uuid::Uuid;

use common::models::Worker;

use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/workers", post(register_worker))
        .route("/workers/:id", delete(unregister_worker))
    // .route("/workers/:id/heartbeat", put(heartbeat))
}

/// Register a new worker
///
/// # Arguments
/// * `worker` - Worker object containing name and data
///
/// # Returns
/// Returns a JSON response containing the registered worker
async fn register_worker(
    State(state): State<AppState>,
    Json(worker): Json<Worker>,
) -> Result<Json<Worker>, StatusCode> {
    todo!("Implement worker registration")
}

/// Unregister an existing worker
///
/// # Arguments
/// * `id` - UUID of the worker to unregister
///
/// # Returns
/// Returns a JSON response containing the unregistered worker
async fn unregister_worker(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Worker>, StatusCode> {
    todo!("Implement worker unregistration")
}
