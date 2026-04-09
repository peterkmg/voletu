use sea_orm::{ColumnTrait, Condition};
use serde_json::Value;

use crate::{api::ApiError, entities::audit_log, enums::AuditTable};

/// Compute a canonical "base discriminant" string for a set of base UUIDs.
///
/// The discriminant is a sorted, deduplicated, comma-joined list of UUIDs in
/// their standard hyphenated form. An empty slice produces an empty string
/// (representing catalog-only scope). The result is stable regardless of the
/// input order, which lets it be compared byte-for-byte against a stored value
/// on a `sync_watermarks` row.
pub fn compute_base_discriminant(base_ids: &[uuid::Uuid]) -> String {
  let mut out: Vec<String> = base_ids.iter().map(|id| id.to_string()).collect();
  out.sort_unstable();
  out.dedup();
  out.join(",")
}

/// The set of audit-log `table_name` values that are never included in sync
/// pulls regardless of scope. These are node-local control / config tables
/// whose rows should never leave the node that owns them.
pub fn excluded_sync_tables() -> Vec<AuditTable> {
  AuditTable::sync_excluded_tables().to_vec()
}

/// Build the audit-log scope filter shared by `pull_logs` and `sync_status`.
///
/// The scope matches:
///   - any log in a "global" table (catalog + system broadcast tables), OR
///   - any log whose `target_base_ids` envelope includes at least one of the
///     provided base UUIDs.
///
/// An empty `base_ids` slice produces a catalog-only filter (global tables
/// only), which is what a peripheral with no base assignments should see.
///
/// Note: callers are still responsible for the `id > W` range filter and the
/// `tableName NOT IN excluded_sync_tables()` exclusion. This helper covers
/// only the positive "what's in scope" part.
pub fn scope_condition_for(base_ids: &[uuid::Uuid]) -> Condition {
  let mut cond = Condition::any().add(
    crate::entities::audit_log::Column::TableName
      .is_in(AuditTable::sync_global_tables().iter().copied()),
  );

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
