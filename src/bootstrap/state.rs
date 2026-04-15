use sqlx::PgPool;
use redis::aio::ConnectionManager as RedisConn;
use crate::configs::app_config::AppConfig;
use crate::configs::jwt::JwtKeys;

/// Shared application state injected into every Axum handler via `State<AppState>`.
#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: PgPool,
    pub redis: RedisConn,
    pub jwt: JwtKeys,
}
