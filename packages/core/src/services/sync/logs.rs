use sea_orm::{
  ActiveModelTrait,
  ActiveValue::Set,
  ColumnTrait,
  EntityTrait,
  QueryFilter,
  QueryOrder,
  TransactionTrait,
};

use super::{
  helpers::{normalize_target_base_ids, parse_json_field},
  restore::apply_audit_log_restore,
  SyncService,
};
use crate::{
  api::ApiError,
  db::ops::exists_by_id,
  dtos::{PushAuditLogRequest, PushAuditLogsResponse},
  entities::audit_log,
  enums::AuditTable,
};

impl SyncService {
  pub async fn push_logs(
    &self,
    logs: &[PushAuditLogRequest],
  ) -> Result<PushAuditLogsResponse, ApiError> {
    tracing::debug!(count = logs.len(), "push_logs: processing incoming batch");
    let txn = self.db.begin().await?;
    let mut accepted = 0_u64;
    let mut rejected = 0_u64;

    for incoming in logs {
      let already_exists = exists_by_id::<audit_log::Entity>(&txn, incoming.id).await?;
      if already_exists {
        continue;
      }

      let latest_for_record = audit_log::Entity::find()
        .filter(audit_log::Column::RecordId.eq(incoming.record_id))
        .order_by_desc(audit_log::Column::Timestamp)
        .one(&txn)
        .await?;
      if let Some(existing) = latest_for_record {
        let incoming_ts = incoming.timestamp;
        if existing.user_role_weight > incoming.user_role_weight
          && existing.timestamp >= incoming_ts
        {
          rejected += 1;
          continue;
        }
      }

      let old_values = parse_json_field(incoming.old_values_json.as_deref(), "oldValuesJson")?;
      let new_values = parse_json_field(incoming.new_values_json.as_deref(), "newValuesJson")?;

      if AuditTable::resolve(&incoming.table_name).is_none() {
        return Err(ApiError::BadRequest(format!(
          "Unsupported audit table '{}' for sync log ingestion",
          incoming.table_name
        )));
      }
      apply_audit_log_restore(
        &txn,
        &incoming.table_name,
        incoming.action,
        old_values.as_ref(),
        new_values.as_ref(),
      )
      .await?;

      audit_log::ActiveModel {
        id: Set(incoming.id),
        table_name: Set(incoming.table_name.clone()),
        record_id: Set(incoming.record_id),
        action: Set(incoming.action),
        old_values: Set(old_values),
        new_values: Set(new_values),
        target_base_ids: Set(normalize_target_base_ids(&incoming.target_base_ids)),
        user_role_weight: Set(incoming.user_role_weight),
        user_id: Set(incoming.user_id),
        timestamp: Set(incoming.timestamp),
        origin_db_id: Set(incoming.origin_db_id),
      }
      .insert(&txn)
      .await?;
      accepted += 1;
    }

    txn.commit().await?;
    Ok(PushAuditLogsResponse { accepted, rejected })
  }
}
