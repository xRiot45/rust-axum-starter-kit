use axum::Router;
use tower_http::{
    compression::CompressionLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use std::time::Duration;

use crate::bootstrap::router::build_router;
use crate::bootstrap::state::AppState;
use crate::common::middlewares::cors::cors_layer;
use crate::configs::app_config::AppConfig;
use crate::configs::database::create_pool;
use crate::configs::jwt::JwtKeys;
use crate::configs::redis::create_connection_manager;

/// Creates and configures the full Axum application.
pub async fn create_app(config: &AppConfig) -> anyhow::Result<Router> {
    let db = create_pool(&config.database).await?;
    let redis = create_connection_manager(&config.redis).await?;
    let jwt = JwtKeys::new(&config.jwt);

    let state = AppState {
        config: config.clone(),
        db,
        redis,
        jwt,
    };

    let router = build_router(state)
        .layer(cors_layer())
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(TraceLayer::new_for_http());

    Ok(router)
}
