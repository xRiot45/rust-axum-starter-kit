use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::common::extractors::ValidatedJson;
use crate::common::utils::pagination::PaginationQuery;
use crate::modules::users::application::dto::{CreateUserRequest, UpdateUserRequest};

/// POST /api/v1/users
pub async fn create_user(
    ValidatedJson(body): ValidatedJson<CreateUserRequest>,
) -> impl IntoResponse {
    tracing::info!(email = %body.email, "Creating user (stub)");
    (
        StatusCode::CREATED,
        Json(json!({ "message": "User created (stub)", "email": body.email })),
    )
}

/// GET /api/v1/users
pub async fn list_users(Query(pagination): Query<PaginationQuery>) -> impl IntoResponse {
    tracing::info!(
        page = pagination.page,
        limit = pagination.limit,
        "Listing users (stub)"
    );
    Json(
        json!({ "data": [], "meta": { "page": pagination.page, "limit": pagination.limit, "total": 0 } }),
    )
}

/// GET /api/v1/users/:id
pub async fn get_user(Path(id): Path<Uuid>) -> impl IntoResponse {
    tracing::info!(id = %id, "Getting user (stub)");
    Json(json!({ "id": id, "name": "Stub User", "email": "stub@example.com" }))
}

/// PATCH /api/v1/users/:id
pub async fn update_user(
    Path(id): Path<Uuid>,
    ValidatedJson(body): ValidatedJson<UpdateUserRequest>,
) -> impl IntoResponse {
    tracing::info!(id = %id, name = %body.name, "Updating user (stub)");
    Json(json!({ "id": id, "name": body.name }))
}

/// DELETE /api/v1/users/:id
pub async fn delete_user(Path(id): Path<Uuid>) -> impl IntoResponse {
    tracing::info!(id = %id, "Deleting user (stub)");
    StatusCode::NO_CONTENT
}
