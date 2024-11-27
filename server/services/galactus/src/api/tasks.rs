use std::{alloc::System, time::SystemTime};

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    routing::{get, post, put},
    Router,
};
use serde::Deserialize;
use tracing::{error, info};
use utoipa::ToSchema;
use uuid::Uuid;

use common::{models::TaskInstance, TaskKind, TaskStatus};

use crate::{
    repo::{TaskInstanceRepository, TaskKindRepository},
    AppState,
};

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
#[utoipa::path(
    get,
    description = "Get a task by its UUID",
    path = "/tasks/:id",
    params(
        ("id" = Uuid, Path, description = "Task ID to get")
    ),
    responses(
        (status = 200, description = "Task found", body = TaskInstance, content_type = "application/json"),
        (status = 404, description = "Task not found", content_type = "text/plain"),
        (status = 500, description = "Internal server error", content_type = "text/plain")
    ),
    tag = "tasks"
)]
async fn get_task_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<TaskInstance>, (StatusCode, String)> {
    info!("Getting task by ID: {:?}", id);

    let task = state.task_repository.get_task_by_id(&id, true).await;

    match task {
        Ok(task) => Ok(Json(task)),
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                info!("Task with ID {:?} not found", id);
                Err((
                    StatusCode::NOT_FOUND,
                    format!("Task with ID {} not found", id),
                ))
            }
            _ => {
                error!("Error getting task by id: {:?}", e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to get task: {}", e),
                ))
            }
        },
    }
}

/// Input data for creating a task
#[derive(Debug, Deserialize, ToSchema)]
struct CreateTaskInput {
    task_kind_name: String,
    input_data: Option<serde_json::Value>,
}

/// Create a new task
///
/// # Arguments
/// * `task` - Task object containing name and data
///
/// # Returns
/// Returns a JSON response containing the created task
#[utoipa::path(
    post,
    description = "Create a new task",
    path = "/tasks",
    request_body = CreateTaskInput,
    responses(
        (status = 200, description = "Task created", body = TaskInstance, content_type = "application/json"),
        (status = 500, description = "Internal server error", content_type = "text/plain")
    ),
    tag = "tasks"
)]
async fn create_task(
    State(state): State<AppState>,
    Json(task_input): Json<CreateTaskInput>,
) -> Result<Json<TaskInstance>, (StatusCode, String)> {
    info!("Creating task with kind: {:?}", task_input.task_kind_name);

    // Get / Create the task kind. If it fails, return an error
    let task_kind = state
        .task_kind_repository
        .get_or_create_task_kind(task_input.task_kind_name)
        .await
        .map_err(|e| {
            error!("Failed to get/create task kind: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get/create task kind: {}", e),
            )
        })?;

    // Create and return the new task instance
    let task = state
        .task_repository
        .create_task(task_kind.id, task_input.input_data)
        .await
        .map_err(|e| {
            error!("Failed to create task: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create task: {}", e),
            )
        })?;

    Ok(Json(task))
}

/// Update the status of a task
///
/// # Arguments
/// * `id` - UUID of the task to update
///
/// # Returns
/// Returns a JSON response containing the updated task
#[utoipa::path(
    put,
    description = "Update the status of a task. This should only be used by workers.",
    path = "/tasks/:id/status",
    params(
        ("id" = Uuid, Path, description = "Task ID to update")
    ),
    request_body = String,
    responses(
        (status = 200, description = "Task status updated"),
        (status = 400, description = "Invalid task status", content_type = "text/plain"),
        (status = 404, description = "Task not found", content_type = "text/plain"),
        (status = 500, description = "Internal server error", content_type = "text/plain")
    ),
    tag = "tasks"
)]
async fn update_task_status(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(task_status): Json<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    info!("Updating task status for task: {:?}", id);

    let status = TaskStatus::try_from(task_status.as_str()).map_err(|e| {
        error!("Invalid task status: {:?}", e);
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid task status: {}. Valid statuses are: pending, queued, running, completed, failed, cancelled", e),
        )
    })?;

    let task = state
        .task_repository
        .get_task_by_id(&id, true)
        .await
        .map_err(|e| {
            error!("Task not found: {:?}", e);
            (
                StatusCode::NOT_FOUND,
                format!("Task with id {} not found", id),
            )
        })?;

    state
        .task_repository
        .update_task_status(&task.id, status)
        .await
        .map_err(|e| {
            error!("Failed to update task status: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to update task status: {}", e),
            )
        })?;

    Ok(StatusCode::OK)
}
