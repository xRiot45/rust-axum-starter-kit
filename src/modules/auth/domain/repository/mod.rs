use async_trait::async_trait;
use uuid::Uuid;
use crate::common::errors::AppResult;
use crate::modules::auth::domain::model::AuthToken;

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn store_refresh_token(&self, user_id: Uuid, token: &str, expires_at: chrono::DateTime<chrono::Utc>) -> AppResult<AuthToken>;
    async fn find_refresh_token(&self, token: &str) -> AppResult<Option<AuthToken>>;
    async fn revoke_refresh_token(&self, token: &str) -> AppResult<()>;
    async fn revoke_all_user_tokens(&self, user_id: Uuid) -> AppResult<()>;
}
