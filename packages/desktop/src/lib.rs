mod commands;
mod config;
mod constants;
mod init;
mod keyring;
mod local_api;
mod state;

use state::AppState;
use tauri::{App, Manager};
use tokio::sync::Mutex;

use crate::init::initialize_state;

pub fn setup_tauri(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
  let state = match initialize_state(app.handle()) {
    Ok(state) => state,
    Err(err) => {
      tracing::error!("failed to initialize startup state, falling back to setup flow: {err:#}");
      AppState::new(config::AppConfig::default())
    }
  };
  app.manage(Mutex::new(state));
  Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> anyhow::Result<()> {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![
      commands::get_startup_state,
      commands::save_remote_config,
      commands::save_local_config,
      commands::start_local_api,
      commands::reset_config_and_mode
    ])
    .setup(setup_tauri)
    .run(tauri::generate_context!())
    .map_err(|e| e.into())
}
