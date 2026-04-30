use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, thiserror::Error)]
pub enum DbConfigError {
  #[error("SQLite file path is required")]
  MissingSqliteFile,
  #[error("SQLite path must be valid UTF-8")]
  InvalidSqlitePath,
  #[error("Host is required")]
  MissingHost,
  #[error("Port is required")]
  MissingPort,
  #[error("Database name is required")]
  MissingDatabase,
  #[error("Username is required")]
  MissingUsername,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum DatabaseType {
  #[default]
  SQLite,
  Postgres,
  MySQL,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DbParams {
  pub db_type: DatabaseType,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub file: Option<PathBuf>, // only for SQLite; must be an absolute path
  #[serde(default)]
  pub sqlite_in_memory: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sqlite_shared_memory_name: Option<String>,
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
      sqlite_in_memory: false,
      sqlite_shared_memory_name: None,
      host: None,
      port: None,
      database: None,
      username: None,
    }
  }

  pub fn sqlite_memory() -> Self {
    Self {
      db_type: DatabaseType::SQLite,
      file: None,
      sqlite_in_memory: true,
      sqlite_shared_memory_name: None,
      host: None,
      port: None,
      database: None,
      username: None,
    }
  }

  pub fn sqlite_shared_memory(name: impl Into<String>) -> Self {
    Self {
      db_type: DatabaseType::SQLite,
      file: None,
      sqlite_in_memory: false,
      sqlite_shared_memory_name: Some(name.into()),
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
      sqlite_in_memory: false,
      sqlite_shared_memory_name: None,
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

  pub fn connection_url(&self) -> Result<String, DbConfigError> {
    match self.params.db_type {
      DatabaseType::SQLite => {
        if self.params.sqlite_in_memory {
          return Ok("sqlite::memory:".to_string());
        }

        if let Some(name) = self.params.sqlite_shared_memory_name.as_deref() {
          return Ok(format!("sqlite:file:{name}?mode=memory&cache=shared"));
        }

        Ok(format!("sqlite://{}?mode=rwc", self.sqlite_path()?))
      }
      DatabaseType::Postgres | DatabaseType::MySQL => {
        let (scheme, host, port, database, username) = self.validated_external()?;
        Ok(format!(
          "{}://{}:{}@{}:{}/{}",
          scheme, username, self.password, host, port, database
        ))
      }
    }
  }

  pub fn connection_url_public(&self) -> Result<String, DbConfigError> {
    let url = self.connection_url()?;

    match self.params.db_type {
      DatabaseType::SQLite => {
        if self.params.sqlite_in_memory {
          return Ok(url);
        }

        Ok(
          url
            .split_once('?')
            .map_or(url.clone(), |(pre_query, _)| pre_query.to_string()),
        )
      }
      DatabaseType::Postgres | DatabaseType::MySQL => Ok(strip_sensitive(&url)),
    }
  }

  fn sqlite_path(&self) -> Result<&str, DbConfigError> {
    self
      .params
      .file
      .as_ref()
      .ok_or(DbConfigError::MissingSqliteFile)?
      .to_str()
      .ok_or(DbConfigError::InvalidSqlitePath)
  }

  fn validated_external(&self) -> Result<(String, &str, u16, &str, &str), DbConfigError> {
    let host = self
      .params
      .host
      .as_deref()
      .ok_or(DbConfigError::MissingHost)?;

    let port = self.params.port.ok_or(DbConfigError::MissingPort)?;

    let database = self
      .params
      .database
      .as_deref()
      .ok_or(DbConfigError::MissingDatabase)?;

    let username = self
      .params
      .username
      .as_deref()
      .ok_or(DbConfigError::MissingUsername)?;

    let scheme = self.params.db_type.to_string();

    Ok((scheme, host, port, database, username))
  }
}

fn strip_sensitive(url: &str) -> String {
  let Some((scheme, rest)) = url.split_once("://") else {
    return url.to_string();
  };

  let Some((_, without_userinfo)) = rest.split_once('@') else {
    return url.to_string();
  };

  format!("{}://{}", scheme, without_userinfo)
}
