use tauri::Manager;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init_logging(app: &tauri::App) -> tauri::Result<()> {
  let app_data_dir = app.path().app_data_dir()?;

  std::fs::create_dir_all(&app_data_dir)
    .map_err(|err| tauri::Error::Anyhow(anyhow::anyhow!(err.to_string())))?;
  let log_dir = app_data_dir.join("logs");
  std::fs::create_dir_all(&log_dir)
    .map_err(|err| tauri::Error::Anyhow(anyhow::anyhow!(err.to_string())))?;

  let file_appender = tracing_appender::rolling::daily(&log_dir, "voletu.log");
  let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

  tracing_subscriber::registry()
    .with(
      EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,voletu_core=debug,voletu_desktop=debug")),
    )
    .with(fmt::layer().with_writer(std::io::stdout).with_ansi(true))
    .with(fmt::layer().with_writer(non_blocking).with_ansi(false))
    .init();

  Ok(())
}
