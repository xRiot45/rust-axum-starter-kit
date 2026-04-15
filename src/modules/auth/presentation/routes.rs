use axum::{routing::post, Router};
use crate::modules::auth::presentation::handlers::{login, logout, refresh_token};

pub fn auth_routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/refresh", post(refresh_token))
        .route("/logout", post(logout))
}
