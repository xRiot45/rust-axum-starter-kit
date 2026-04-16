use crate::common::errors::AppResult;
use async_trait::async_trait;
use uuid::Uuid;

/// Base repository trait for CRUD operations.
/// Implement this for each domain aggregate root.
#[async_trait]
pub trait Repository<T, CreateDto, UpdateDto>: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<T>>;
    async fn find_all(&self, limit: i64, offset: i64) -> AppResult<Vec<T>>;
    async fn create(&self, dto: CreateDto) -> AppResult<T>;
    async fn update(&self, id: Uuid, dto: UpdateDto) -> AppResult<T>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
}
