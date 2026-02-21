use std::{env, net::SocketAddr};

use voletu_core::{AppConfig, DatabaseConfig, DatabaseType, JwtConfig};

fn env_required(name: &str) -> String {
  env::var(name).unwrap_or_else(|_| panic!("Missing required environment variable: {name}"))
}

fn env_parse_or_default<T>(name: &str, default: T) -> T
where
  T: std::str::FromStr,
  <T as std::str::FromStr>::Err: std::fmt::Display,
{
  match env::var(name) {
    Ok(raw) => raw
      .parse::<T>()
      .unwrap_or_else(|err| panic!("Invalid value for {name}: {err}")),
    Err(_) => default,
  }
}

pub fn load_config_from_env() -> AppConfig {
  let _ = dotenvy::dotenv();

  let host = env_required("API_HOST");
  let port = env_required("API_PORT");

  let address: SocketAddr = format!("{}:{}", host, port)
    .parse()
    .expect("API_HOST and API_PORT must form a valid socket address");

  let jwt = JwtConfig {
    secret: env_required("JWT_SECRET"),
    expiration_seconds: env_parse_or_default("JWT_EXPIRATION_SECONDS", 28800_i64),
    refresh_expiration_seconds: env_parse_or_default("JWT_REFRESH_EXPIRATION_SECONDS", 604800_i64),
  };

  let db_type_raw = env_required("DB_TYPE");
  let db_type = match db_type_raw.to_ascii_lowercase().as_str() {
    "sqlite" => DatabaseType::SQLite,
    "postgres" | "postgresql" => DatabaseType::Postgres,
    "mysql" => DatabaseType::MySQL,
    _ => panic!(
      "Invalid DB_TYPE value '{}'. Expected sqlite|postgres|mysql",
      db_type_raw
    ),
  };

  let password = env_required("DB_PASSWORD");

  let db = match db_type {
    DatabaseType::SQLite => {
      let file = env_required("DB_FILE");
      DatabaseConfig::sqlite(&file, password)
    }
    DatabaseType::Postgres | DatabaseType::MySQL => {
      let default_port = if matches!(db_type, DatabaseType::Postgres) {
        5432_u16
      } else {
        3306_u16
      };

      DatabaseConfig {
        db_type,
        file: None,
        host: Some(env_required("DB_HOST")),
        port: Some(env_parse_or_default("DB_PORT", default_port)),
        database: Some(env_required("DB_NAME")),
        username: Some(env_required("DB_USER")),
        password,
      }
    }
  };

  AppConfig { address, jwt, db }
}
