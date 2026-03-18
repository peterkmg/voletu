use sea_orm::{entity::prelude::*, model, ActiveValue::Set, ConnectionTrait};

use crate::enums;

#[voletu_core_macros::handle_uuid(before_save = audit_log_before_save)]
#[model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "audit_logs")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub table_name: String,
  pub record_id: Uuid,
  pub action: enums::AuditAction,
  pub old_values: Option<Json>,
  pub new_values: Option<Json>,
  pub target_base_ids: String,
  pub user_role_weight: i32,
  pub user_id: Uuid,
  pub timestamp: DateTimeUtc,
  pub origin_db_id: Uuid,
}

pub async fn audit_log_before_save<C: ConnectionTrait>(
  mut model: ActiveModel,
  _db: &C,
  insert: bool,
) -> Result<ActiveModel, DbErr> {
  if insert {
    model.timestamp = Set(ChronoUtc::now());
  }
  Ok(model)
}
