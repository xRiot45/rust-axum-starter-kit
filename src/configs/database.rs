use sqlx::{postgres::PgPoolOptions, PgPool};
use crate::configs::app_config::DatabaseConfig;

pub async fn create_pool(cfg: &DatabaseConfig) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(cfg.max_connections)
        .connect(&cfg.url)
        .await?;

    tracing::info!("Database connection pool established (max={})", cfg.max_connections);
    Ok(pool)
}
