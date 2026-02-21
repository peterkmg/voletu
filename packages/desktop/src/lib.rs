mod bootstrap;
mod commands;
mod logging;
mod server;

use std::sync::Arc;

use tokio::sync::Mutex;

use crate::server::{ServerRuntime, SharedServerRuntime};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let server_runtime: SharedServerRuntime = Arc::new(Mutex::new(ServerRuntime::default()));

  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .manage(server_runtime.clone())
    .invoke_handler(tauri::generate_handler![
      commands::resolve_startup_state,
      commands::initialize_app,
      commands::reset_app_initialization
    ])
    .setup(move |app| {
      logging::init_logging(app)?;

      tracing::info!("=== Voletu Desktop Starting ===");

      tracing::info!("Desktop runtime ready, frontend will resolve startup state");

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
