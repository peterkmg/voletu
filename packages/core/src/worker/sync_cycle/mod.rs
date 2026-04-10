mod pull;
mod push;
mod remote;

use reqwest::Client;
use uuid::Uuid;

use crate::{config::SyncConfig, services::sync::SyncService};

pub(super) struct SyncCycleResult {
  changed_log_count: u64,
}

impl SyncCycleResult {
  pub(super) fn changed_log_count(&self) -> u64 {
    self.changed_log_count
  }
}

pub(super) async fn run_sync_cycle(
  client: &Client,
  sync_service: &SyncService,
  central_sync_api_url: &str,
  assigned_base_ids: &[Uuid],
  config: &SyncConfig,
) -> anyhow::Result<SyncCycleResult> {
  let remote_status =
    remote::load_sync_cycle_remote_status(client, central_sync_api_url, assigned_base_ids, config)
      .await?;
  let watermarks = sync_service.list_sync_watermarks().await?;

  let push_outcome = push::push_outbound_logs(
    client,
    sync_service,
    central_sync_api_url,
    &remote_status,
    &watermarks,
    config,
  )
  .await?;
  let pull_outcome = pull::pull_remote_logs(
    client,
    sync_service,
    central_sync_api_url,
    &remote_status,
    &watermarks,
    config,
  )
  .await?;

  Ok(SyncCycleResult {
    changed_log_count: push_outcome.pushed_log_count + pull_outcome.pulled_log_count,
  })
}
