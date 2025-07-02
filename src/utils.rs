use std::sync::{LazyLock, OnceLock};
pub mod claims;
pub mod jwt;
use jwt::JwtService;
use tracing_subscriber::fmt;

use crate::config::AppConfig;
static TRACING: OnceLock<()> = OnceLock::new();
static JWT_SERVICE: LazyLock<JwtService> = LazyLock::new(|| {
    let config = tokio::runtime::Handle::current()
        .block_on(load_config())
        .expect("Failed to load config");
    
    JwtService::new(&config.jwt_secret, config.expiration_seconds)
});
pub async fn init() -> anyhow::Result<()> {
    TRACING.get_or_init(|| {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_span_events(fmt::format::FmtSpan::CLOSE) // 显示 span 进入/退出
            .with_target(true) // 显示模块路径
            .with_thread_ids(true) // 显示线程ID
            .with_file(true) // 显示文件名
            .with_line_number(true) // 显示行号
            // .pretty()
            .init();
    });

    Ok(())
}

pub async fn load_config() -> anyhow::Result<AppConfig> {
    let app_config = AppConfig::load()?;
    Ok(app_config)
}


pub fn get_jwt_service() -> &'static JwtService {
    &JWT_SERVICE
}