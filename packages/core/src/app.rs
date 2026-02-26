use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use tokio::{net::TcpListener, sync::oneshot};
use tracing::info;

use crate::{
  api::{router::build_router, ApiState},
  config::ApiConfig,
  database::init_database,
  DbConfig,
  JwtConfig,
};

pub async fn serve_api(
  host: String,
  port: String,
  db_cfg: DbConfig,
  jwt_cfg: JwtConfig,
) -> Result<()> {
  serve_api_with_shutdown(host, port, db_cfg, jwt_cfg, None).await
}

pub async fn serve_api_with_shutdown(
  host: String,
  port: String,
  db_cfg: DbConfig,
  jwt_cfg: JwtConfig,
  shutdown_rx: Option<oneshot::Receiver<()>>,
) -> Result<()> {
  info!("Initializing database at: {}", db_cfg.connection_url());
  let (db, node_cfg) = init_database(&db_cfg).await?;

  let cfg = ApiConfig::new(node_cfg, jwt_cfg, db_cfg);

  info!("Initializing API state...");
  let state = Arc::new(ApiState::build(cfg, Arc::new(db)));

  info!("Building API routes...");
  let router = build_router(state);

  let address: SocketAddr = format!("{}:{}", host, port).parse()?;
  let listener = TcpListener::bind(address).await?;
  info!("API server listening on http://{address}");

  if let Some(rx) = shutdown_rx {
    axum::serve(listener, router)
      .with_graceful_shutdown(async move {
        let _ = rx.await;
      })
      .await?;
  } else {
    axum::serve(listener, router).await?;
  }

  Ok(())
}
