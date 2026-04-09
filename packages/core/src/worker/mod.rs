use std::sync::Arc;

use sea_orm::DatabaseConnection;
use tokio::{sync::oneshot, time};
use tracing::info;

use crate::config::{ApiConfig, SyncConfig};

mod cycle;
mod state;
mod tick;
mod topology;

pub use state::{WorkerState, WorkerStatus};

pub fn spawn_sync_worker(
  db: Arc<DatabaseConnection>,
  cfg: Arc<ApiConfig>,
  shutdown_rx: oneshot::Receiver<()>,
  shared_status: Arc<tokio::sync::RwLock<WorkerStatus>>,
) -> tokio::task::JoinHandle<()> {
  spawn_sync_worker_with_config(db, cfg, shutdown_rx, SyncConfig::default(), shared_status)
}

pub fn spawn_sync_worker_with_config(
  db: Arc<DatabaseConnection>,
  cfg: Arc<ApiConfig>,
  mut shutdown_rx: oneshot::Receiver<()>,
  config: SyncConfig,
  shared_status: Arc<tokio::sync::RwLock<WorkerStatus>>,
) -> tokio::task::JoinHandle<()> {
  tokio::spawn(async move {
    let mut tick = time::interval(config.tick_interval);
    tick.set_missed_tick_behavior(time::MissedTickBehavior::Delay);
    let mut runtime = tick::WorkerRuntime::default(db, cfg, config);

    info!("sync worker started");

    loop {
      tokio::select! {
        _ = &mut shutdown_rx => {
          info!("sync worker shutdown requested");
          break;
        }
        _ = tick.tick() => {
          let outcome = tick::run_tick(&mut runtime).await;
          tick::publish_tick_outcome(&shared_status, outcome).await;
        }
      }
    }

    info!("sync worker stopped");
  })
}
