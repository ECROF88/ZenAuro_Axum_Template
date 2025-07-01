
use anyhow::{Context, Result};
use axum_p1::{run, utils::init_tracing};


#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    run()
        .await
        .with_context(|| "Failed to run the application")?;
    Ok(())
}
