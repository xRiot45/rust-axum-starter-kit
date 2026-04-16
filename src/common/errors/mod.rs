use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use uuid::Uuid;

/// Field-level validation error
#[derive(Debug, Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

/// Application-wide error type.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Unprocessable entity: {0}")]
    UnprocessableEntity(String),

    #[error("Internal server error")]
    InternalServerError(#[from] anyhow::Error),

    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Validation failed")]
    ValidationError(Vec<FieldError>),
}

impl AppError {
    fn code(&self) -> &'static str {
        match self {
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::BadRequest(_) => "BAD_REQUEST",
            AppError::Unauthorized(_) => "UNAUTHORIZED",
            AppError::Forbidden(_) => "FORBIDDEN",
            AppError::Conflict(_) => "CONFLICT",
            AppError::UnprocessableEntity(_) => "UNPROCESSABLE_ENTITY",
            AppError::ValidationError(_) => "VALIDATION_ERROR",
            AppError::DatabaseError(_) => "DATABASE_ERROR",
            AppError::InternalServerError(_) => "INTERNAL_SERVER_ERROR",
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::DatabaseError(e) => {
                tracing::error!(error = %e, "Database error");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            AppError::InternalServerError(e) => {
                tracing::error!(error = %e, "Internal error");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        let request_id = format!("req_{}", Uuid::new_v4());

        // Base error message
        let error_message = match &self {
            AppError::DatabaseError(_) | AppError::InternalServerError(_) => {
                "Internal server error".to_string()
            }
            AppError::ValidationError(_) => "Validation failed".to_string(),
            _ => self.to_string(),
        };

        // Build error body
        let mut error_body = json!({
            "type": self.code(),
            "message": error_message
        });

        if let AppError::ValidationError(details) = &self {
            error_body["details"] = json!(details);
        }

        let body = json!({
            "success": false,
            "statusCode": status.as_u16(),
            "error": error_body,
            "meta": {
                "timestamp": Utc::now().to_rfc3339(),
                "request_id": request_id,
                "path": "",   // TODO: inject from middleware
                "method": ""  // TODO: inject from middleware
            }
        });

        (status, Json(body)).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
