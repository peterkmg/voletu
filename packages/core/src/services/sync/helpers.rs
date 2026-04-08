use sea_orm::{ColumnTrait, Condition};
use serde_json::Value;

use crate::{api::ApiError, entities::audit_log};

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

#[cfg(test)]
mod discriminant_tests {
  use uuid::Uuid;

  use super::compute_base_discriminant;

  #[test]
  fn empty_slice_produces_empty_string() {
    assert_eq!(compute_base_discriminant(&[]), "");
  }

  #[test]
  fn single_base_produces_raw_uuid_string() {
    let id = Uuid::parse_str("019d6cac-a37c-7b33-8986-3fce8fef921f").unwrap();
    assert_eq!(
      compute_base_discriminant(&[id]),
      "019d6cac-a37c-7b33-8986-3fce8fef921f"
    );
  }

  #[test]
  fn multiple_bases_are_sorted_and_comma_joined() {
    let a = Uuid::parse_str("019d6cac-a37c-7b33-8986-3fce8fef921f").unwrap();
    let b = Uuid::parse_str("019d6cac-0000-7b33-8986-3fce8fef921f").unwrap();
    let c = Uuid::parse_str("019d6cac-ffff-7b33-8986-3fce8fef921f").unwrap();
    let got = compute_base_discriminant(&[a, b, c]);
    assert_eq!(
      got,
      "019d6cac-0000-7b33-8986-3fce8fef921f,019d6cac-a37c-7b33-8986-3fce8fef921f,019d6cac-ffff-7b33-8986-3fce8fef921f"
    );
  }

  #[test]
  fn result_is_stable_regardless_of_input_order() {
    let a = Uuid::parse_str("019d6cac-a37c-7b33-8986-3fce8fef921f").unwrap();
    let b = Uuid::parse_str("019d6cac-0000-7b33-8986-3fce8fef921f").unwrap();
    let order1 = compute_base_discriminant(&[a, b]);
    let order2 = compute_base_discriminant(&[b, a]);
    assert_eq!(order1, order2);
  }

  #[test]
  fn duplicates_are_removed() {
    let a = Uuid::parse_str("019d6cac-a37c-7b33-8986-3fce8fef921f").unwrap();
    let got = compute_base_discriminant(&[a, a, a]);
    assert_eq!(got, "019d6cac-a37c-7b33-8986-3fce8fef921f");
  }
}
