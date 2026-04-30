use reqwest::Client;
use uuid::Uuid;

use crate::{
  config::SyncConfig,
  dtos::SyncStatusResponse,
  services::sync::helpers::{compute_base_discriminant, join_uuid_csv},
  utils::http::get_api_json,
};

pub(super) struct SyncCycleRemoteStatus {
  pub(super) central_status: SyncStatusResponse,
  pub(super) assigned_base_ids_query: String,
  pub(super) assigned_base_discriminant: String,
}

impl SyncCycleRemoteStatus {
  pub(super) fn remote_node_id(&self) -> Uuid {
    self.central_status.node_id
  }
}

pub(super) async fn load_sync_cycle_remote_status(
  client: &Client,
  central_sync_api_url: &str,
  assigned_base_ids: &[Uuid],
  config: &SyncConfig,
) -> anyhow::Result<SyncCycleRemoteStatus> {
  let assigned_base_ids_query = join_uuid_csv(assigned_base_ids);

  let central_status: SyncStatusResponse = get_api_json(
    client,
    &format!(
      "{}/sync/status?baseIds={}",
      central_sync_api_url, assigned_base_ids_query
    ),
    config.probe_timeout,
  )
  .await?;

  Ok(SyncCycleRemoteStatus {
    central_status,
    assigned_base_ids_query,
    assigned_base_discriminant: compute_base_discriminant(assigned_base_ids),
  })
}
