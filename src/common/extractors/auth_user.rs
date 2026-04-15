use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::common::errors::AppError;

/// Claims embedded in the JWT access token.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,       // user id
    pub email: String,
    pub exp: usize,
    pub iat: usize,
}

/// Axum extractor that validates the Bearer token and injects the authenticated user.
pub struct AuthUser(pub Claims);

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let token = extract_bearer_token(&parts.headers)
            .ok_or_else(|| AppError::Unauthorized("Missing or malformed Authorization header".to_string()))?;

        // NOTE: In production, inject the secret from AppState instead of env directly.
        let secret = std::env::var("JWT__SECRET")
            .unwrap_or_else(|_| "change_me_in_production".to_string());

        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| AppError::Unauthorized(format!("Invalid token: {e}")))?;

        Ok(AuthUser(token_data.claims))
    }
}

fn extract_bearer_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|t| t.to_string())
}
