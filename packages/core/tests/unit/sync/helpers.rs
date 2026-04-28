use uuid::Uuid;
use voletu_core::{
  enums::AuditTable,
  services::sync::helpers::{compute_base_discriminant, excluded_sync_tables, scope_condition_for},
};

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

#[test]
fn scope_condition_for_empty_base_ids_constructs() {
  let _cond = scope_condition_for(&[]);
}

#[test]
fn scope_condition_for_single_base_constructs() {
  let base_id = Uuid::now_v7();
  let _cond = scope_condition_for(&[base_id]);
}

#[test]
fn scope_condition_for_multiple_bases_constructs() {
  let a = Uuid::now_v7();
  let b = Uuid::now_v7();
  let _cond = scope_condition_for(&[a, b]);
}

#[test]
fn excluded_tables_include_expected_variants() {
  let excluded = excluded_sync_tables();
  assert_eq!(excluded, vec![AuditTable::Local, AuditTable::Roles]);
}
