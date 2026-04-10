use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
  dtos::SyncWatermarkResponse,
  enums::SyncDirection,
  services::system::{
    database_instance::load_active_database_instance,
    local::load_local_bootstrap,
    node_bases::load_node_base_ids as load_assigned_base_ids,
  },
};

pub(super) struct WorkerTopology {
  pub(super) local_node_id: Uuid,
  pub(super) node_type: String,
  pub(super) central_sync_api_url: Option<String>,
}

pub(super) async fn load_worker_topology(
  db: &DatabaseConnection,
) -> anyhow::Result<WorkerTopology> {
  let local_row = load_local_bootstrap(db)
    .await
    .map_err(|err| anyhow::anyhow!(err.to_string()))?;
  let instance = load_active_database_instance(db, local_row.local_db_id)
    .await
    .map_err(|err| anyhow::anyhow!(err.to_string()))?;

  let central_api_url = if let Some(value) = local_row.central_api_url.as_ref() {
    let trimmed = value.trim();
    if trimmed.is_empty() {
      None
    } else {
      Some(trimmed.to_string())
    }
  } else {
    None
  };

  Ok(WorkerTopology {
    local_node_id: instance.id,
    node_type: instance.node_type.to_string(),
    central_sync_api_url: central_api_url,
  })
}

/// Load base IDs assigned to the local node from the node_base_assignment table.
pub(super) async fn load_local_base_ids(
  db: &DatabaseConnection,
  node_id: Uuid,
) -> anyhow::Result<Vec<Uuid>> {
  load_assigned_base_ids(db, node_id)
    .await
    .map_err(Into::into)
}

/// Find the `(last_audit_log_id, base_discriminant)` pair for a given target
/// node and direction. Returns `(Uuid::nil(), String::new())` when no
/// watermark row exists yet.
pub(super) fn find_sync_watermark(
  watermarks: &[SyncWatermarkResponse],
  target_node_id: Uuid,
  direction: &str,
) -> (Uuid, String) {
  let watermark = watermarks.iter().find(|wm| {
    wm.target_node_id == target_node_id
      && matches!(
        (&wm.direction, direction),
        (SyncDirection::Push, "PUSH") | (SyncDirection::Pull, "PULL")
      )
  });

  match watermark {
    Some(wm) => (wm.last_audit_log_id, wm.base_discriminant.clone()),
    None => (Uuid::nil(), String::new()),
  }
}
