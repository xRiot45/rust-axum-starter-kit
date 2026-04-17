use crate::configs::app_config::JwtConfig;

/// Holds pre-loaded JWT config for use in handlers/services.
#[derive(Clone)]
pub struct JwtKeys {
    pub access_secret: String,
    pub refresh_secret: String,

    pub access_expiry_secs: u64,
    pub refresh_expiry_secs: u64,
}

impl JwtKeys {
    pub fn new(cfg: &JwtConfig) -> Self {
        Self {
            access_secret: cfg.access_token.clone(),
            refresh_secret: cfg.refresh_token.clone(),

            access_expiry_secs: cfg.access_token_expiry_secs,
            refresh_expiry_secs: cfg.refresh_token_expiry_secs,
        }
    }
}
