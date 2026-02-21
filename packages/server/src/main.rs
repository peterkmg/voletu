mod config;
mod logging;

use voletu_core::init_api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  logging::init_logging()?;

  tracing::info!("=== Voletu Server Starting ===");

  let cfg = config::load_config_from_env();

  tracing::info!("Config: address={}", cfg.address);

  let (router, _openapi) = init_api(cfg.clone()).await?.split_for_parts();

  let listener = tokio::net::TcpListener::bind(cfg.address).await?;
  tracing::info!("Listening on http://{}", cfg.address);

  axum::serve(listener, router).await?;
  Ok(())
}
