mod api;
mod config;
mod repo;

use config::Config;
use tokio::net::TcpListener;

use axum::{serve, Router};

use common::brokers::Broker;
use db_common::db::DatabasePools;

use tracing::info;
use tracing_subscriber;

use std::sync::Arc;

/// Represents the shared application state that can be accessed by all routes
///
/// Contains database connection pools for read and write operations
#[derive(Clone)]
pub struct AppState {
    pub db_pools: DatabasePools,
    pub broker: Arc<Broker>,
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
        .expect("Failed to initialize database pools");

    info!("Database pools initialized");

    // Setup the broker
    let broker = Arc::new(
        Broker::new(&config.broker_addr)
            .await
            .expect("Failed to initialize broker"),
    );

    // Setup the app state
    let app_state = AppState { db_pools, broker };

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
