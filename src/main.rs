mod background;
mod bootstrap;
mod common;
mod configs;
mod modules;
mod observability;

use bootstrap::app::create_app;
use configs::app_config::AppConfig;
use observability::tracing::init_tracing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file (if present)
    dotenvy::dotenv().ok();

    // Initialize structured tracing
    init_tracing();

    // Load application configuration
    let config = AppConfig::load()?;

    tracing::info!(
        env = %config.env,
        port = %config.server.port,
        "Starting rust-axum-starter-kit"
    );

    // Build and run the Axum application
    let app = create_app(&config).await?;

    let listener = tokio::net::TcpListener::bind(
        format!("{}:{}", config.server.host, config.server.port)
    ).await?;

    tracing::info!("Listening on http://{}:{}", config.server.host, config.server.port);

    axum::serve(listener, app).await?;
    Ok(())
}
