use async_trait::async_trait;
use uuid::Uuid;
use crate::common::errors::AppResult;
use crate::modules::users::domain::model::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>>;
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>>;
    async fn find_all(&self, limit: i64, offset: i64) -> AppResult<Vec<User>>;
    async fn create(&self, name: &str, email: &str, password_hash: &str) -> AppResult<User>;
    async fn update_name(&self, id: Uuid, name: &str) -> AppResult<User>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
}
