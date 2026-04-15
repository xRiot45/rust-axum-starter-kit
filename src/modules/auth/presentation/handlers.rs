use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use crate::common::extractors::ValidatedJson;
use crate::modules::auth::application::dto::{LoginRequest, RefreshRequest};

/// POST /api/v1/auth/login
pub async fn login(ValidatedJson(body): ValidatedJson<LoginRequest>) -> impl IntoResponse {
    // TODO: inject AuthService via AppState and call auth_service.login(body).await
    tracing::info!(email = %body.email, "Login attempt (stub)");
    (StatusCode::OK, Json(json!({
        "access_token": "stub_access_token",
        "refresh_token": "stub_refresh_token",
        "token_type": "Bearer",
        "expires_in": 900
    })))
}

/// POST /api/v1/auth/refresh
pub async fn refresh_token(ValidatedJson(body): ValidatedJson<RefreshRequest>) -> impl IntoResponse {
    tracing::info!("Token refresh attempt (stub)");
    let _ = body;
    (StatusCode::OK, Json(json!({ "message": "Refreshed (stub)" })))
}

/// POST /api/v1/auth/logout
pub async fn logout(Json(body): Json<serde_json::Value>) -> impl IntoResponse {
    tracing::info!("Logout (stub)");
    let _ = body;
    StatusCode::NO_CONTENT
}
