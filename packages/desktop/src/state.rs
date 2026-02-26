use serde::Serialize;
use tauri::async_runtime::JoinHandle;
use tokio::{
  sync::oneshot,
  time::{timeout, Duration},
};

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
  pub local_api_shutdown: Option<oneshot::Sender<()>>,
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
      local_api_shutdown: None,
      logging_initialized: false,
    }
  }

  pub async fn stop_local_api(&mut self) {
    if let Some(shutdown_tx) = self.local_api_shutdown.take() {
      let _ = shutdown_tx.send(());
    }

    if let Some(mut task) = self.local_api_task.take() {
      match timeout(Duration::from_secs(3), &mut task).await {
        Ok(_) => {}
        Err(_) => {
          tracing::warn!("local api shutdown timed out, aborting task");
          task.abort();
        }
      }
    }
  }
}
