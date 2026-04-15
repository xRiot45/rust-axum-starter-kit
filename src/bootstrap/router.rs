use crate::bootstrap::state::AppState;
use crate::modules::auth::presentation::routes::auth_routes;
use crate::modules::users::presentation::routes::user_routes;
use axum::Router;

/// Assembles all module routers under a versioned API prefix.
pub fn build_router(state: AppState) -> Router {
    Router::new()
        .nest("/api/v1/auth", auth_routes())
        .nest("/api/v1/users", user_routes())
    // .with_state(state)
}
