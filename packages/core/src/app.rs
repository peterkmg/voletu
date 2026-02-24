use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

use crate::{
  api::{router::build_router, ApiState},
  config::ApiConfig,
  database::init_database,
  DbConfig, JwtConfig,
};

pub async fn init_api(db_cfg: DbConfig, jwt_cfg: JwtConfig) -> Result<Router> {
  info!("Initializing database at: {}", db_cfg.connection_url());
  let (db, node_cfg) = init_database(&db_cfg).await?;
  let cfg = ApiConfig::new(node_cfg, jwt_cfg, db_cfg);
  let state = Arc::new(ApiState::build(cfg, Arc::new(db)));
  Ok(build_router(state))
}

pub async fn serve(router: Router, host: String, port: String) -> Result<()> {
  let address: SocketAddr = format!("{}:{}", host, port).parse()?;
  let listener = TcpListener::bind(address).await?;
  info!("API server listening on http://{address}");
  axum::serve(listener, router).await?;
  Ok(())
}
