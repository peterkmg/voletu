use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

use crate::{
  db::ops::load_local_bootstrap,
  dtos::SyncWatermarkResponse,
  entities::database_instance,
  enums::SyncDirection,
};

pub(super) async fn load_runtime_topology(
  db: &DatabaseConnection,
) -> anyhow::Result<(Uuid, String, Option<String>)> {
  let local_row = load_local_bootstrap(db)
    .await
    .map_err(|err| anyhow::anyhow!(err.to_string()))?;
  let instance_row = database_instance::Entity::find_by_id(local_row.local_db_id)
    .one(db)
    .await?;
  let instance = match instance_row {
    Some(instance) => instance,
    None => return Err(anyhow::anyhow!("Database instance row is missing")),
  };

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

  Ok((instance.id, instance.node_type.to_string(), central_api_url))
}

pub(super) fn watermark_for(
  watermarks: &[SyncWatermarkResponse],
  target_node_id: Uuid,
  direction: &str,
) -> Uuid {
  let watermark = watermarks.iter().find(|wm| {
    wm.target_node_id == target_node_id
      && matches!(
        (&wm.direction, direction),
        (SyncDirection::Push, "PUSH") | (SyncDirection::Pull, "PULL")
      )
  });

  match watermark {
    Some(wm) => wm.last_audit_log_id,
    None => Uuid::nil(),
  }
}
