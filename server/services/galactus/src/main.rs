mod api;
mod brokers;
mod config;
mod repo;

use std::sync::Arc;

use config::Config;
use tokio::net::TcpListener;

use axum::{serve, Router};

use db_common::db::DatabasePools;
use repo::{
    PgRepositoryCore, PgTaskRepository, PgTaskTypeRepository, PgWorkerRepository, TaskRepository,
    TaskTypeRepository, WorkerRepository,
};

use tracing::info;
use tracing_subscriber;

/// Represents the shared application state that can be accessed by all routes
///
/// Contains all the repositories used for the application logic
#[derive(Clone)]
pub struct AppState {
    pub task_repository: PgTaskRepository,
    pub task_type_repository: PgTaskTypeRepository,
    pub worker_repository: PgWorkerRepository,
}

/// Initializes the logger with the appropriate formatting
async fn setup_logger() {
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .init();

    info!("Logger initialized");
}

/// Initializes the application state based on the given configuration
async fn setup_app_state(config: Config) -> AppState {
    // Setup the database pools
    let db_pools = DatabasePools::new(&config.db_reader_url, &config.db_writer_url)
        .await
        .unwrap();

    // Setup the repositories
    let core = PgRepositoryCore::new(db_pools.reader);
    let task_repository: PgTaskRepository = PgTaskRepository::new(core.clone());
    let task_type_repository = PgTaskTypeRepository::new(core.clone());
    let worker_repository = PgWorkerRepository::new(core);

    AppState {
        task_repository,
        task_type_repository,
        worker_repository,
    }
}

#[tokio::main]
async fn main() {
    let config = Config::new();

    // Setup the logger
    setup_logger().await;

    info!("Database pools initialized");

    let app_state = setup_app_state(config).await;

    info!("App state initialized");

    // Setup the router
    let app = Router::new().merge(api::routes()).with_state(app_state);

    info!("Router initialized");

    // Setup the listener and bind to the port
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("Listener initialized");

    // Serve the app
    serve(listener, app.into_make_service()).await.unwrap();
}

#[cfg(test)]
pub fn init_test_logger() {
    let _ = tracing_subscriber::fmt()
        .with_test_writer()
        .with_max_level(tracing::Level::DEBUG)
        .try_init();
}
