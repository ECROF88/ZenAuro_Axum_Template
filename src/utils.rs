use std::sync::OnceLock;

use tracing_subscriber::fmt;

use crate::config::AppConfig;
static TRACING: OnceLock<()> = OnceLock::new();

pub fn init_tracing() {
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
}

pub async fn load_config() -> anyhow::Result<AppConfig> {
    let app_config = AppConfig::load()?;
    Ok(app_config)
}
