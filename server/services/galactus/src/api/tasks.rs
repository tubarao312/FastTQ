use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    routing::{get, post, put},
    Router,
};
use uuid::Uuid;

use common::models::Task;

use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/:id", get(get_task_by_id))
        .route("/", post(create_task))
        .route("/:id/status", put(update_task_status))
}

/// Get a task by its UUID
///
/// # Arguments
/// * `id` - UUID of the task to retrieve
///
/// # Returns
/// Returns a JSON response containing the task if found
async fn get_task_by_id(
    Path(id): Path<Uuid>,
    State(AppState { db_pools }): State<AppState>,
) -> Result<Json<Task>, StatusCode> {
    todo!("Implement me")
}

/// Create a new task
///
/// # Arguments
/// * `task` - Task object containing name and data
///
/// # Returns
/// Returns a JSON response containing the created task
async fn create_task(
    State(AppState { db_pools }): State<AppState>,
    Json(task): Json<Task>,
) -> Result<Json<Task>, StatusCode> {
    todo!("Implement me")
}

/// Update the status of a task
///
/// # Arguments
/// * `id` - UUID of the task to update
///
/// # Returns
/// Returns a JSON response containing the updated task
async fn update_task_status(
    Path(id): Path<Uuid>,
    State(AppState { db_pools }): State<AppState>,
) -> Result<Json<Task>, StatusCode> {
    todo!("Implement me")
}
