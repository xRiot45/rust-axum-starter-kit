use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

use anyhow::Error as AnyhowError;
use sqlx::Error as SqlxError;

/// Field-level validation error
#[derive(Debug, Serialize, Clone)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

/// Error detail body
#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    pub r#type: String,
    pub message: String,
    pub details: Option<Vec<FieldError>>,
}

/// Metadata for error response
#[derive(Debug, Serialize)]
pub struct ErrorMeta {
    pub timestamp: String,
    pub request_id: String,
    pub path: String,
    pub method: String,
}

/// Final error response shape
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub status_code: u16,
    pub error: ErrorDetail,
    pub meta: ErrorMeta,
}

/// Application-wide error type
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

    #[error("Validation failed")]
    ValidationError(Vec<FieldError>),

    #[error("Database error")]
    DatabaseError(#[from] SqlxError),

    #[error("Internal server error")]
    InternalServerError(#[from] AnyhowError),
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

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn public_message(&self) -> String {
        match self {
            AppError::DatabaseError(_) | AppError::InternalServerError(_) => {
                "Internal server error".to_string()
            }
            AppError::ValidationError(_) => "Validation failed".to_string(),
            _ => self.to_string(),
        }
    }

    /// Helper constructors
    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::BadRequest(msg.into())
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    pub fn unauthorized(msg: impl Into<String>) -> Self {
        Self::Unauthorized(msg.into())
    }

    pub fn forbidden(msg: impl Into<String>) -> Self {
        Self::Forbidden(msg.into())
    }

    pub fn conflict(msg: impl Into<String>) -> Self {
        Self::Conflict(msg.into())
    }
}

/// Convert validator errors → AppError
impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        let field_errors = err
            .field_errors()
            .iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |e| FieldError {
                    field: field.to_string(),
                    message: e
                        .message
                        .clone()
                        .unwrap_or_else(|| "Invalid value".into())
                        .to_string(),
                })
            })
            .collect();

        AppError::ValidationError(field_errors)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();

        // Logging (only for server errors)
        match &self {
            AppError::DatabaseError(e) => {
                tracing::error!(error = %e, error_type = %self.code(), "Database error");
            }
            AppError::InternalServerError(e) => {
                tracing::error!(error = %e, error_type = %self.code(), "Internal server error");
            }
            _ => {}
        }

        // TODO: Replace with middleware injection
        let request_id = format!("req_{}", Uuid::new_v4());

        let error_detail = ErrorDetail {
            r#type: self.code().to_string(),
            message: self.public_message(),
            details: match self {
                AppError::ValidationError(ref d) => Some(d.clone()),
                _ => None,
            },
        };

        let response = ErrorResponse {
            success: false,
            status_code: status_code.as_u16(),
            error: error_detail,
            meta: ErrorMeta {
                timestamp: Utc::now().to_rfc3339(),
                request_id,
                path: "".to_string(),
                method: "".to_string(),
            },
        };

        (status_code, Json(response)).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
