use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use redis::AsyncCommands;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info};

use crate::state::AppState;

const DATA_TTL_SECONDS: u64 = 300;
use crate::errors::*;
use crate::structs::*;

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::Timeout(msg) => (StatusCode::REQUEST_TIMEOUT, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(ErrorResponse {
            error: message,
            timestamp: Utc::now().to_rfc3339(),
        });

        (status, body).into_response()
    }
}

/// GET /health - Health check endpoint
pub async fn health_handler(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    let redis_status = if state.is_redis_connected().await {
        "connected"
    } else {
        "disconnected"
    };

    Json(HealthResponse {
        status: "ok".to_string(),
        redis: redis_status.to_string(),
        timestamp: Utc::now().to_rfc3339(),
    })
}

/// GET /data/:key - Retrieve data from Redis
pub async fn get_data_handler(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> Result<Json<DataResponse>, ApiError> {
    info!("Retrieving data for key: {}", key);

    let mut conn = state.redis_client.clone();

    // Query Redis with timeout
    let result =
        tokio::time::timeout(Duration::from_secs(2), conn.get::<_, Option<String>>(&key)).await;

    match result {
        Ok(Ok(Some(value))) => {
            info!("Successfully retrieved value for key: {}", key);
            Ok(Json(DataResponse {
                key,
                value,
                retrieved_at: Utc::now().to_rfc3339(),
            }))
        }
        Ok(Ok(None)) => {
            error!("Key not found: {}", key);
            Err(ApiError::NotFound(format!("Key '{}' not found", key)))
        }
        Ok(Err(e)) => {
            error!("Redis error: {}", e);
            Err(ApiError::InternalError(format!("Redis error: {}", e)))
        }
        Err(_) => {
            error!("Redis timeout for key: {}", key);
            Err(ApiError::Timeout("Redis operation timed out".to_string()))
        }
    }
}

/// POST /store - Store data in Redis
pub async fn store_data_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<StoreRequest>,
) -> Result<Json<StoreResponse>, ApiError> {
    // Validate input
    if payload.key.is_empty() {
        return Err(ApiError::BadRequest("Key cannot be empty".to_string()));
    }
    if payload.value.is_empty() {
        return Err(ApiError::BadRequest("Value cannot be empty".to_string()));
    }

    info!("Storing data for key: {}", payload.key);

    let mut conn = state.redis_client.clone();

    // Store in Redis with TTL of 300 seconds
    let result = tokio::time::timeout(
        Duration::from_secs(2),
        conn.set_ex::<_, _, ()>(&payload.key, &payload.value, DATA_TTL_SECONDS),
    )
    .await;

    match result {
        Ok(Ok(_)) => {
            info!("Successfully stored value for key: {}", payload.key);
            Ok(Json(StoreResponse {
                success: true,
                key: payload.key,
                message: "Data stored successfully with 300 second TTL".to_string(),
            }))
        }
        Ok(Err(e)) => {
            error!("Redis error: {}", e);
            Err(ApiError::InternalError(format!("Redis error: {}", e)))
        }
        Err(_) => {
            error!("Redis timeout for key: {}", payload.key);
            Err(ApiError::Timeout("Redis operation timed out".to_string()))
        }
    }
}

/// GET /slow - Slow operation that takes 3 seconds
pub async fn slow_handler() -> Json<SlowResponse> {
    info!("Starting slow operation");
    tokio::time::sleep(Duration::from_secs(3)).await;
    info!("Slow operation completed");

    Json(SlowResponse {
        message: "Slow operation completed".to_string(),
        duration: 3000,
    })
}

/// GET /stats - Get application statistics
pub async fn stats_handler(State(state): State<Arc<AppState>>) -> Json<StatsResponse> {
    let uptime = state.get_uptime();
    let requests = state.get_request_count();
    let redis_connected = state.is_redis_connected().await;

    Json(StatsResponse {
        uptime,
        requests,
        redis_connected,
    })
}
