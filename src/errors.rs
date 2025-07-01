use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O error")]
    IO(#[from] std::io::Error),
    // #[error("Database error: {0}")]
    // DbError(#[from] sqlx::Error),
    #[error("Redis error: {0}")]
    Redis(#[from] deadpool_redis::PoolError),

    #[error("Redis command error: {0}")]
    RedisCmd(#[from] deadpool_redis::redis::RedisError),

    #[error("Not found")]
    NotFound,

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            AppError::Redis(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AppError::RedisCmd(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong".into(),
            ),
            AppError::IO(error) => todo!(),
        };
        let body = Json(json!({ "error": msg }));
        (status, body).into_response()
    }
}
