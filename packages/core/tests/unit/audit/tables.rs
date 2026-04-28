use sea_orm::{ActiveEnum, EntityName};
use voletu_core::{
  entities::{dispatch_document, local},
  enums::AuditTable,
};

#[test]
fn table_serializes_as_snake_case() {
  let serialized = serde_json::to_string(&AuditTable::DispatchDocuments).unwrap();
  assert_eq!(serialized, "\"dispatch_documents\"");
}

#[test]
fn table_deserializes_from_snake_case() {
  let parsed: AuditTable = serde_json::from_str("\"truck_waybills\"").unwrap();
  assert_eq!(parsed, AuditTable::TruckWaybills);
}

#[test]
fn table_exposes_canonical_names_for_mapped_entities() {
  assert_eq!(
    AuditTable::DispatchDocuments.table_name(),
    dispatch_document::Entity.table_name()
  );
  assert_eq!(AuditTable::Local.table_name(), local::Entity.table_name());
}

#[test]
fn table_active_enum_values_match_storage_format() {
  let value = AuditTable::DispatchDocuments.to_value();
  assert_eq!(value, "dispatch_documents");
}

#[test]
fn table_exposes_expected_sync_exclusions_as_canonical_variants() {
  assert_eq!(AuditTable::sync_excluded_tables(), &[
    AuditTable::Local,
    AuditTable::Roles
  ]);
}

#[test]
fn table_exposes_sync_global_set_as_canonical_variants() {
  assert_eq!(AuditTable::sync_global_tables(), &[
    AuditTable::Companies,
    AuditTable::Products,
    AuditTable::ProductGroups,
    AuditTable::ProductTypes,
    AuditTable::Bases,
    AuditTable::Warehouses,
    AuditTable::Storages,
    AuditTable::Ports,
    AuditTable::Users,
    AuditTable::DatabaseInstances,
  ]);
}
