use sea_orm::EntityName;
use voletu_core::{
  entities::{dispatch_document, local},
  enums::AuditTable,
  services::audit::routing::{
    AuditRouteProfile,
    AuditTableCategory,
    DocumentHeaderRouteKind,
    ReferenceRouteKind,
    StorageItemRouteKind,
    TransportRouteKind,
  },
};

#[test]
fn audit_tables_expose_public_route_metadata_for_canonical_entities() {
  assert_eq!(
    AuditTable::DispatchDocuments.table_name(),
    "dispatch_documents"
  );
  assert_eq!(
    AuditTable::DispatchDocuments.table_name(),
    dispatch_document::Entity.table_name()
  );
  assert_eq!(AuditTable::Local.table_name(), local::Entity.table_name());
}

#[test]
fn audit_tables_map_into_stable_domain_categories() {
  assert_eq!(
    AuditTable::AcceptanceItems.category(),
    AuditTableCategory::StorageItems
  );
  assert_eq!(
    AuditTable::PhysicalTransferItems.category(),
    AuditTableCategory::StorageItems
  );
  assert_eq!(
    AuditTable::DispatchDocuments.category(),
    AuditTableCategory::DocumentHeaders
  );
  assert_eq!(
    AuditTable::InventoryReconciliations.category(),
    AuditTableCategory::Reconciliation
  );
  assert_eq!(
    AuditTable::Warehouses.category(),
    AuditTableCategory::References
  );
  assert_eq!(
    AuditTable::RailWagonWeights.category(),
    AuditTableCategory::Transport
  );
  assert_eq!(
    AuditTable::DispatchStorageMeasurements.category(),
    AuditTableCategory::DispatchMeasurements
  );
  assert_eq!(
    AuditTable::Companies.category(),
    AuditTableCategory::Broadcast
  );
}

#[test]
fn transport_tables_map_into_concrete_route_kinds() {
  assert_eq!(
    AuditTable::TruckWaybills.transport_route_kind(),
    Some(TransportRouteKind::TruckWaybill)
  );
  assert_eq!(
    AuditTable::RailWaybills.transport_route_kind(),
    Some(TransportRouteKind::RailWaybill)
  );
  assert_eq!(
    AuditTable::TruckWaybillItems.transport_route_kind(),
    Some(TransportRouteKind::TruckWaybillItem)
  );
  assert_eq!(
    AuditTable::TruckWeightDocs.transport_route_kind(),
    Some(TransportRouteKind::TruckWeightDoc)
  );
  assert_eq!(
    AuditTable::RailWagonManifests.transport_route_kind(),
    Some(TransportRouteKind::RailWagonManifest)
  );
  assert_eq!(
    AuditTable::RailWagonMeasurements.transport_route_kind(),
    Some(TransportRouteKind::RailWagonMeasurement)
  );
  assert_eq!(
    AuditTable::RailWagonWeights.transport_route_kind(),
    Some(TransportRouteKind::RailWagonWeight)
  );
  assert_eq!(AuditTable::DispatchDocuments.transport_route_kind(), None);
}

#[test]
fn document_header_tables_map_into_concrete_route_kinds() {
  assert_eq!(
    AuditTable::AcceptanceDocuments.document_header_route_kind(),
    Some(DocumentHeaderRouteKind::Acceptance)
  );
  assert_eq!(
    AuditTable::DispatchDocuments.document_header_route_kind(),
    Some(DocumentHeaderRouteKind::Dispatch)
  );
  assert_eq!(
    AuditTable::PhysicalStorageTransfers.document_header_route_kind(),
    Some(DocumentHeaderRouteKind::PhysicalTransfer)
  );
  assert_eq!(
    AuditTable::OwnershipTransfers.document_header_route_kind(),
    Some(DocumentHeaderRouteKind::OwnershipTransfer)
  );
  assert_eq!(
    AuditTable::BlendingDocuments.document_header_route_kind(),
    Some(DocumentHeaderRouteKind::Blending)
  );
  assert_eq!(AuditTable::Storages.document_header_route_kind(), None);
}

#[test]
fn reference_tables_map_into_concrete_route_kinds() {
  assert_eq!(
    AuditTable::Storages.reference_route_kind(),
    Some(ReferenceRouteKind::Storage)
  );
  assert_eq!(
    AuditTable::Warehouses.reference_route_kind(),
    Some(ReferenceRouteKind::Warehouse)
  );
  assert_eq!(
    AuditTable::Bases.reference_route_kind(),
    Some(ReferenceRouteKind::Base)
  );
  assert_eq!(AuditTable::TruckWaybills.reference_route_kind(), None);
}

#[test]
fn storage_item_tables_map_into_concrete_route_kinds() {
  assert_eq!(
    AuditTable::AcceptanceItems.storage_item_route_kind(),
    Some(StorageItemRouteKind::SingleStorage)
  );
  assert_eq!(
    AuditTable::DispatchItems.storage_item_route_kind(),
    Some(StorageItemRouteKind::SingleStorage)
  );
  assert_eq!(
    AuditTable::PhysicalTransferItems.storage_item_route_kind(),
    Some(StorageItemRouteKind::PhysicalTransfer)
  );
  assert_eq!(
    AuditTable::InventoryLedgerEntries.storage_item_route_kind(),
    Some(StorageItemRouteKind::SingleStorage)
  );
  assert_eq!(
    AuditTable::DispatchDocuments.storage_item_route_kind(),
    None
  );
}

#[test]
fn route_profiles_expose_consistent_category_and_subroute_metadata() {
  assert_eq!(
    AuditTable::PhysicalTransferItems.route_profile(),
    AuditRouteProfile {
      category: AuditTableCategory::StorageItems,
      storage_item: Some(StorageItemRouteKind::PhysicalTransfer),
      document_header: None,
      reference: None,
      transport: None,
    }
  );
  assert_eq!(
    AuditTable::DispatchDocuments.route_profile(),
    AuditRouteProfile {
      category: AuditTableCategory::DocumentHeaders,
      storage_item: None,
      document_header: Some(DocumentHeaderRouteKind::Dispatch),
      reference: None,
      transport: None,
    }
  );
  assert_eq!(AuditTable::Storages.route_profile(), AuditRouteProfile {
    category: AuditTableCategory::References,
    storage_item: None,
    document_header: None,
    reference: Some(ReferenceRouteKind::Storage),
    transport: None,
  });
  assert_eq!(
    AuditTable::RailWagonWeights.route_profile(),
    AuditRouteProfile {
      category: AuditTableCategory::Transport,
      storage_item: None,
      document_header: None,
      reference: None,
      transport: Some(TransportRouteKind::RailWagonWeight),
    }
  );
}
