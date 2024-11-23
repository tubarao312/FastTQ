mod api;
mod brokers;
mod config;
mod repo;

use config::Config;
use tokio::net::TcpListener;

use axum::{serve, Router};

use db_common::db::DatabasePools;

/// Represents the shared application state that can be accessed by all routes
///
/// Contains database connection pools for read and write operations
#[derive(Clone)]
pub struct AppState {
    pub db_pools: DatabasePools,
}

#[tokio::main]
async fn main() {
    let config = Config::new();

    // Setup the database pools
    let db_pools = DatabasePools::new(&config.db_reader_url, &config.db_writer_url)
        .await
        .unwrap();

    // Setup the app state
    let app_state = AppState { db_pools };

    // Setup the router
    let app = Router::new().merge(api::routes()).with_state(app_state);

    // Setup the listener and bind to the port
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // Serve the app
    serve(listener, app.into_make_service()).await.unwrap();
}
