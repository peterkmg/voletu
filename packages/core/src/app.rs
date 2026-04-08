use std::{
  net::SocketAddr,
  sync::{Arc, Mutex},
  time::Duration,
};

use anyhow::Result;
use sea_orm::EntityTrait;
use tokio::{
  net::TcpListener,
  sync::{oneshot, watch},
};
use tracing::info;

use crate::{
  api::{router::build_router, ApiState},
  config::{ApiConfig, SyncConfig},
  db::init_database,
  entities::local,
  worker::{spawn_sync_worker, spawn_sync_worker_with_config, WorkerStatus},
  DbConfig,
  JwtConfig,
};

pub async fn serve_api(
  host: String,
  port: String,
  db_cfg: DbConfig,
  jwt_cfg: JwtConfig,
  shutdown_rx: oneshot::Receiver<()>,
) -> Result<()> {
  serve_api_with_sync_config(host, port, db_cfg, jwt_cfg, None, shutdown_rx).await
}

pub async fn serve_api_with_sync_config(
  host: String,
  port: String,
  db_cfg: DbConfig,
  jwt_cfg: JwtConfig,
  sync_config: Option<SyncConfig>,
  shutdown_rx: oneshot::Receiver<()>,
) -> Result<()> {
  let address: SocketAddr = format!("{}:{}", host, port).parse()?;
  let (shutdown_state_tx, shutdown_state_rx) = watch::channel(false);

  tokio::spawn(async move {
    let _ = shutdown_rx.await;
    let _ = shutdown_state_tx.send(true);
  });

  loop {
    let db_target = db_cfg
      .connection_url_public()
      .map_err(|err| anyhow::anyhow!("Invalid database configuration: {err}"))?;
    info!("Initializing database at: {db_target}");
    let (db, node_cfg) = init_database(&db_cfg).await?;
    let db = Arc::new(db);

    let cfg = Arc::new(ApiConfig::new(node_cfg, jwt_cfg.clone(), db_cfg.clone()));

    let (restart_tx, restart_rx) = oneshot::channel();
    let restart_tx = Arc::new(Mutex::new(Some(restart_tx)));

    let is_initialized = local::Entity::find_by_id(1)
      .one(db.as_ref())
      .await?
      .map(|row| row.is_initialized)
      .unwrap_or(false);

    let worker_status = Arc::new(tokio::sync::RwLock::new(WorkerStatus::default()));

    info!("Initializing API state...");
    let state = Arc::new(ApiState::new(
      db.clone(),
      cfg.clone(),
      restart_tx.clone(),
      worker_status.clone(),
      is_initialized,
    ));

    let mut worker_shutdown_tx = None;
    let mut worker_task = None;
    let has_central_api_url = cfg
      .node
      .central_api_url
      .as_ref()
      .map(|value| !value.trim().is_empty())
      .unwrap_or(false);
    if has_central_api_url {
      let (tx, rx) = oneshot::channel();
      worker_shutdown_tx = Some(tx);
      worker_task = Some(if let Some(ref sync_cfg) = sync_config {
        spawn_sync_worker_with_config(db, cfg.clone(), rx, sync_cfg.clone(), worker_status)
      } else {
        spawn_sync_worker(db, cfg.clone(), rx, worker_status)
      });
    }

    info!("Building API routes...");
    let router = build_router(state);

    let listener = TcpListener::bind(address).await?;
    info!("API server listening on http://{address}");
    let mut external_shutdown_rx = shutdown_state_rx.clone();

    axum::serve(listener, router)
      .with_graceful_shutdown(async move {
        tokio::select! {
          _ = wait_for_shutdown_signal(&mut external_shutdown_rx) => {},
          _ = restart_rx => {},
        }
        if let Some(worker_shutdown_tx) = worker_shutdown_tx {
          let _ = worker_shutdown_tx.send(());
        }
      })
      .await?;

    if let Some(mut worker_task) = worker_task {
      match tokio::time::timeout(Duration::from_secs(5), &mut worker_task).await {
        Ok(_) => {}
        Err(_) => {
          tracing::warn!("sync worker shutdown timed out, aborting task");
          worker_task.abort();
        }
      }
    }

    if *shutdown_state_rx.borrow() {
      break;
    }

    if restart_tx
      .lock()
      .expect("restart channel lock poisoned")
      .is_none()
    {
      tracing::info!("Restart requested via API endpoint, reinitializing API...");
      continue;
    }

    break;
  }

  Ok(())
}

async fn wait_for_shutdown_signal(shutdown_rx: &mut watch::Receiver<bool>) {
  if *shutdown_rx.borrow() {
    return;
  }

  while shutdown_rx.changed().await.is_ok() {
    if *shutdown_rx.borrow() {
      return;
    }
  }
}
