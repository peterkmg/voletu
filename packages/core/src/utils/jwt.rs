use anyhow::anyhow;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chrono::{Duration, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
  pub sub: String,
  pub uid: Uuid,
  pub role: String,
  pub iat: usize,
  pub exp: usize,
}

pub fn generate_secret() -> String {
  let mut bytes = [0_u8; 48];
  let mut rng = rand::rng();
  rng.fill_bytes(&mut bytes);
  URL_SAFE_NO_PAD.encode(bytes)
}

impl Claims {
  pub fn new(
    uid: Uuid,
    sub: String,
    role: String,
    expiration_seconds: i64,
  ) -> anyhow::Result<Self> {
    let expiration =
      Duration::try_seconds(expiration_seconds).ok_or(anyhow!("Invalid expiration time"))?;
    let exp = Utc::now()
      .checked_add_signed(expiration)
      .ok_or(anyhow!("Invalid expiration time"))?
      .timestamp() as usize;

    Ok(Self {
      sub,
      uid,
      role,
      iat: Utc::now().timestamp() as usize,
      exp,
    })
  }

  pub fn encode(&self, secret: &str) -> anyhow::Result<String> {
    Ok(jsonwebtoken::encode(
      &jsonwebtoken::Header::default(),
      self,
      &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
    )?)
  }

  pub fn decode(token: &str, secret: &str) -> anyhow::Result<Self> {
    Ok(
      jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
      )?
      .claims,
    )
  }
}
