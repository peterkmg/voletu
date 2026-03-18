use sea_orm::{ColumnTrait, Condition};
use serde_json::Value;

use crate::{api::ApiError, entities::audit_log};

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
