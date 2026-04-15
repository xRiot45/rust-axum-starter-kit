use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;

use crate::common::errors::{AppError, AppResult};
use crate::common::utils::{hash::verify_password, jwt::generate_access_token};
use crate::configs::jwt::JwtKeys;
use crate::modules::auth::application::dto::{AuthResponse, LoginRequest, RefreshRequest};
use crate::modules::auth::domain::repository::AuthRepository;
use crate::modules::users::domain::repository::UserRepository;

pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    auth_repo: Arc<dyn AuthRepository>,
    jwt: JwtKeys,
}

impl AuthService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        auth_repo: Arc<dyn AuthRepository>,
        jwt: JwtKeys,
    ) -> Self {
        Self { user_repo, auth_repo, jwt }
    }

    pub async fn login(&self, req: LoginRequest) -> AppResult<AuthResponse> {
        let user = self.user_repo
            .find_by_email(&req.email)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;

        if !verify_password(&req.password, &user.password_hash)? {
            return Err(AppError::Unauthorized("Invalid email or password".to_string()));
        }

        let access_token = generate_access_token(
            user.id,
            &user.email,
            &self.jwt.secret,
            self.jwt.access_expiry_secs,
        )?;

        let refresh_token = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + chrono::Duration::seconds(self.jwt.refresh_expiry_secs as i64);
        self.auth_repo.store_refresh_token(user.id, &refresh_token, expires_at).await?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.jwt.access_expiry_secs,
        })
    }

    pub async fn refresh(&self, req: RefreshRequest) -> AppResult<AuthResponse> {
        let stored = self.auth_repo
            .find_refresh_token(&req.refresh_token)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid refresh token".to_string()))?;

        if stored.expires_at < Utc::now() {
            return Err(AppError::Unauthorized("Refresh token expired".to_string()));
        }

        self.auth_repo.revoke_refresh_token(&req.refresh_token).await?;

        let user = self.user_repo
            .find_by_id(stored.user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        let access_token = generate_access_token(
            user.id,
            &user.email,
            &self.jwt.secret,
            self.jwt.access_expiry_secs,
        )?;

        let new_refresh = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + chrono::Duration::seconds(self.jwt.refresh_expiry_secs as i64);
        self.auth_repo.store_refresh_token(user.id, &new_refresh, expires_at).await?;

        Ok(AuthResponse {
            access_token,
            refresh_token: new_refresh,
            token_type: "Bearer".to_string(),
            expires_in: self.jwt.access_expiry_secs,
        })
    }

    pub async fn logout(&self, refresh_token: &str) -> AppResult<()> {
        self.auth_repo.revoke_refresh_token(refresh_token).await
    }
}
