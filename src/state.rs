use anyhow::{Context, Result};
use redis::aio::ConnectionManager;
use redis::Client;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{error, info};

/// Application state shared across all request handlers
#[derive(Clone)]
pub struct AppState {
    pub redis_client: ConnectionManager,
    pub start_time: Instant,
    pub request_count: Arc<AtomicU64>,
}

impl AppState {
    /// Creates a new AppState with a Redis connection manager
    /// Retries connection 5 times with 1 second delay between attempts
    pub async fn new(redis_url: &str) -> Result<Self> {
        info!("Connecting to Redis at {}", redis_url);

        let client = Client::open(redis_url).context("Failed to create Redis client")?;

        // Retry logic: 5 attempts with 1 second between retries
        let mut last_error = None;
        for attempt in 1..=5 {
            match Self::create_connection_manager(&client).await {
                Ok(manager) => {
                    info!("Successfully connected to Redis on attempt {}", attempt);
                    return Ok(Self {
                        redis_client: manager,
                        start_time: Instant::now(),
                        request_count: Arc::new(AtomicU64::new(0)),
                    });
                }
                Err(e) => {
                    error!("Redis connection attempt {} failed: {}", attempt, e);
                    last_error = Some(e);
                    if attempt < 5 {
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                }
            }
        }

        Err(last_error
            .unwrap_or_else(|| anyhow::anyhow!("Failed to connect to Redis after 3 attempts")))
    }

    /// Creates a connection manager with timeout
    async fn create_connection_manager(client: &Client) -> Result<ConnectionManager> {
        tokio::time::timeout(
            Duration::from_secs(2),
            ConnectionManager::new(client.clone()),
        )
        .await
        .context("Connection timeout after 2 seconds")?
        .context("Failed to create connection manager")
    }

    /// Checks if Redis is connected by sending a PING command
    pub async fn is_redis_connected(&self) -> bool {
        let mut conn = self.redis_client.clone();

        match tokio::time::timeout(
            Duration::from_secs(2),
            redis::cmd("PING").query_async::<_, String>(&mut conn),
        )
        .await
        {
            Ok(Ok(response)) => response == "PONG",
            Ok(Err(e)) => {
                error!("Redis PING failed: {}", e);
                false
            }
            Err(_) => {
                error!("Redis PING timeout");
                false
            }
        }
    }

    /// Atomically increments the request counter
    pub fn increment_request_count(&self) {
        self.request_count.fetch_add(1, Ordering::SeqCst);
    }

    /// Gets the current request count
    pub fn get_request_count(&self) -> u64 {
        self.request_count.load(Ordering::SeqCst)
    }

    /// Returns uptime in seconds since application start
    pub fn get_uptime(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }
}
