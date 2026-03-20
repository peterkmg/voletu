use anyhow::{Ok, Result};
use uuid::Uuid;

use super::SystemService;
use crate::{
  api::ApiError,
  utils::jwt::{generate_secret, Claims},
};

impl SystemService {
  pub async fn access_create(&self, id: Uuid, name: &str, role: &str) -> Result<String> {
    let claims = Claims::new(
      id,
      name.to_string(),
      role.to_string(),
      self.cfg.jwt.expiration_seconds,
    )?;
    Ok(claims.encode(&self.cfg.node.jwt_secret)?)
  }

  pub async fn verify_access(&self, token: &str) -> Result<Claims, ApiError> {
    Claims::decode(token, &self.cfg.node.jwt_secret)
      .map_err(|_| ApiError::Unauthorized("Invalid access token".to_string()))
  }

  pub fn create_refresh_secret(&self) -> String {
    generate_secret()
  }

  pub fn refresh_expiration_seconds(&self) -> i64 {
    self.cfg.jwt.refresh_expiration_seconds
  }
}
