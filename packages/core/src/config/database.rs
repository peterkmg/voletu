use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum DatabaseType {
  #[default]
  SQLite,
  Postgres,
  MySQL,
}

impl DatabaseType {
  pub fn parse(raw: &str) -> Result<Self, String> {
    match raw.to_ascii_lowercase().as_str() {
      "sqlite" => Ok(Self::SQLite),
      "postgres" | "postgresql" => Ok(Self::Postgres),
      "mysql" => Ok(Self::MySQL),
      _ => Err(format!(
        "Invalid database type '{}'. Expected sqlite|postgres|mysql",
        raw
      )),
    }
  }
}

/// Serialisable subset of database configuration (no password).
/// Used by Tauri to persist non-sensitive settings via confy.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DbParams {
  pub db_type: DatabaseType,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub file: Option<PathBuf>, // only for SQLite; must be an absolute path
  #[serde(skip_serializing_if = "Option::is_none")]
  pub host: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub port: Option<u16>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub database: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub username: Option<String>,
}

impl DbParams {
  pub fn sqlite(file: impl Into<PathBuf>) -> Self {
    Self {
      db_type: DatabaseType::SQLite,
      file: Some(file.into()),
      host: None,
      port: None,
      database: None,
      username: None,
    }
  }

  pub fn external(
    db_type: DatabaseType,
    host: impl Into<String>,
    port: u16,
    database: impl Into<String>,
    username: impl Into<String>,
  ) -> Self {
    Self {
      db_type,
      file: None,
      host: Some(host.into()),
      port: Some(port),
      database: Some(database.into()),
      username: Some(username.into()),
    }
  }

  pub fn postgres(
    host: impl Into<String>,
    port: u16,
    database: impl Into<String>,
    username: impl Into<String>,
  ) -> Self {
    Self::external(DatabaseType::Postgres, host, port, database, username)
  }

  pub fn mysql(
    host: impl Into<String>,
    port: u16,
    database: impl Into<String>,
    username: impl Into<String>,
  ) -> Self {
    Self::external(DatabaseType::MySQL, host, port, database, username)
  }
}

#[derive(Debug, Clone)]
pub struct DbConfig {
  pub params: DbParams,
  pub password: String,
}

impl DbConfig {
  pub fn new(params: DbParams, password: impl Into<String>) -> Self {
    Self {
      params,
      password: password.into(),
    }
  }

  pub fn connection_url(&self) -> String {
    match self.params.db_type {
      DatabaseType::SQLite => {
        let file = self
          .params
          .file
          .as_ref()
          .expect("SQLite file path is required");
        format!(
          "sqlite://{}?mode=rwc",
          file.to_str().expect("SQLite path must be valid UTF-8")
        )
      }
      DatabaseType::Postgres | DatabaseType::MySQL => {
        let host = self.params.host.as_deref().expect("Host is required");
        let port = self.params.port.expect("Port is required");
        let database = self
          .params
          .database
          .as_deref()
          .expect("Database name is required");
        let username = self
          .params
          .username
          .as_deref()
          .expect("Username is required");
        let password = self.password.as_str();
        let scheme = match self.params.db_type {
          DatabaseType::Postgres => "postgres",
          DatabaseType::MySQL => "mysql",
          _ => unreachable!(),
        };
        format!(
          "{}://{}:{}@{}:{}/{}",
          scheme, username, password, host, port, database
        )
      }
    }
  }
}
