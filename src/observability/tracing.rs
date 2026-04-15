use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize structured logging using tracing-subscriber.
/// Set RUST_LOG env var to control log levels (e.g. RUST_LOG=info,sqlx=warn).
pub fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,sqlx=warn,hyper=warn"));

    // In production, use JSON format for log aggregators (e.g. Datadog, Loki, CloudWatch).
    // In development, use pretty-print format.
    let is_prod = std::env::var("ENV").as_deref() == Ok("production");

    if is_prod {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer().pretty())
            .init();
    }
}
