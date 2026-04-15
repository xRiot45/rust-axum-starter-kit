use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use crate::common::errors::{AppError, AppResult};

pub fn hash_password(plain: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(plain.as_bytes(), &salt)
        .map_err(|e| AppError::InternalServerError(anyhow::anyhow!("Hashing failed: {e}")))?
        .to_string();
    Ok(hash)
}

pub fn verify_password(plain: &str, hash: &str) -> AppResult<bool> {
    let parsed = PasswordHash::new(hash)
        .map_err(|e| AppError::InternalServerError(anyhow::anyhow!("Invalid hash: {e}")))?;
    Ok(Argon2::default().verify_password(plain.as_bytes(), &parsed).is_ok())
}
