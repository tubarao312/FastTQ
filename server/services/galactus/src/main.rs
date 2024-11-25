mod api;
mod brokers;
mod config;
mod repo;

use config::Config;
use tokio::net::TcpListener;

use axum::{serve, Router};

use db_common::db::DatabasePools;

use tracing::info;
use tracing_subscriber;

/// Represents the shared application state that can be accessed by all routes
///
/// Contains database connection pools for read and write operations
#[derive(Clone)]
pub struct AppState {
    pub db_pools: DatabasePools,
}

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

#[tokio::main]
async fn main() {
    let config = Config::new();

    // Setup the logger
    setup_logger().await;

    // Setup the database pools
    let db_pools = DatabasePools::new(&config.db_reader_url, &config.db_writer_url)
        .await
        .unwrap();

    info!("Database pools initialized");

    // Setup the app state
    let app_state = AppState { db_pools };

    // Setup the router
    let app = Router::new().merge(api::routes()).with_state(app_state);

    info!("Router initialized");

    // Setup the listener and bind to the port
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("Listener initialized");

    // Serve the app
    serve(listener, app.into_make_service()).await.unwrap();
}
