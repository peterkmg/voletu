use std::path::PathBuf;

use anyhow::{anyhow, Context};
use tauri::{async_runtime::JoinHandle, AppHandle, Manager};
use tokio::sync::oneshot;
use voletu_core::{
  ensure_dir,
  ensure_parent_dir,
  logging::init_logging,
  resolve_relative,
  DatabaseType,
  DbConfig,
  DbParams,
  JwtConfig,
  LoggingConfig,
};

use crate::{
  config::{AppConfig, AppMode},
  constants::{DEFAULT_LOCAL_HOST, DEFAULT_LOCAL_PORT, DEFAULT_LOG_FILTER},
  state::AppState,
};

pub fn app_data_dir(app: &AppHandle) -> anyhow::Result<PathBuf> {
  app
    .path()
    .app_data_dir()
    .map_err(|e| anyhow!("failed to resolve app data dir: {e}"))
}

pub fn local_api_base_url() -> String {
  format!("http://{}:{}", DEFAULT_LOCAL_HOST, DEFAULT_LOCAL_PORT)
}

fn normalize_db_params(app: &AppHandle, db_params: &mut DbParams) -> anyhow::Result<()> {
  let data_dir = app_data_dir(app)?;
  ensure_dir(&data_dir)?;

  if matches!(db_params.db_type, DatabaseType::SQLite) {
    let sqlite_file = db_params
      .file
      .as_ref()
      .ok_or_else(|| anyhow!("sqlite file path is required for sqlite mode"))?;
    let resolved = resolve_relative(sqlite_file, &data_dir);
    ensure_parent_dir(&resolved)?;
    db_params.file = Some(resolved);
  }
  Ok(())
}

fn default_log_file(app: &AppHandle) -> anyhow::Result<std::path::PathBuf> {
  Ok(app_data_dir(app)?.join("logs").join("voletu.log"))
}

fn spawn_local_api_task(
  db_params: DbParams,
  jwt_cfg: JwtConfig,
  db_password: String,
  shutdown_rx: oneshot::Receiver<()>,
) -> JoinHandle<()> {
  let db_cfg = DbConfig::new(db_params, db_password);
  let host = DEFAULT_LOCAL_HOST.to_string();
  let port = DEFAULT_LOCAL_PORT.to_string();

  tauri::async_runtime::spawn(async move {
    if let Err(err) =
      voletu_core::serve_api_with_shutdown(host, port, db_cfg, jwt_cfg, Some(shutdown_rx)).await
    {
      tracing::error!("local api exited with error: {err:#}");
    }
  })
}

pub fn start_local_mode(
  app: &AppHandle,
  state: &mut AppState,
  db_params: DbParams,
  jwt_cfg: JwtConfig,
  db_password: &str,
) -> anyhow::Result<String> {
  let mut normalized_db_params = db_params;
  normalize_db_params(app, &mut normalized_db_params)?;

  if !state.logging_initialized {
    let log_file = default_log_file(app)?;
    ensure_parent_dir(&log_file)?;
    let logging_cfg = LoggingConfig::new(DEFAULT_LOG_FILTER.to_string(), log_file);
    init_logging(&logging_cfg).context("failed to initialize logging")?;
    state.logging_initialized = true;
  }

  let (shutdown_tx, shutdown_rx) = oneshot::channel();
  let task = spawn_local_api_task(
    normalized_db_params,
    jwt_cfg,
    db_password.to_string(),
    shutdown_rx,
  );
  state.local_api_shutdown = Some(shutdown_tx);
  state.local_api_task = Some(task);
  Ok(local_api_base_url())
}

pub fn compute_startup_state(
  cfg: &AppConfig,
  db_password_present: bool,
) -> (bool, Option<AppMode>, Option<String>) {
  match cfg.resolved_mode(db_password_present) {
    Some(AppMode::Remote) => (false, Some(AppMode::Remote), cfg.remote_api_url.clone()),
    Some(AppMode::Local) => (false, Some(AppMode::Local), Some(local_api_base_url())),
    None => (true, None, None),
  }
}
