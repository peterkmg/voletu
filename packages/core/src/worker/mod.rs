use std::sync::Arc;

use reqwest::Client;
use sea_orm::DatabaseConnection;
use tokio::{sync::oneshot, time};
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::{
  config::{ApiConfig, SyncConfig},
  dtos::SyncStatusResponse,
  services::sync::SyncService,
  utils::http::{get_api_json, normalize_base_url},
};

mod cycle;
mod state;
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
    let client = Client::new();
    let sync_service = SyncService::new(db.clone(), cfg.clone());
    let mut tick = time::interval(config.tick_interval);
    tick.set_missed_tick_behavior(time::MissedTickBehavior::Delay);

    let mut worker_state = state::WorkerState::Sleeping;
    let mut is_online = false;
    let mut has_updates = false;
    let mut last_local_highest = Uuid::nil();

    info!("sync worker started");

    loop {
      tokio::select! {
        _ = &mut shutdown_rx => {
          info!("sync worker shutdown requested");
          break;
        }
        _ = tick.tick() => {
          let (local_node_id, node_type, central_api_url) = match topology::load_runtime_topology(&db).await {
            Ok(config) => config,
            Err(error) => {
              warn!(%error, "sync worker could not load local topology");
              state::transition(&mut worker_state, state::WorkerState::Backoff);
              shared_status.write().await.state = worker_state;
              continue;
            }
          };

          if !node_type.eq_ignore_ascii_case("PERIPHERAL") {
            state::transition(&mut worker_state, state::WorkerState::Sleeping);
            shared_status.write().await.state = worker_state;
            continue;
          }

          let Some(central_api_url) = central_api_url.map(|value| normalize_base_url(&value)) else {
            state::transition(&mut worker_state, state::WorkerState::Sleeping);
            shared_status.write().await.state = worker_state;
            continue;
          };

          let local_base_ids = match topology::load_local_base_ids(&db, local_node_id).await {
            Ok(ids) => ids,
            Err(error) => {
              warn!(%error, "sync worker could not load base assignments");
              state::transition(&mut worker_state, state::WorkerState::Backoff);
              shared_status.write().await.state = worker_state;
              continue;
            }
          };

          let local_status = match sync_service.sync_status().await {
            Ok(status) => status,
            Err(error) => {
              warn!(%error, "sync worker could not fetch local status");
              state::transition(&mut worker_state, state::WorkerState::Backoff);
              shared_status.write().await.state = worker_state;
              continue;
            }
          };

          if local_status.highest_audit_log_id != last_local_highest {
            last_local_highest = local_status.highest_audit_log_id;
            has_updates = true;
          }

          let central_status = get_api_json::<SyncStatusResponse>(
            &client,
            &format!("{}/sync/status", central_api_url),
            config.probe_timeout,
          )
          .await;
          let online_now = central_status.is_ok();
          if online_now != is_online {
            is_online = online_now;
            has_updates = true;
            debug!(online = is_online, "sync worker remote online state changed");
          }

          if !is_online {
            state::transition(&mut worker_state, state::WorkerState::Offline);
            shared_status.write().await.state = worker_state;
            continue;
          }

          // Check if Central has new data we haven't pulled yet
          if let Ok(ref remote_status) = central_status {
            let watermarks = match sync_service.list_sync_watermarks().await {
              Ok(w) => w,
              Err(_) => vec![],
            };
            let pull_after = topology::watermark_for(&watermarks, remote_status.node_id, "PULL");
            if remote_status.highest_audit_log_id > pull_after {
              has_updates = true;
            }
          }

          if !has_updates {
            state::transition(&mut worker_state, state::WorkerState::OnlineIdle);
            shared_status.write().await.state = worker_state;
            continue;
          }

          state::transition(&mut worker_state, state::WorkerState::Syncing);
          shared_status.write().await.state = worker_state;
          match cycle::sync_once(&client, &sync_service, &central_api_url, local_node_id, &local_base_ids, &config).await {
            Ok(changed) => {
              has_updates = false;
              state::transition(&mut worker_state, state::WorkerState::OnlineIdle);
              let notify = {
                let mut status = shared_status.write().await;
                status.state = worker_state;
                status.last_sync_at = Some(chrono::Utc::now());
                Arc::clone(&status.cycle_completed)
              };
              notify.notify_waiters();
              debug!(changed, "sync worker cycle completed");
            }
            Err(error) => {
              warn!(%error, "sync worker cycle failed");
              state::transition(&mut worker_state, state::WorkerState::Backoff);
              shared_status.write().await.state = worker_state;
            }
          }
        }
      }
    }

    info!("sync worker stopped");
  })
}
