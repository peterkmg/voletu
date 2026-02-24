use serde::Serialize;
use tauri::async_runtime::JoinHandle;

use crate::config::{AppConfig, AppMode};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartupState {
  pub needs_setup: bool,
  pub mode: Option<AppMode>,
  pub api_base_url: Option<String>,
}

pub struct AppState {
  pub config: AppConfig,
  pub startup: StartupState,
  pub local_api_task: Option<JoinHandle<()>>,
  pub logging_initialized: bool,
}

impl AppState {
  pub fn new(config: AppConfig) -> Self {
    Self {
      config,
      startup: StartupState {
        needs_setup: true,
        mode: None,
        api_base_url: None,
      },
      local_api_task: None,
      logging_initialized: false,
    }
  }

  pub fn stop_local_api(&mut self) {
    if let Some(task) = self.local_api_task.take() {
      task.abort();
    }
  }
}
