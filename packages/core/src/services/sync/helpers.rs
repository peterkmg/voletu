use std::fmt::Write;

use sea_orm::{ColumnTrait, Condition};
use serde_json::Value;
use uuid::Uuid;

use crate::{api::ApiError, entities::audit_log, enums::AuditTable};

pub fn compute_base_discriminant(base_ids: &[Uuid]) -> String {
  let mut out = base_ids.to_vec();
  out.sort_unstable();
  out.dedup();
  join_uuid_csv(&out)
}

pub fn join_uuid_csv(ids: &[Uuid]) -> String {
  let mut out = String::with_capacity(ids.len().saturating_mul(37));

  for (index, id) in ids.iter().enumerate() {
    if index > 0 {
      out.push(',');
    }
    let _ = write!(&mut out, "{id}");
  }

  out
}

pub fn excluded_sync_tables() -> Vec<AuditTable> {
  AuditTable::sync_excluded_tables().to_vec()
}

pub fn scope_condition_for(base_ids: &[uuid::Uuid]) -> Condition {
  let mut cond = Condition::any()
    .add(audit_log::Column::TableName.is_in(AuditTable::sync_global_tables().iter().copied()));

  for base_id in base_ids {
    cond = cond.add(targeted_base_condition(&base_id.to_string()));
  }

  cond
}

pub fn normalize_target_base_ids(value: &str) -> String {
  let mut base_ids = value
    .split(',')
    .map(str::trim)
    .filter(|part| !part.is_empty())
    .collect::<Vec<_>>();
  base_ids.sort_unstable();
  base_ids.dedup();

  if base_ids.is_empty() {
    return String::new();
  }

  format!(",{},", base_ids.join(","))
}

pub fn targeted_base_condition(base_id: &str) -> Condition {
  let wrapped = format!("%,{},%", base_id);
  let starts_with = format!("{},%", base_id);
  let ends_with = format!("%,{}", base_id);

  Condition::any()
    .add(audit_log::Column::TargetBaseIds.eq(base_id))
    .add(audit_log::Column::TargetBaseIds.like(starts_with))
    .add(audit_log::Column::TargetBaseIds.like(ends_with))
    .add(audit_log::Column::TargetBaseIds.like(wrapped))
}

pub fn parse_json_field(raw: Option<&str>, field: &str) -> Result<Option<Value>, ApiError> {
  raw
    .map(|json| {
      serde_json::from_str(json)
        .map_err(|_| ApiError::Validation(format!("Invalid JSON payload for {}", field)))
    })
    .transpose()
}
