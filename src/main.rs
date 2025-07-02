use anyhow::{Context, Result};
use axum_p1::{run, utils::init};


#[tokio::main]
async fn main() -> Result<()> {
    init().await?;
    run()
        .await
        .with_context(|| "Failed to run the application")?;
    Ok(())
}
