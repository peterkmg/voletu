mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let (host, port, db_cfg, jwt_cfg, logging_cfg) = config::load_config_from_env();
  voletu_core::logging::init_logging(&logging_cfg)?;

  tracing::info!("Starting voletu server...");

  voletu_core::serve_api(host, port, db_cfg, jwt_cfg).await?;
  Ok(())
}
