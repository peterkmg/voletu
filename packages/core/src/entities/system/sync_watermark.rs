use sea_orm::{entity::prelude::*, model, ActiveValue::Set, ConnectionTrait};

use crate::enums;

#[voletu_core_macros::handle_uuid(before_save = sync_watermark_before_save)]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "sync_watermarks")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub target_node_id: Uuid,
  pub direction: enums::SyncDirection,
  pub last_audit_log_id: Uuid,
  #[sea_orm(default_value = "")]
  pub base_discriminant: String,
  pub synced_at: DateTimeUtc,
}

pub async fn sync_watermark_before_save<C: ConnectionTrait>(
  mut model: ActiveModel,
  _db: &C,
  _insert: bool,
) -> Result<ActiveModel, DbErr> {
  model.synced_at = Set(ChronoUtc::now());
  Ok(model)
}
