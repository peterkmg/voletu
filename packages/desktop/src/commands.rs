use tauri::State;
use voletu_core::preflight_startup;

use crate::{
  bootstrap::{self, InitializeAppRequest, StartupStage, StartupState},
  server::{self, SharedServerRuntime},
};

#[tauri::command]
pub async fn resolve_startup_state(
  server_runtime: State<'_, SharedServerRuntime>,
) -> Result<StartupState, String> {
  let bootstrap = bootstrap::sanitize_initialization_state()?;
  let api_url = format!("http://{}", bootstrap.api_address);

  if !bootstrap::startup_prerequisites_ready()? {
    return Ok(StartupState {
      stage: StartupStage::Setup,
      api_url,
      reason: Some("Missing configuration, secure secrets, or SQLite database file".to_string()),
    });
  }

  let runtime_cfg = bootstrap::compose_runtime_config()?;
  let preflight = preflight_startup(&runtime_cfg)
    .await
    .map_err(|err| err.to_string())?;

  server::start_server_if_needed(&server_runtime).await?;

  if !preflight.has_superadmin_user {
    return Ok(StartupState {
      stage: StartupStage::Superadmin,
      api_url,
      reason: Some("No superadmin user exists".to_string()),
    });
  }

  Ok(StartupState {
    stage: StartupStage::Login,
    api_url,
    reason: None,
  })
}

#[tauri::command]
pub async fn initialize_app(
  server_runtime: State<'_, SharedServerRuntime>,
  request: InitializeAppRequest,
) -> Result<(), String> {
  bootstrap::initialize_from_request(request)?;
  server::start_server_if_needed(&server_runtime).await
}

#[tauri::command]
pub async fn reset_app_initialization(
  server_runtime: State<'_, SharedServerRuntime>,
) -> Result<(), String> {
  server::stop_server(&server_runtime).await;
  bootstrap::reset_initialization()
}
