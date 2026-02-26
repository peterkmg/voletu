use anyhow::anyhow;
use tauri::AppHandle;

use crate::{
  config::{AppConfig, AppMode},
  keyring::load_db_password,
  local_api::start_local_mode,
  state::AppState,
};

pub fn initialize_state(app: &AppHandle) -> anyhow::Result<AppState> {
  let cfg = AppConfig::load()?;
  let db_password = load_db_password()?;
  let mut state = AppState::new(cfg.clone());

  match cfg.resolved_mode(db_password.is_some()) {
    Some(AppMode::Remote) => {
      state.startup.needs_setup = false;
      state.startup.mode = Some(AppMode::Remote);
      state.startup.api_base_url = cfg.remote_api_url.clone();
      Ok(state)
    }
    Some(AppMode::Local) => {
      let db_params = cfg
        .db_params
        .clone()
        .ok_or_else(|| anyhow!("local db params missing"))?;
      let jwt_cfg = cfg
        .jwt_config
        .clone()
        .ok_or_else(|| anyhow!("local jwt config missing"))?;
      let password = db_password.ok_or_else(|| anyhow!("local db password missing"))?;
      let base_url = start_local_mode(app, &mut state, db_params, jwt_cfg, &password)?;
      state.startup.needs_setup = false;
      state.startup.mode = Some(AppMode::Local);
      state.startup.api_base_url = Some(base_url);
      Ok(state)
    }
    None => Ok(state),
  }
}
