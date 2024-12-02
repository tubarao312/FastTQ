mod api;
mod config;
mod repo;
mod testing;

use std::sync::Arc;

use axum::Router;
use sqlx::PgPool;
use tokio::sync::RwLock;
use tracing::info;
use tracing_subscriber;

use common::brokers::Broker;

use config::Config;
use repo::{PgRepositoryCore, PgTaskInstanceRepository, PgTaskKindRepository, PgWorkerRepository};

/// Initializes the logger
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
/// Contains all the repositories used for the application logic and the broker
#[derive(Clone)]
pub struct AppState {
    pub task_repository: PgTaskInstanceRepository,
    pub task_kind_repository: PgTaskKindRepository,
    pub worker_repository: PgWorkerRepository,
    pub broker: Arc<RwLock<Broker>>,
}

/// Creates database connection pools
///
/// # Arguments
///
/// * `config` - The configuration for the database
async fn setup_db_pools(config: &Config) -> PgPool {
    PgPool::connect(&config.db_reader_url).await.unwrap()
}

/// Initializes the broker
///
/// # Arguments
///
/// * `config` - The configuration for the broker   
async fn setup_broker(config: &Config) -> Broker {
    Broker::new(&config.broker_addr)
        .await
        .expect("Failed to initialize broker")
}

/// Initializes the application state based on the given configuration
///
/// # Arguments
///
/// * `db_pools` - The database connection pools
/// * `broker` - The broker
async fn setup_app_state(db_pools: PgPool, broker: Broker) -> AppState {
    // Setup the repositories
    let core = PgRepositoryCore::new(db_pools.clone());
    let task_repository = PgTaskInstanceRepository::new(core.clone());
    let task_kind_repository = PgTaskKindRepository::new(core.clone());
    let worker_repository = PgWorkerRepository::new(core.clone());

    AppState {
        task_repository,
        task_kind_repository,
        worker_repository,
        broker: Arc::new(RwLock::new(broker)),
    }
}

/// Initializes the application router
///
/// # Arguments
///
/// * `db_pools` - The database connection pools
/// * `broker` - The broker
async fn setup_app(db_pools: PgPool, broker: Broker) -> Router {
    let app_state = setup_app_state(db_pools, broker).await;
    info!("App state created");
    Router::new().merge(api::routes()).with_state(app_state)
}

fn main() {
    println!("Hello from Galactus!");
}
