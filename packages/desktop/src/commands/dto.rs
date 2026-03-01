use std::path::PathBuf;

use anyhow::anyhow;
use serde::Deserialize;
use voletu_core::{DatabaseType, DbParams, JwtConfig};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveRemoteConfigRequest {
  pub remote_api_url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveLocalConfigRequest {
  pub db_type: String,
  pub sqlite_file: Option<String>,
  pub host: Option<String>,
  pub port: Option<u16>,
  pub database: Option<String>,
  pub username: Option<String>,
  pub db_password: String,
  pub jwt_expiration_seconds: i64,
  pub jwt_refresh_expiration_seconds: i64,
}

impl SaveLocalConfigRequest {
  fn require_non_empty(value: Option<String>, field: &str) -> anyhow::Result<String> {
    let raw = value.ok_or_else(|| anyhow!("{field} is required"))?;
    let trimmed = raw.trim();
    if trimmed.is_empty() {
      return Err(anyhow!("{field} cannot be empty"));
    }
    Ok(trimmed.to_string())
  }

  pub fn parse_db_params(&self) -> anyhow::Result<DbParams> {
    let db_type = self
      .db_type
      .parse::<DatabaseType>()
      .map_err(anyhow::Error::msg)?;
    match db_type {
      DatabaseType::SQLite => {
        let file = Self::require_non_empty(self.sqlite_file.clone(), "sqliteFile")?;
        Ok(DbParams::sqlite(PathBuf::from(file)))
      }
      DatabaseType::Postgres | DatabaseType::MySQL => {
        let host = Self::require_non_empty(self.host.clone(), "host")?;
        let port = self.port.ok_or_else(|| anyhow!("port is required"))?;
        let database = Self::require_non_empty(self.database.clone(), "database")?;
        let username = Self::require_non_empty(self.username.clone(), "username")?;
        Ok(DbParams::external(db_type, host, port, database, username))
      }
    }
  }

  pub fn parse_jwt_config(&self) -> JwtConfig {
    JwtConfig::new(
      self.jwt_expiration_seconds,
      self.jwt_refresh_expiration_seconds,
    )
  }
}
