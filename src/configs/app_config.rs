use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub env: String,

    pub server: ServerConfig,
    pub cors: CorsConfig,
    pub rate_limit: RateLimitConfig,

    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub redis: RedisConfig,

    pub smtp: SmtpConfig,
    pub storage: StorageConfig,

    pub third_party: ThirdPartyConfig,
}

/// ─── Server ────────────────────────────────────────────────────────────────
#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

/// ─── CORS ──────────────────────────────────────────────────────────────────
#[derive(Debug, Deserialize, Clone)]
pub struct CorsConfig {
    pub allowed_origins: String,
    pub allowed_methods: String,
    pub allowed_headers: String,
}

impl CorsConfig {
    pub fn origins(&self) -> Vec<&str> {
        self.allowed_origins.split(',').collect()
    }

    pub fn methods(&self) -> Vec<&str> {
        self.allowed_methods.split(',').collect()
    }

    pub fn headers(&self) -> Vec<&str> {
        self.allowed_headers.split(',').collect()
    }
}

/// ─── Rate Limiting ─────────────────────────────────────────────────────────
#[derive(Debug, Deserialize, Clone)]
pub struct RateLimitConfig {
    pub requests: u32,
    pub window_secs: u64,
}

/// ─── Database ──────────────────────────────────────────────────────────────
#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_secs: u64,
    pub log_slow_queries: bool,
}

/// ─── JWT ───────────────────────────────────────────────────────────────────
#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_expiry_secs: u64,
    pub refresh_token_expiry_secs: u64,
}

/// ─── Redis ─────────────────────────────────────────────────────────────────
#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32,
    pub ttl_default_secs: u64,
}

/// ─── SMTP ──────────────────────────────────────────────────────────────────
#[derive(Debug, Deserialize, Clone)]
pub struct SmtpConfig {
    pub from_name: String,
    pub from_email: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

/// ─── Storage ───────────────────────────────────────────────────────────────
#[derive(Debug, Deserialize, Clone)]
pub struct StorageConfig {
    pub driver: String,
    pub local_path: Option<String>,

    pub s3_bucket: Option<String>,
    pub s3_region: Option<String>,
}

/// ─── Third Party ───────────────────────────────────────────────────────────
#[derive(Debug, Deserialize, Clone)]
pub struct ThirdPartyConfig {
    pub api_url: String,
    pub api_key: String,
}

/// ─── Loader ────────────────────────────────────────────────────────────────
impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        let config = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            // ─── Defaults ───────────────────────────────────────────────
            .set_default("env", "development")?
            // Server
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?
            // Database
            .set_default("database.max_connections", 10)?
            .set_default("database.min_connections", 2)?
            .set_default("database.connect_timeout_secs", 5)?
            .set_default("database.log_slow_queries", true)?
            // JWT
            .set_default("jwt.access_token_expiry_secs", 900)?
            .set_default("jwt.refresh_token_expiry_secs", 2592000)?
            // Redis
            .set_default("redis.pool_size", 10)?
            .set_default("redis.ttl_default_secs", 3600)?
            // Rate limit
            .set_default("rate_limit.requests", 100)?
            .set_default("rate_limit.window_secs", 60)?
            .build()?;

        Ok(config.try_deserialize()?)
    }

    pub fn is_production(&self) -> bool {
        self.env == "production"
    }

    pub fn is_development(&self) -> bool {
        self.env == "development"
    }
}
