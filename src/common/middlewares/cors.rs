use axum::http::{HeaderName, Method};
use tower_http::cors::{Any, CorsLayer};

/// Permissive CORS for development.
/// In production, replace `Any` origins with an allow-list from config.
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            HeaderName::from_static("content-type"),
            HeaderName::from_static("authorization"),
            HeaderName::from_static("x-request-id"),
        ])
}
