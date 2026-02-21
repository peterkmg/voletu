use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init_logging() -> anyhow::Result<()> {
  let log_dir = std::path::Path::new("./logs");
  std::fs::create_dir_all(log_dir)?;

  let file_appender = tracing_appender::rolling::daily(log_dir, "voletu-server.log");
  let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

  tracing_subscriber::registry()
    .with(
      EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,voletu_core=debug,voletu_server=debug")),
    )
    .with(fmt::layer().with_writer(std::io::stdout).with_ansi(true))
    .with(fmt::layer().with_writer(non_blocking).with_ansi(false))
    .init();

  Ok(())
}
