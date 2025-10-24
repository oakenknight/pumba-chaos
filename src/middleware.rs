use axum::{body::Body, extract::State, http::Request, middleware::Next, response::IntoResponse};
use chrono::Utc;
use std::sync::Arc;
use tracing::info;

use crate::state::AppState;

/// Middleware that logs requests and increments request counter
pub async fn request_counter_middleware(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    // Log request details
    let method = req.method().clone();
    let uri = req.uri().clone();
    let timestamp = Utc::now().to_rfc3339();

    // Increment request counter
    state.increment_request_count();

    // Process request
    let response = next.run(req).await;

    // Log response
    let status = response.status();
    info!(
        "[{}] {} {} - Status: {}",
        timestamp,
        method,
        uri.path(),
        status.as_u16()
    );

    response
}
