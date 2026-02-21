use anyhow::{Ok, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::JwtConfig;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
  pub sub: String,
  pub uid: Uuid,
  pub role: String,
  pub iat: usize,
  pub exp: usize,
}

pub struct JwtService {
  secret: String,
  expiration_seconds: i64,
}

impl JwtService {
  pub fn new(cfg: &JwtConfig) -> Self {
    Self {
      secret: cfg.secret.clone(),
      expiration_seconds: cfg.expiration_seconds,
    }
  }

  pub async fn create_jwt(&self, id: Uuid, name: &str, role: &str) -> Result<String> {
    let exp = Utc::now()
      .checked_add_signed(Duration::seconds(self.expiration_seconds))
      .expect("valid timestamp")
      .timestamp() as usize;

    let claims = Claims {
      sub: name.to_string(),
      uid: id,
      role: role.to_string(),
      iat: Utc::now().timestamp() as usize,
      exp,
    };

    Ok(encode(
      &Header::default(),
      &claims,
      &EncodingKey::from_secret(self.secret.as_ref()),
    )?)
  }

  pub async fn verify_jwt(&self, token: &str) -> Result<Claims> {
    Ok(
      decode::<Claims>(
        token,
        &DecodingKey::from_secret(self.secret.as_ref()),
        &jsonwebtoken::Validation::default(),
      )?
      .claims,
    )
  }
}
