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

pub fn spawn_sync_worker(
  db: Arc<DatabaseConnection>,
  cfg: Arc<ApiConfig>,
  shutdown_rx: oneshot::Receiver<()>,
) -> tokio::task::JoinHandle<()> {
  spawn_sync_worker_with_config(db, cfg, shutdown_rx, SyncConfig::default())
}

pub fn spawn_sync_worker_with_config(
  db: Arc<DatabaseConnection>,
  cfg: Arc<ApiConfig>,
  mut shutdown_rx: oneshot::Receiver<()>,
  config: SyncConfig,
) -> tokio::task::JoinHandle<()> {
  tokio::spawn(async move {
    let client = Client::new();
    let sync_service = SyncService::new(db.clone(), cfg.clone());
    let mut tick = time::interval(config.tick_interval);
    tick.set_missed_tick_behavior(time::MissedTickBehavior::Delay);

    let mut state = state::WorkerState::Sleeping;
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
              state::transition(&mut state, state::WorkerState::Backoff);
              continue;
            }
          };

          if !node_type.eq_ignore_ascii_case("PERIPHERAL") {
            state::transition(&mut state, state::WorkerState::Sleeping);
            continue;
          }

          let Some(central_api_url) = central_api_url.map(|value| normalize_base_url(&value)) else {
            state::transition(&mut state, state::WorkerState::Sleeping);
            continue;
          };

          let local_status = match sync_service.sync_status().await {
            Ok(status) => status,
            Err(error) => {
              warn!(%error, "sync worker could not fetch local status");
              state::transition(&mut state, state::WorkerState::Backoff);
              continue;
            }
          };

          if local_status.highest_audit_log_id != last_local_highest {
            last_local_highest = local_status.highest_audit_log_id;
            has_updates = true;
          }

          let online_now = get_api_json::<SyncStatusResponse>(
            &client,
            &format!("{}/sync/status", central_api_url),
            config.probe_timeout,
          )
          .await
          .is_ok();
          if online_now != is_online {
            is_online = online_now;
            has_updates = true;
            debug!(online = is_online, "sync worker remote online state changed");
          }

          if !is_online {
            state::transition(&mut state, state::WorkerState::Offline);
            continue;
          }

          if !has_updates {
            state::transition(&mut state, state::WorkerState::OnlineIdle);
            continue;
          }

          state::transition(&mut state, state::WorkerState::Syncing);
          match cycle::sync_once(&client, &sync_service, &central_api_url, local_node_id, &config).await {
            Ok(changed) => {
              has_updates = false;
              state::transition(&mut state, state::WorkerState::OnlineIdle);
              debug!(changed, "sync worker cycle completed");
            }
            Err(error) => {
              warn!(%error, "sync worker cycle failed");
              state::transition(&mut state, state::WorkerState::Backoff);
            }
          }
        }
      }
    }

    info!("sync worker stopped");
  })
}
