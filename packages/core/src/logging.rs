use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub use crate::config::LoggingConfig;
use crate::utils::paths::{ensure_parent_dir, split_file_path};

pub fn init_logging(cfg: &LoggingConfig) -> anyhow::Result<()> {
  if !cfg.enabled {
    return Ok(());
  }

  ensure_parent_dir(&cfg.log_file)?;
  let (log_directory, filename) = split_file_path(&cfg.log_file)?;

  let file_appender = tracing_appender::rolling::daily(log_directory, filename);
  let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

  std::mem::forget(guard); // keep the guard alive for the process lifetime.

  let env_filter =
    EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&cfg.filter));

  let registry = tracing_subscriber::registry().with(env_filter);

  #[cfg(debug_assertions)]
  let registry = registry.with(fmt::layer().with_writer(std::io::stdout).with_ansi(true));

  registry
    .with(fmt::layer().with_writer(non_blocking).with_ansi(false))
    .init();

  Ok(())
}
