use uuid::Uuid;

use super::{engine::WorkerRuntime, topology};
use crate::{services::sync::specs::SyncStatusQuerySpec, utils::http::normalize_base_url};

pub(super) struct WorkerContext {
  pub(super) central_sync_api_url: String,
  pub(super) local_base_ids: Vec<Uuid>,
  pub(super) local_base_discriminant: String,
  pub(super) local_highest_audit_log_id: Uuid,
}

pub(super) struct LoadedWorkerContext {
  pub(super) context: WorkerContext,
  pub(super) local_progress_changed: bool,
}

pub(super) async fn load_worker_context(
  runtime: &WorkerRuntime,
) -> anyhow::Result<Option<LoadedWorkerContext>> {
  let topology = topology::load_worker_topology(runtime.db.as_ref()).await?;

  if !topology.node_type.eq_ignore_ascii_case("PERIPHERAL") {
    return Ok(None);
  }

  let Some(central_sync_api_url) = topology
    .central_sync_api_url
    .as_deref()
    .map(normalize_base_url)
  else {
    return Ok(None);
  };

  let local_base_ids =
    topology::load_local_base_ids(runtime.db.as_ref(), topology.local_node_id).await?;
  let local_base_discriminant =
    crate::services::sync::helpers::compute_base_discriminant(&local_base_ids);
  let local_status = runtime
    .sync_service
    .sync_status(SyncStatusQuerySpec::default())
    .await?;
  let local_highest_audit_log_id = local_status.highest_audit_log_id;

  Ok(Some(LoadedWorkerContext {
    local_progress_changed: local_highest_audit_log_id != runtime.last_observed_local_audit_log_id,
    context: WorkerContext {
      central_sync_api_url,
      local_base_ids,
      local_base_discriminant,
      local_highest_audit_log_id,
    },
  }))
}
