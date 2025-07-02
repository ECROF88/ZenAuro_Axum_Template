use tracing::info;

use crate::{
    errors::AppError, router::create_router, state::AppState,
    utils::load_config,
};

pub mod config;
pub mod errors;
pub mod handler;
pub mod router;
pub mod state;
pub mod utils;

pub async fn run() -> Result<(), AppError> {
    let cfg = load_config().await?;

    let router = create_router(AppState::new(cfg.redis_url).await?)?;

    let addr = format!("0.0.0.0:{}", cfg.port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("🚀 Server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router).await?;
    Ok(())
}
