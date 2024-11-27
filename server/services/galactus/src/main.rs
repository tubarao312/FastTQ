mod api;
mod config;
mod repo;

use axum::{serve, Router};
use std::sync::Arc;
use tokio::net::TcpListener;

use common::brokers::Broker;
use config::Config;
use db_common::db::DatabasePools;
use repo::{PgRepositoryCore, PgTaskInstanceRepository, PgTaskKindRepository, PgWorkerRepository};
use tracing::info;
use tracing_subscriber;

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

/// Represents the shared application state that can be accessed by all routes
///
/// Contains all the repositories used for the application logic
#[derive(Clone)]
pub struct AppState {
    pub task_repository: PgTaskInstanceRepository,
    pub task_kind_repository: PgTaskKindRepository,
    pub worker_repository: PgWorkerRepository,
    pub broker: Arc<Broker>,
}

/// Initializes the application state based on the given configuration
async fn setup_app_state(config: Config) -> AppState {
    // Setup the database pools
    let db_pools = DatabasePools::new(&config.db_reader_url, &config.db_writer_url)
        .await
        .unwrap();

    // Setup the repositories
    let core = PgRepositoryCore::new(db_pools.reader);
    let task_repository = PgTaskInstanceRepository::new(core.clone());
    let task_kind_repository = PgTaskKindRepository::new(core.clone());
    let worker_repository = PgWorkerRepository::new(core);

    // Setup the broker
    let broker = Arc::new(
        Broker::new(&config.broker_addr)
            .await
            .expect("Failed to initialize broker"),
    );

    AppState {
        task_repository,
        task_kind_repository,
        worker_repository,
        broker,
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
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port");

    info!("Listener initialized");

    // Serve the app
    serve(listener, app.into_make_service())
        .await
        .expect("Failed to serve app");
}

#[cfg(test)]
pub fn init_test_logger() {
    let _ = tracing_subscriber::fmt()
        .with_test_writer()
        .with_max_level(tracing::Level::DEBUG)
        .try_init();
}
