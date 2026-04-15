use axum::{
    middleware,
    routing::{delete, get, patch, post},
    Router,
};
use crate::modules::users::presentation::handlers::{
    create_user, delete_user, get_user, list_users, update_user,
};

pub fn user_routes() -> Router {
    Router::new()
        .route("/", post(create_user).get(list_users))
        .route("/:id", get(get_user).patch(update_user).delete(delete_user))
    // TODO: protect private routes:
    // .route_layer(middleware::from_fn(require_auth))
}
