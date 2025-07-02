use anyhow::{Context};
use axum::http::HeaderValue;
use std::sync::{Arc, LazyLock};

use axum::extract::FromRequestParts;
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation, decode, encode,
};

use crate::{
    errors::AppError, state::AppState, utils::{
        claims::{Claims, UserRole}, load_config, JWT_SERVICE
    }
};

#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
    expiration_seconds: u64,
}

impl JwtService {
    pub fn new(secret: &str, expiration_seconds: u64) -> Self {
        let encoding_key = EncodingKey::from_secret(secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        let validation = Validation::new(jsonwebtoken::Algorithm::HS256);

        Self {
            encoding_key,
            decoding_key,
            validation,
            expiration_seconds,
        }
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
        _state: &AppState,
    ) -> Result<Claims, Self::Rejection>
    {   
        // Extract the Authorization header
        let auth_header =
            parts.headers.get("authorization").ok_or_else(|| {
                AppError::UnauthorizedWithMessage {
                    message: "Missing authorization header".to_string(),
                }
            })?;

        // Convert header to string
        let auth_str = auth_header
            .to_str().expect("msg");
            // .context("convert header to string error")?;
        // Check if it starts with "Bearer "
        if !auth_str.starts_with("Bearer ") {
            return Err(AppError::UnauthorizedWithMessage {
                message: "Not starts with 'Bearer '".to_string(),
            });
        }
        // Extract the token
        let token = &auth_str[7..];

        JWT_SERVICE.validate_token(token)

    }
}
