
use axum::{
    Json,
    body::Body,
    extract::{Request, State},
};

use crate::{
    errors::AppError,
    state::AppState,
    utils::{claims::{self, Claims}, jwt::{self, JwtService}},
};
#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(json): Json<String>,
) -> Result<String, AppError> {
    Ok(format!("sadsadasd"))
}

pub async fn show_token(
    State(jwt_service): State<JwtService>,
    // claims: Claims,
) -> Result<String, AppError> {
    // println!("{:?}",state.jwt_service);
    println!("sec:{}",jwt_service.expiration_seconds);
    // Ok(claims.sub)
    Ok("value".to_string())
}
