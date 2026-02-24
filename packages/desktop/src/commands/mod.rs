mod dto;

pub use dto::{SaveLocalConfigRequest, SaveRemoteConfigRequest};
use tauri::{AppHandle, State};
use tokio::sync::Mutex;

use crate::{
  config::AppMode,
  keyring::{clear_db_password, load_db_password, save_db_password},
  local_api::{compute_startup_state, local_api_base_url, start_local_mode},
  state::{AppState, StartupState},
};

type CmdResult<T> = Result<T, String>;

fn to_cmd_error(err: anyhow::Error) -> String {
  format!("{err:#}")
}

#[tauri::command]
pub async fn get_startup_state(state: State<'_, Mutex<AppState>>) -> CmdResult<StartupState> {
  let state = state.lock().await;
  Ok(state.startup.clone())
}

#[tauri::command]
pub async fn save_remote_config(
  state: State<'_, Mutex<AppState>>,
  req: SaveRemoteConfigRequest,
) -> CmdResult<StartupState> {
  let remote_url = req.remote_api_url.trim().to_string();
  if remote_url.is_empty() {
    return Err("remoteApiUrl cannot be empty".to_string());
  }

  let mut state = state.lock().await;
  let mut cfg = state.config.clone();
  cfg.mode = Some(AppMode::Remote);
  cfg.remote_api_url = Some(remote_url.clone());
  cfg.db_params = None;
  cfg.jwt_config = None;
  cfg.save().map_err(to_cmd_error)?;
  state.config = cfg;

  state.stop_local_api();
  state.startup.needs_setup = false;
  state.startup.mode = Some(AppMode::Remote);
  state.startup.api_base_url = Some(remote_url);
  Ok(state.startup.clone())
}

#[tauri::command]
pub async fn save_local_config(
  state: State<'_, Mutex<AppState>>,
  req: SaveLocalConfigRequest,
) -> CmdResult<StartupState> {
  if req.db_password.trim().is_empty() {
    return Err("dbPassword cannot be empty".to_string());
  }

  let db_params = req.parse_db_params().map_err(to_cmd_error)?;
  let jwt_config = req.parse_jwt_config();

  let mut state = state.lock().await;
  let mut cfg = state.config.clone();
  cfg.mode = Some(AppMode::Local);
  cfg.remote_api_url = None;
  cfg.db_params = Some(db_params);
  cfg.jwt_config = Some(jwt_config);
  cfg.save().map_err(to_cmd_error)?;
  state.config = cfg;
  save_db_password(req.db_password.trim()).map_err(to_cmd_error)?;

  state.startup.mode = Some(AppMode::Local);
  state.startup.needs_setup = false;
  state.startup.api_base_url = Some(local_api_base_url());
  Ok(state.startup.clone())
}

#[tauri::command]
pub async fn start_local_api(
  app: AppHandle,
  state: State<'_, Mutex<AppState>>,
) -> CmdResult<StartupState> {
  let mut state = state.lock().await;
  let db_params = state
    .config
    .db_params
    .clone()
    .ok_or_else(|| "local db params are missing".to_string())?;
  let jwt_cfg = state
    .config
    .jwt_config
    .clone()
    .ok_or_else(|| "local jwt config is missing".to_string())?;
  let password = load_db_password()
    .map_err(to_cmd_error)?
    .ok_or_else(|| "db password is missing".to_string())?;

  let base_url =
    start_local_mode(&app, &mut state, db_params, jwt_cfg, &password).map_err(to_cmd_error)?;
  state.startup.mode = Some(AppMode::Local);
  state.startup.needs_setup = false;
  state.startup.api_base_url = Some(base_url);
  Ok(state.startup.clone())
}

#[tauri::command]
pub async fn reset_config_and_mode(state: State<'_, Mutex<AppState>>) -> CmdResult<StartupState> {
  let mut state = state.lock().await;
  let mut cfg = state.config.clone();
  cfg.reset();
  cfg.save().map_err(to_cmd_error)?;
  state.config = cfg.clone();
  clear_db_password().map_err(to_cmd_error)?;

  state.stop_local_api();
  let (needs_setup, mode, api_base_url) = compute_startup_state(&cfg, false);
  state.startup.needs_setup = needs_setup;
  state.startup.mode = mode;
  state.startup.api_base_url = api_base_url;
  Ok(state.startup.clone())
}
