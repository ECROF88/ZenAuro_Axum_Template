use anyhow::Context;
use deadpool_redis::{Config, Pool, redis::AsyncCommands};
use tracing::{debug, info, instrument};


#[derive(Clone)]
pub struct AppState {
    pub redis: Pool,
}

impl AppState {
    #[instrument(name = "create_app_state", skip(redis_url))]
    pub async fn new(redis_url:String) -> anyhow::Result<AppState> {
        debug!("Creating Redis connection pool");
        let cfg = Config::from_url(redis_url);
        let redis = cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1))?;

        debug!("Testing Redis connection");
        let mut conn = redis.get().await
            .context("Failed to get Redis connection from pool")?;

        debug!("Sending PING command to Redis");
        let response: String = conn.ping().await
            .context("Failed to ping Redis server")?;

        info!(response = %response, "✅ Redis connection established successfully");

        Ok(Self { redis })
    }
}
