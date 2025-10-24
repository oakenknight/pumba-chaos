use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub timestamp: String,
}

/// Health check endpoint response
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub redis: String,
    pub timestamp: String,
}

/// Data retrieval response
#[derive(Serialize)]
pub struct DataResponse {
    pub key: String,
    pub value: String,
    pub retrieved_at: String,
}

/// Store data request body
#[derive(Deserialize)]
pub struct StoreRequest {
    pub key: String,
    pub value: String,
}

/// Store data response
#[derive(Serialize)]
pub struct StoreResponse {
    pub success: bool,
    pub key: String,
    pub message: String,
}

/// Slow operation response
#[derive(Serialize)]
pub struct SlowResponse {
    pub message: String,
    pub duration: u32,
}

/// Statistics response
#[derive(Serialize)]
pub struct StatsResponse {
    pub uptime: u64,
    pub requests: u64,
    pub redis_connected: bool,
}
