use std::sync::Arc;

use anyhow::Context;
use deadpool_redis::{Config, Pool, redis::AsyncCommands};
use tracing::{debug, info, instrument};

use crate::utils::{jwt::JwtService, ENV_CONFIG};

#[derive(Clone)]
pub struct AppState {
    pub redis: Pool,
    pub jwt_service: JwtService,
}

impl AppState {
    #[instrument(name = "create_app_state", skip())]
    pub async fn new() -> anyhow::Result<AppState> {
        let jwt_secret = &ENV_CONFIG.jwt_secret;
        let expiration_seconds = ENV_CONFIG.expiration_seconds;
        debug!("Creating Redis connection pool");
        let redis_cfg = Config::from_url(&ENV_CONFIG.redis_url);
        let redis =
            redis_cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1))?;

        debug!("Creating JWT service");
        let jwt_service = JwtService::new(jwt_secret, expiration_seconds)?;
        
        debug!("Testing Redis connection");
        let mut conn = redis
            .get()
            .await
            .context("Failed to get Redis connection from pool")?;

        debug!("Sending PING command to Redis");
        let response: String =
            conn.ping().await.context("Failed to ping Redis server")?;

        info!(response = %response, "✅ Redis connection established successfully");

        Ok(Self { redis, jwt_service })
    }
}
