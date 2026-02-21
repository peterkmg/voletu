use anyhow::{anyhow, Result};
use argon2::{
  password_hash::{rand_core::OsRng, SaltString},
  Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

pub async fn hash_password(password: &str) -> Result<String> {
  let salt = SaltString::generate(&mut OsRng);
  Ok(
    Argon2::default()
      .hash_password(password.as_bytes(), &salt)
      .map_err(|e| anyhow!("Password hashing failed: {}", e))?
      .to_string(),
  )
}

pub async fn verify_password(password: &str, hash: &str) -> Result<bool> {
  let parsed_hash = PasswordHash::new(hash).map_err(|e| anyhow!("Invalid password hash: {}", e))?;

  Ok(
    Argon2::default()
      .verify_password(password.as_bytes(), &parsed_hash)
      .is_ok(),
  )
}
