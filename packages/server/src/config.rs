use std::{env, path::PathBuf};

use voletu_core::{DatabaseType, DbConfig, DbParams, JwtConfig, LoggingConfig};

fn env_required(name: &str) -> String {
  env::var(name).unwrap_or_else(|_| panic!("Missing required environment variable: {name}"))
}

fn env_parse_required<T>(name: &str) -> T
where
  T: std::str::FromStr,
  <T as std::str::FromStr>::Err: std::fmt::Display,
{
  let raw = env_required(name);
  raw
    .parse::<T>()
    .unwrap_or_else(|err| panic!("Invalid value for {name}: {err}"))
}

pub fn load_config_from_env() -> (String, String, DbConfig, JwtConfig, LoggingConfig) {
  let _ = dotenvy::dotenv();

  let host = env_required("API_HOST");
  let port = env_required("API_PORT");
  let _ = port
    .parse::<u16>()
    .unwrap_or_else(|_| panic!("API_PORT must be a valid port number"));

  let jwt = JwtConfig::new(
    env_parse_required("JWT_EXPIRATION_SECONDS"),
    env_parse_required("JWT_REFRESH_EXPIRATION_SECONDS"),
  );

  let db_type = env_required("DB_TYPE")
    .parse::<DatabaseType>()
    .unwrap_or_else(|err| panic!("Invalid value for DB_TYPE: {err}"));
  let db_password = env_required("DB_PASSWORD");

  let db = match db_type {
    DatabaseType::SQLite => DbConfig::new(DbParams::sqlite(env_required("DB_FILE")), db_password),
    DatabaseType::Postgres => DbConfig::new(
      DbParams::postgres(
        env_required("DB_HOST"),
        env_parse_required("DB_PORT"),
        env_required("DB_NAME"),
        env_required("DB_USER"),
      ),
      db_password,
    ),
    DatabaseType::MySQL => DbConfig::new(
      DbParams::mysql(
        env_required("DB_HOST"),
        env_parse_required("DB_PORT"),
        env_required("DB_NAME"),
        env_required("DB_USER"),
      ),
      db_password,
    ),
  };

  let logging = LoggingConfig::new(
    env_required("LOG_FILTER"),
    PathBuf::from(env_required("LOG_FILE")),
  );

  (host, port, db, jwt, logging)
}
