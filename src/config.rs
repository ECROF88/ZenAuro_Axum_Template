use anyhow::{Context, Result};
use serde::Deserialize;


#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub jwt_secret: String,
    pub database_url: String,
    pub redis_url: String,
    pub expiration_seconds:u64
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        dotenv::dotenv().context("Failed to load .env file")?;

        let config = Self {
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .context("Invalid PORT format")?,

            jwt_secret: std::env::var("JWT_SECRET")
                .context("JWT_SECRET environment variable not set")?,

            database_url: std::env::var("DATABASE_URL")
                .context("DATABASE_URL environment variable not set")?,

            redis_url: std::env::var("REDIS_URL")
                .context("REDIS_URL environment variable not set")?,

            expiration_seconds: std::env::var("EXPIRATION_SECONDS")
                .context("EXPIRATION_SECONDS environment variable not set")?
                .parse()
                .context("Invalid EXPIRATION_SECONDS format")?
        };

        Ok(config)
    }
}
