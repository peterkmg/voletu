use std::net::SocketAddr;
use std::path::Path;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::Rng;
use serde::{Deserialize, Serialize};
use voletu_core::{AppConfig, DatabaseConfig, DatabaseType, JwtConfig};

const CONFY_APP_NAME: &str = "voletu";
const CONFY_CONFIG_NAME: &str = "desktop";
const KEYRING_SERVICE: &str = "dev.peterkmg.voletu";
const KEYRING_ACCOUNT_JWT_SECRET: &str = "jwt_secret";
const KEYRING_ACCOUNT_DB_PASSWORD: &str = "db_password";

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PlainDatabaseConfig {
  db_type: DatabaseType,
  sqlite_file: Option<String>,
  host: Option<String>,
  port: Option<u16>,
  database: Option<String>,
  username: Option<String>,
}

impl Default for PlainDatabaseConfig {
  fn default() -> Self {
    Self {
      db_type: DatabaseType::SQLite,
      sqlite_file: None,
      host: None,
      port: None,
      database: None,
      username: None,
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct DesktopBootstrapConfig {
  pub initialized: bool,
  pub api_address: String,
  db: PlainDatabaseConfig,
  jwt_expiration_seconds: i64,
  jwt_refresh_expiration_seconds: i64,
}

impl Default for DesktopBootstrapConfig {
  fn default() -> Self {
    Self {
      initialized: false,
      api_address: "127.0.0.1:3030".to_string(),
      db: PlainDatabaseConfig::default(),
      jwt_expiration_seconds: 3600,
      jwt_refresh_expiration_seconds: 86400,
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SetupDatabaseType {
  Sqlite,
  Postgres,
  Mysql,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InitializeAppRequest {
  pub db_type: SetupDatabaseType,
  pub sqlite_file: Option<String>,
  pub sqlite_password: Option<String>,
  pub host: Option<String>,
  pub port: Option<u16>,
  pub database: Option<String>,
  pub username: Option<String>,
  pub password: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StartupStage {
  Setup,
  Superadmin,
  Login,
}

#[derive(Clone, Debug, Serialize)]
pub struct StartupState {
  pub stage: StartupStage,
  pub api_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub reason: Option<String>,
}

pub fn load_bootstrap_config() -> Result<DesktopBootstrapConfig, String> {
  confy::load(CONFY_APP_NAME, CONFY_CONFIG_NAME).map_err(|err| err.to_string())
}

fn store_bootstrap_config(config: &DesktopBootstrapConfig) -> Result<(), String> {
  confy::store(CONFY_APP_NAME, CONFY_CONFIG_NAME, config).map_err(|err| err.to_string())
}

fn keyring_entry(account: &str) -> Result<keyring::Entry, keyring::Error> {
  keyring::Entry::new(KEYRING_SERVICE, account)
}

fn keyring_set(account: &str, value: &str) -> Result<(), String> {
  let entry = keyring_entry(account).map_err(|err| err.to_string())?;
  entry.set_password(value).map_err(|err| err.to_string())
}

fn keyring_get(account: &str) -> Result<String, keyring::Error> {
  let entry = keyring_entry(account)?;
  entry.get_password()
}

fn keyring_verify_present(account: &str) -> Result<(), String> {
  keyring_get_or_string(account).map(|_| ())
}

fn keyring_get_or_string(account: &str) -> Result<String, String> {
  keyring_get(account).map_err(|err| match err {
    keyring::Error::NoEntry => {
      "No matching entry found in secure storage. Please run setup again.".to_string()
    }
    _ => err.to_string(),
  })
}

fn keyring_delete(account: &str) {
  let Ok(entry) = keyring_entry(account) else {
    return;
  };
  let _ = entry.delete_credential();
}

fn generate_jwt_secret() -> String {
  let mut bytes = [0_u8; 48];
  let mut rng = rand::rng();
  rng.fill_bytes(&mut bytes);
  URL_SAFE_NO_PAD.encode(bytes)
}

pub fn secrets_present() -> bool {
  keyring_get(KEYRING_ACCOUNT_JWT_SECRET).is_ok()
    && keyring_get(KEYRING_ACCOUNT_DB_PASSWORD).is_ok()
}

pub fn sanitize_initialization_state() -> Result<DesktopBootstrapConfig, String> {
  let bootstrap = load_bootstrap_config().unwrap_or_default();

  if bootstrap.initialized && !secrets_present() {
    tracing::warn!("Initialization flag is set but secure secrets are missing");
  }

  Ok(bootstrap)
}

pub fn startup_prerequisites_ready() -> Result<bool, String> {
  let bootstrap = sanitize_initialization_state()?;
  if !bootstrap.initialized || !secrets_present() {
    return Ok(false);
  }

  if matches!(bootstrap.db.db_type, DatabaseType::SQLite) {
    let sqlite_file = match bootstrap.db.sqlite_file.as_deref() {
      Some(value) if !value.trim().is_empty() => value,
      _ => return Ok(false),
    };

    if !Path::new(sqlite_file).exists() {
      return Ok(false);
    }
  }

  Ok(true)
}

pub fn compose_runtime_config() -> Result<AppConfig, String> {
  let plain = sanitize_initialization_state()?;
  if !plain.initialized {
    return Err("Application is not initialized".to_string());
  }

  let address: SocketAddr = plain
    .api_address
    .parse()
    .map_err(|err| format!("Invalid API address '{}': {err}", plain.api_address))?;

  let jwt_secret = keyring_get_or_string(KEYRING_ACCOUNT_JWT_SECRET)?;
  let db_password = keyring_get_or_string(KEYRING_ACCOUNT_DB_PASSWORD)?;

  let db = match plain.db.db_type {
    DatabaseType::SQLite => {
      let file = plain
        .db
        .sqlite_file
        .clone()
        .ok_or_else(|| "SQLite file path is not configured".to_string())?;
      DatabaseConfig::sqlite(&file, db_password)
    }
    DatabaseType::Postgres | DatabaseType::MySQL => DatabaseConfig {
      db_type: plain.db.db_type.clone(),
      file: None,
      host: plain.db.host.clone(),
      port: plain.db.port,
      database: plain.db.database.clone(),
      username: plain.db.username.clone(),
      password: db_password,
    },
  };

  Ok(AppConfig {
    address,
    jwt: JwtConfig {
      secret: jwt_secret,
      expiration_seconds: plain.jwt_expiration_seconds,
      refresh_expiration_seconds: plain.jwt_refresh_expiration_seconds,
    },
    db,
  })
}

fn validate_initialize_request(request: &InitializeAppRequest) -> Result<(), String> {
  match request.db_type {
    SetupDatabaseType::Sqlite => {
      if request
        .sqlite_file
        .as_ref()
        .is_none_or(|value| value.trim().is_empty())
      {
        return Err("sqlite_file is required for SQLite".to_string());
      }
      if request
        .sqlite_password
        .as_ref()
        .is_none_or(|value| value.trim().is_empty())
      {
        return Err("sqlite_password is required for SQLite".to_string());
      }
    }
    SetupDatabaseType::Postgres | SetupDatabaseType::Mysql => {
      if request
        .host
        .as_ref()
        .is_none_or(|value| value.trim().is_empty())
      {
        return Err("host is required for external databases".to_string());
      }
      if request.port.is_none() {
        return Err("port is required for external databases".to_string());
      }
      if request
        .database
        .as_ref()
        .is_none_or(|value| value.trim().is_empty())
      {
        return Err("database is required for external databases".to_string());
      }
      if request
        .username
        .as_ref()
        .is_none_or(|value| value.trim().is_empty())
      {
        return Err("username is required for external databases".to_string());
      }
      if request
        .password
        .as_ref()
        .is_none_or(|value| value.trim().is_empty())
      {
        return Err("password is required for external databases".to_string());
      }
    }
  }

  Ok(())
}

pub fn initialize_from_request(request: InitializeAppRequest) -> Result<(), String> {
  validate_initialize_request(&request)?;

  let mut bootstrap = DesktopBootstrapConfig {
    initialized: true,
    ..DesktopBootstrapConfig::default()
  };

  let db_password = match request.db_type {
    SetupDatabaseType::Sqlite => {
      bootstrap.db = PlainDatabaseConfig {
        db_type: DatabaseType::SQLite,
        sqlite_file: request.sqlite_file,
        host: None,
        port: None,
        database: None,
        username: None,
      };
      request.sqlite_password.unwrap_or_default()
    }
    SetupDatabaseType::Postgres => {
      bootstrap.db = PlainDatabaseConfig {
        db_type: DatabaseType::Postgres,
        sqlite_file: None,
        host: request.host,
        port: request.port,
        database: request.database,
        username: request.username,
      };
      request.password.unwrap_or_default()
    }
    SetupDatabaseType::Mysql => {
      bootstrap.db = PlainDatabaseConfig {
        db_type: DatabaseType::MySQL,
        sqlite_file: None,
        host: request.host,
        port: request.port,
        database: request.database,
        username: request.username,
      };
      request.password.unwrap_or_default()
    }
  };

  keyring_set(KEYRING_ACCOUNT_JWT_SECRET, &generate_jwt_secret())?;
  keyring_set(KEYRING_ACCOUNT_DB_PASSWORD, &db_password)?;

  keyring_verify_present(KEYRING_ACCOUNT_JWT_SECRET)
    .inspect_err(|_| {
      keyring_delete(KEYRING_ACCOUNT_JWT_SECRET);
      keyring_delete(KEYRING_ACCOUNT_DB_PASSWORD);
    })
    .map_err(|err| format!("Failed to verify JWT secret in secure storage: {err}"))?;

  keyring_verify_present(KEYRING_ACCOUNT_DB_PASSWORD)
    .inspect_err(|_| {
      keyring_delete(KEYRING_ACCOUNT_JWT_SECRET);
      keyring_delete(KEYRING_ACCOUNT_DB_PASSWORD);
    })
    .map_err(|err| format!("Failed to verify database secret in secure storage: {err}"))?;

  store_bootstrap_config(&bootstrap)
}

pub fn reset_initialization() -> Result<(), String> {
  keyring_delete(KEYRING_ACCOUNT_JWT_SECRET);
  keyring_delete(KEYRING_ACCOUNT_DB_PASSWORD);
  store_bootstrap_config(&DesktopBootstrapConfig::default())
}
