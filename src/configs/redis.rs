use redis::{aio::ConnectionManager, Client};
use crate::configs::app_config::RedisConfig;

pub async fn create_connection_manager(cfg: &RedisConfig) -> anyhow::Result<ConnectionManager> {
    let client = Client::open(cfg.url.as_str())?;
    let manager = ConnectionManager::new(client).await?;
    tracing::info!("Redis connection manager ready");
    Ok(manager)
}
