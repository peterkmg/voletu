use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};
use tracing::log::LevelFilter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseType {
  SQLite,
  Postgres,
  MySQL,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
  pub db_type: DatabaseType,
  pub file: Option<String>, // sqlite
  pub host: Option<String>,
  pub port: Option<u16>,
  pub database: Option<String>,
  pub username: Option<String>,
  pub password: String,
}

impl DatabaseConfig {
  pub fn sqlite(filename: impl Into<String>, passwd: impl Into<String>) -> Self {
    Self {
      db_type: DatabaseType::SQLite,
      file: Some(filename.into()),
      host: None,
      port: None,
      database: None,
      username: None,
      password: passwd.into(),
    }
  }

  fn external(
    db_type: DatabaseType,
    host: impl Into<String>,
    port: u16,
    db: impl Into<String>,
    user: impl Into<String>,
    passwd: impl Into<String>,
  ) -> Self {
    Self {
      db_type,
      file: None,
      host: Some(host.into()),
      port: Some(port),
      database: Some(db.into()),
      username: Some(user.into()),
      password: passwd.into(),
    }
  }

  pub fn postgres(
    host: impl Into<String>,
    port: u16,
    db: impl Into<String>,
    user: impl Into<String>,
    passwd: impl Into<String>,
  ) -> Self {
    Self::external(DatabaseType::Postgres, host, port, db, user, passwd)
  }

  pub fn mysql(
    host: impl Into<String>,
    port: u16,
    db: impl Into<String>,
    user: impl Into<String>,
    passwd: impl Into<String>,
  ) -> Self {
    Self::external(DatabaseType::MySQL, host, port, db, user, passwd)
  }

  /// Build database connection URL
  pub fn connection_url(&self) -> String {
    match self.db_type {
      DatabaseType::SQLite => {
        let file = self
          .file
          .as_deref()
          .expect("SQLite configuration requires a file path");
        format!("sqlite://{}?mode=rwc", file)
      }
      DatabaseType::Postgres | DatabaseType::MySQL => {
        let host = self.host.as_deref().expect("Host is required");
        let port = self.port.expect("Port is required");
        let database = self.database.as_deref().expect("Database name is required");
        let username = self.username.as_deref().expect("Username is required");
        let password = self.password.as_str();
        let scheme = match self.db_type {
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

pub async fn init_database(cfg: &DatabaseConfig) -> Result<DatabaseConnection, DbErr> {
  tracing::info!("Initializing database...");

  let mut options = ConnectOptions::new(cfg.connection_url());
  options.sqlx_logging(true);
  options.sqlx_logging_level(LevelFilter::Trace);

  if let DatabaseType::SQLite = cfg.db_type {
    options.sqlcipher_key(cfg.password.clone());
  }

  Ok(
    Database::connect(options)
      .await
      .expect("Failed to connect to database"),
  )
}
