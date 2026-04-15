use std::sync::Arc;
use uuid::Uuid;
use crate::common::errors::{AppError, AppResult};
use crate::common::utils::hash::hash_password;
use crate::modules::users::application::dto::{CreateUserRequest, UpdateUserRequest};
use crate::modules::users::domain::model::{User, UserProfile};
use crate::modules::users::domain::repository::UserRepository;

pub struct UserService {
    repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn create(&self, req: CreateUserRequest) -> AppResult<UserProfile> {
        // Prevent duplicate emails
        if self.repo.find_by_email(&req.email).await?.is_some() {
            return Err(AppError::Conflict(format!("Email '{}' is already in use", req.email)));
        }
        let hash = hash_password(&req.password)?;
        let user = self.repo.create(&req.name, &req.email, &hash).await?;
        Ok(user.into())
    }

    pub async fn get_by_id(&self, id: Uuid) -> AppResult<UserProfile> {
        self.repo
            .find_by_id(id)
            .await?
            .map(Into::into)
            .ok_or_else(|| AppError::NotFound(format!("User {id} not found")))
    }

    pub async fn list(&self, limit: i64, offset: i64) -> AppResult<Vec<UserProfile>> {
        let users = self.repo.find_all(limit, offset).await?;
        Ok(users.into_iter().map(Into::into).collect())
    }

    pub async fn update(&self, id: Uuid, req: UpdateUserRequest) -> AppResult<UserProfile> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User {id} not found")))?;
        let user = self.repo.update_name(id, &req.name).await?;
        Ok(user.into())
    }

    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User {id} not found")))?;
        self.repo.delete(id).await
    }
}
