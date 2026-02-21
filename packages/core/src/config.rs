use std::net::SocketAddr;

use crate::database::DatabaseConfig;

#[derive(Clone)]
pub struct JwtConfig {
  pub secret: String,
  pub expiration_seconds: i64,
  pub refresh_expiration_seconds: i64,
}

#[derive(Clone)]
pub struct AppConfig {
  pub address: SocketAddr,
  pub jwt: JwtConfig,
  pub db: DatabaseConfig,
}
