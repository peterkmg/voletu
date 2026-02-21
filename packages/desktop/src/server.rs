use std::sync::Arc;

use tokio::sync::{oneshot, Mutex};
use voletu_core::{init_api, AppConfig};

use crate::bootstrap;

#[derive(Default)]
pub struct ServerRuntime {
  task: Option<tauri::async_runtime::JoinHandle<()>>,
  running: bool,
}

pub type SharedServerRuntime = Arc<Mutex<ServerRuntime>>;

pub async fn start_server_if_needed(server_runtime: &SharedServerRuntime) -> Result<(), String> {
  {
    let runtime = server_runtime.lock().await;
    if runtime.running {
      return Ok(());
    }
  }

  let cfg = bootstrap::compose_runtime_config()?;
  let api_url = format!("http://{}", cfg.address);

  let (ready_tx, ready_rx) = oneshot::channel();
  let task = tauri::async_runtime::spawn(async move {
    if let Err(err) = start_server(cfg, ready_tx).await {
      tracing::error!("Embedded server failed: {err:#}");
    }
  });

  {
    let mut runtime = server_runtime.lock().await;
    runtime.task = Some(task);
    runtime.running = true;
  }

  match tokio::time::timeout(std::time::Duration::from_secs(10), ready_rx).await {
    Ok(Ok(_)) => {
      tracing::info!("API server ready on {api_url}");
      Ok(())
    }
    Ok(Err(_)) => {
      stop_server(server_runtime).await;
      Err("Server startup failed".to_string())
    }
    Err(_) => {
      stop_server(server_runtime).await;
      Err("Server startup timeout".to_string())
    }
  }
}

pub async fn stop_server(server_runtime: &SharedServerRuntime) {
  let mut runtime = server_runtime.lock().await;
  if let Some(task) = runtime.task.take() {
    task.abort();
  }
  runtime.running = false;
}

async fn start_server(cfg: AppConfig, ready_tx: oneshot::Sender<()>) -> anyhow::Result<()> {
  tracing::info!("Starting API server on {}", cfg.address);

  let (router, _openapi) = init_api(cfg.clone()).await?.split_for_parts();

  let listener = tokio::net::TcpListener::bind(cfg.address).await?;
  let _ = ready_tx.send(());
  tracing::info!("Server listening on http://{}", cfg.address);

  axum::serve(listener, router).await?;
  Ok(())
}
