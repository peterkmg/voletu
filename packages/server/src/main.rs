mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let (host, port, db_cfg, jwt_cfg, logging_cfg) = config::load_config_from_env();
  voletu_core::logging::init_logging(&logging_cfg)?;

  tracing::info!("Starting voletu server...");

  let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
  tokio::spawn(async move {
    wait_for_shutdown_signal().await;
    let _ = shutdown_tx.send(());
  });

  voletu_core::serve_api_with_shutdown(host, port, db_cfg, jwt_cfg, Some(shutdown_rx)).await?;
  Ok(())
}

async fn wait_for_shutdown_signal() {
  #[cfg(unix)]
  {
    use tokio::signal::unix::{signal, SignalKind};

    let mut terminate =
      signal(SignalKind::terminate()).expect("failed to register SIGTERM handler");
    tokio::select! {
      _ = tokio::signal::ctrl_c() => {},
      _ = terminate.recv() => {},
    }
  }

  #[cfg(not(unix))]
  {
    let _ = tokio::signal::ctrl_c().await;
  }
}
