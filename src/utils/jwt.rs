use anyhow::Context;
use axum::{extract::FromRef, http::HeaderValue};
use std::{fmt, sync::{Arc, LazyLock}};

use axum::extract::FromRequestParts;
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation, decode, encode,
};

use crate::{
    errors::AppError,
    state::AppState,
    utils::claims::{Claims, UserRole},
};


#[derive(Clone)]
pub struct JwtService {
    encoding_key: Arc<EncodingKey>,
    decoding_key: Arc<DecodingKey>,
    validation: Arc<Validation>,
    expiration_seconds: u64,
}

impl fmt::Debug for JwtService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JwtService")
            .field("validation", &self.validation)
            .field("expiration_seconds", &self.expiration_seconds)
            .finish_non_exhaustive()
    }
}


impl FromRef<AppState> for JwtService {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.jwt_service.clone()
    }
}
impl JwtService {
    pub fn new(secret: &str, expiration_seconds: u64) -> anyhow::Result<Self> {
        let encoding_key = Arc::new(EncodingKey::from_secret(secret.as_bytes()));
        let decoding_key = Arc::new(DecodingKey::from_secret(secret.as_bytes()));
        let validation = Arc::new(Validation::new(jsonwebtoken::Algorithm::HS256));

        Ok(Self {
            encoding_key,
            decoding_key,
            validation,
            expiration_seconds,
        })
    }

    pub fn generate_token(
        &self,
        user_id: &str,
        username: &str,
        roles: Vec<UserRole>,
    ) -> anyhow::Result<String> {
        let claim =
            Claims::new(user_id, username, roles, self.expiration_seconds);
        encode(&Header::default(), &claim, &self.encoding_key)
            .context("Generation Error")
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, AppError> {
        let token_data =
            decode::<Claims>(token, &self.decoding_key, &self.validation)?;
        let c: Claims = token_data.claims;

        Ok(c)
    }
}

impl FromRequestParts<AppState> for Claims {
    type Rejection = AppError;
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Claims, Self::Rejection> {
        // Extract the Authorization header
        let auth_header =
            parts.headers.get("authorization").ok_or_else(|| {
                AppError::UnauthorizedWithMessage {
                    message: "Missing authorization header".to_string(),
                }
            })?;

        // Convert header to string
        let auth_str = auth_header.to_str().expect("msg");
        // .context("convert header to string error")?;
        // Check if it starts with "Bearer "
        if !auth_str.starts_with("Bearer ") {
            return Err(AppError::UnauthorizedWithMessage {
                message: "Not starts with 'Bearer '".to_string(),
            });
        }
        // Extract the token
        let token = &auth_str[7..];
        state.jwt_service.validate_token(token)
    }
}
