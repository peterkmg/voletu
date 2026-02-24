use anyhow::{Ok, Result};
use uuid::Uuid;

use crate::{config::ApiConfig, utils::jwt::Claims};

pub struct TokenService {
  secret: String,
  expiration_seconds: i64,
}

impl TokenService {
  pub fn new(cfg: &ApiConfig) -> Self {
    Self {
      secret: cfg.node.jwt_secret.clone(),
      expiration_seconds: cfg.jwt.expiration_seconds,
    }
  }

  pub async fn create_access(&self, id: Uuid, name: &str, role: &str) -> Result<String> {
    let claims = Claims::new(
      id,
      name.to_string(),
      role.to_string(),
      self.expiration_seconds,
    )?;
    Ok(claims.encode(&self.secret)?)
  }

  pub async fn verify_access(&self, token: &str) -> Result<Claims> {
    Ok(Claims::decode(token, &self.secret)?)
  }
}
