use axum::{body::Body, extract::{Request, State}, Json};

use crate::{errors::AppError, state::AppState};
#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(json): Json<String>,
) -> Result<String, AppError>{
    Ok(format!("sadsadasd"))
}