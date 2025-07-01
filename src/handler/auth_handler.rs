use axum::extract::State;

use crate::{errors::AppError, state::AppState};
// #[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>
) -> Result<String, AppError>{
    Ok(format!("sadsadasd"))
}