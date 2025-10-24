#![warn(clippy::all)]

mod errors;
mod handlers;
mod middleware;
mod state;
mod structs;

use axum::{
    middleware as axum_middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::signal;
use tracing::{error, info};
use tracing_subscriber;

use crate::handlers::{
    get_data_handler, health_handler, slow_handler, stats_handler, store_data_handler,
};
use crate::middleware::request_counter_middleware;
use crate::state::AppState;
#[tokio::main]
async fn main() {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    info!("Starting Pumba Chaos Demo application");

    // Get Redis URL from environment variable
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://redis:6379".to_string());

    // Create application state with Redis connection
    let state = match AppState::new(&redis_url).await {
        Ok(state) => Arc::new(state),
        Err(e) => {
            error!("Failed to initialize application state: {}", e);
            std::process::exit(1);
        }
    };

    // Build the application router
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/data/:key", get(get_data_handler))
        .route("/store", post(store_data_handler))
        .route("/slow", get(slow_handler))
        .route("/stats", get(stats_handler))
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            request_counter_middleware,
        ))
        .with_state(state);

    // Bind to 0.0.0.0:3000
    let addr = "0.0.0.0:3000";
    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(e) => {
            error!("Failed to bind to {}: {}", addr, e);
            std::process::exit(1);
        }
    };

    info!("Server listening on http://{}", addr);

    // Start server with graceful shutdown
    if let Err(e) = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
    {
        error!("Server error: {}", e);
    }

    info!("Shutting down gracefully");
}

/// Handles graceful shutdown on SIGTERM or SIGINT
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C signal");
        },
        _ = terminate => {
            info!("Received SIGTERM signal");
        },
    }
}
