use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JwtConfig {
  pub expiration_seconds: i64,
  pub refresh_expiration_seconds: i64,
}

impl JwtConfig {
  pub fn new(expiration_seconds: i64, refresh_expiration_seconds: i64) -> Self {
    Self {
      expiration_seconds,
      refresh_expiration_seconds,
    }
  }
}
