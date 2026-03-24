use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JwtConfig {
  pub expiration_seconds: i64,
  pub refresh_expiration_seconds: i64,
}

const MAX_EXPIRATION: i64 = 2_592_000; // 30 days

impl JwtConfig {
  pub fn new(expiration_seconds: i64, refresh_expiration_seconds: i64) -> Self {
    assert!(
      expiration_seconds > 0,
      "JWT expiration_seconds must be positive, got {expiration_seconds}"
    );
    assert!(
      refresh_expiration_seconds > 0,
      "JWT refresh_expiration_seconds must be positive, got {refresh_expiration_seconds}"
    );
    assert!(
      refresh_expiration_seconds > expiration_seconds,
      "JWT refresh_expiration_seconds ({refresh_expiration_seconds}) must be greater than expiration_seconds ({expiration_seconds})"
    );
    assert!(
      expiration_seconds <= MAX_EXPIRATION,
      "JWT expiration_seconds ({expiration_seconds}) exceeds maximum of {MAX_EXPIRATION} seconds (30 days)"
    );
    assert!(
      refresh_expiration_seconds <= MAX_EXPIRATION,
      "JWT refresh_expiration_seconds ({refresh_expiration_seconds}) exceeds maximum of {MAX_EXPIRATION} seconds (30 days)"
    );
    Self {
      expiration_seconds,
      refresh_expiration_seconds,
    }
  }
}

impl Default for JwtConfig {
  fn default() -> Self {
    Self {
      expiration_seconds: 28800,
      refresh_expiration_seconds: 604800,
    }
  }
}
