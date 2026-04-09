//! Routing envelope resolver — determines which bases are affected by an entity change.
//!
//! Called during audit log registration to populate `target_base_ids`.
//! Resolution chain: entity → storage_id → storage.warehouse_id → warehouse.base_id

use sea_orm::{ColumnTrait, ConnectionTrait, EntityLoaderTrait, QueryFilter};
use serde_json::Value;
use uuid::Uuid;

use crate::{
  api::ApiError,
  entities::{
    acceptance_document,
    acceptance_item,
    blending_component,
    blending_document,
    blending_result,
    dispatch_document,
    dispatch_item,
    dispatch_storage_measurement,
    inventory_reconciliation,
    ownership_transfer,
    ownership_transfer_item,
    physical_storage_transfer,
    physical_transfer_item,
    rail_wagon_manifest,
    rail_wagon_measurement,
    rail_wagon_weight,
    rail_waybill,
    role,
    storage,
    truck_waybill,
    truck_waybill_item,
    truck_weight_doc,
    user,
    warehouse,
  },
  enums::{AuditTable, RoleType},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditTableCategory {
  StorageItems,
  DocumentHeaders,
  Reconciliation,
  References,
  Transport,
  DispatchMeasurements,
  Broadcast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportRouteKind {
  TruckWaybill,
  RailWaybill,
  TruckWaybillItem,
  TruckWeightDoc,
  RailWagonManifest,
  RailWagonMeasurement,
  RailWagonWeight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentHeaderRouteKind {
  Acceptance,
  Dispatch,
  PhysicalTransfer,
  OwnershipTransfer,
  Blending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceRouteKind {
  Storage,
  Warehouse,
  Base,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageItemRouteKind {
  SingleStorage,
  PhysicalTransfer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuditRouteProfile {
  pub category: AuditTableCategory,
  pub storage_item: Option<StorageItemRouteKind>,
  pub document_header: Option<DocumentHeaderRouteKind>,
  pub reference: Option<ReferenceRouteKind>,
  pub transport: Option<TransportRouteKind>,
}

impl AuditTable {
  pub fn category(self) -> AuditTableCategory {
    self.route_profile().category
  }

  pub fn transport_route_kind(self) -> Option<TransportRouteKind> {
    self.route_profile().transport
  }

  pub fn document_header_route_kind(self) -> Option<DocumentHeaderRouteKind> {
    self.route_profile().document_header
  }

  pub fn reference_route_kind(self) -> Option<ReferenceRouteKind> {
    self.route_profile().reference
  }

  pub fn storage_item_route_kind(self) -> Option<StorageItemRouteKind> {
    self.route_profile().storage_item
  }

  pub fn route_profile(self) -> AuditRouteProfile {
    match self {
      Self::AcceptanceItems
      | Self::DispatchItems
      | Self::BlendingComponents
      | Self::BlendingResults
      | Self::OwnershipTransferItems
      | Self::InventoryAdjustments
      | Self::InventoryLedgerEntries => AuditRouteProfile {
        category: AuditTableCategory::StorageItems,
        storage_item: Some(StorageItemRouteKind::SingleStorage),
        document_header: None,
        reference: None,
        transport: None,
      },
      Self::PhysicalTransferItems => AuditRouteProfile {
        category: AuditTableCategory::StorageItems,
        storage_item: Some(StorageItemRouteKind::PhysicalTransfer),
        document_header: None,
        reference: None,
        transport: None,
      },
      Self::AcceptanceDocuments => AuditRouteProfile {
        category: AuditTableCategory::DocumentHeaders,
        storage_item: None,
        document_header: Some(DocumentHeaderRouteKind::Acceptance),
        reference: None,
        transport: None,
      },
      Self::DispatchDocuments => AuditRouteProfile {
        category: AuditTableCategory::DocumentHeaders,
        storage_item: None,
        document_header: Some(DocumentHeaderRouteKind::Dispatch),
        reference: None,
        transport: None,
      },
      Self::PhysicalStorageTransfers => AuditRouteProfile {
        category: AuditTableCategory::DocumentHeaders,
        storage_item: None,
        document_header: Some(DocumentHeaderRouteKind::PhysicalTransfer),
        reference: None,
        transport: None,
      },
      Self::OwnershipTransfers => AuditRouteProfile {
        category: AuditTableCategory::DocumentHeaders,
        storage_item: None,
        document_header: Some(DocumentHeaderRouteKind::OwnershipTransfer),
        reference: None,
        transport: None,
      },
      Self::BlendingDocuments => AuditRouteProfile {
        category: AuditTableCategory::DocumentHeaders,
        storage_item: None,
        document_header: Some(DocumentHeaderRouteKind::Blending),
        reference: None,
        transport: None,
      },
      Self::InventoryReconciliations => AuditRouteProfile {
        category: AuditTableCategory::Reconciliation,
        storage_item: None,
        document_header: None,
        reference: None,
        transport: None,
      },
      Self::Storages => AuditRouteProfile {
        category: AuditTableCategory::References,
        storage_item: None,
        document_header: None,
        reference: Some(ReferenceRouteKind::Storage),
        transport: None,
      },
      Self::Warehouses => AuditRouteProfile {
        category: AuditTableCategory::References,
        storage_item: None,
        document_header: None,
        reference: Some(ReferenceRouteKind::Warehouse),
        transport: None,
      },
      Self::Bases => AuditRouteProfile {
        category: AuditTableCategory::References,
        storage_item: None,
        document_header: None,
        reference: Some(ReferenceRouteKind::Base),
        transport: None,
      },
      Self::TruckWaybills => AuditRouteProfile {
        category: AuditTableCategory::Transport,
        storage_item: None,
        document_header: None,
        reference: None,
        transport: Some(TransportRouteKind::TruckWaybill),
      },
      Self::RailWaybills => AuditRouteProfile {
        category: AuditTableCategory::Transport,
        storage_item: None,
        document_header: None,
        reference: None,
        transport: Some(TransportRouteKind::RailWaybill),
      },
      Self::TruckWaybillItems => AuditRouteProfile {
        category: AuditTableCategory::Transport,
        storage_item: None,
        document_header: None,
        reference: None,
        transport: Some(TransportRouteKind::TruckWaybillItem),
      },
      Self::TruckWeightDocs => AuditRouteProfile {
        category: AuditTableCategory::Transport,
        storage_item: None,
        document_header: None,
        reference: None,
        transport: Some(TransportRouteKind::TruckWeightDoc),
      },
      Self::RailWagonManifests => AuditRouteProfile {
        category: AuditTableCategory::Transport,
        storage_item: None,
        document_header: None,
        reference: None,
        transport: Some(TransportRouteKind::RailWagonManifest),
      },
      Self::RailWagonMeasurements => AuditRouteProfile {
        category: AuditTableCategory::Transport,
        storage_item: None,
        document_header: None,
        reference: None,
        transport: Some(TransportRouteKind::RailWagonMeasurement),
      },
      Self::RailWagonWeights => AuditRouteProfile {
        category: AuditTableCategory::Transport,
        storage_item: None,
        document_header: None,
        reference: None,
        transport: Some(TransportRouteKind::RailWagonWeight),
      },
      Self::DispatchStorageMeasurements => AuditRouteProfile {
        category: AuditTableCategory::DispatchMeasurements,
        storage_item: None,
        document_header: None,
        reference: None,
        transport: None,
      },
      Self::AuditLogs
      | Self::Companies
      | Self::DatabaseInstances
      | Self::Local
      | Self::Ports
      | Self::ProductGroups
      | Self::Products
      | Self::ProductTypes
      | Self::RefreshTokens
      | Self::Roles
      | Self::SyncWatermarks
      | Self::Users => AuditRouteProfile {
        category: AuditTableCategory::Broadcast,
        storage_item: None,
        document_header: None,
        reference: None,
        transport: None,
      },
    }
  }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Resolve the target base IDs for a given entity change.
/// Returns empty vec for global/broadcast tables (catalog, system).
pub async fn resolve_target_base_ids<C: ConnectionTrait>(
  conn: &C,
  table: AuditTable,
  record_id: Uuid,
  new_values: Option<&Value>,
) -> Result<Vec<Uuid>, ApiError> {
  resolve_target_base_ids_by_category(conn, table, record_id, new_values).await
}

async fn resolve_target_base_ids_by_category<C: ConnectionTrait>(
  conn: &C,
  table: AuditTable,
  record_id: Uuid,
  new_values: Option<&Value>,
) -> Result<Vec<Uuid>, ApiError> {
  match table.category() {
    AuditTableCategory::StorageItems => {
      resolve_storage_item_bases(conn, table, record_id, new_values).await
    }
    AuditTableCategory::DocumentHeaders => {
      resolve_document_header_bases(conn, table, record_id).await
    }
    AuditTableCategory::Reconciliation => {
      resolve_reconciliation_bases(conn, new_values, record_id).await
    }
    AuditTableCategory::References => {
      resolve_reference_bases(conn, table, record_id, new_values).await
    }
    AuditTableCategory::Transport => {
      resolve_transport_bases(conn, table, record_id, new_values).await
    }
    // --- Dispatch measurements: inherit from parent dispatch document ---
    AuditTableCategory::DispatchMeasurements => {
      resolve_dispatch_measurement_base(conn, new_values, record_id).await
    }
    // --- Global tables (catalog, system): broadcast to all ---
    AuditTableCategory::Broadcast => Ok(vec![]),
  }
}

/// Resolve the role weight for an actor (user) for conflict resolution.
pub async fn resolve_role_weight<C: ConnectionTrait>(
  conn: &C,
  actor_id: Uuid,
) -> Result<i32, ApiError> {
  let user_with_role: Option<user::ModelEx> = user::Entity::load()
    .filter_by_id(actor_id)
    .with(role::Entity)
    .one(conn)
    .await?;

  let weight = match user_with_role.as_ref().and_then(|user| user.role.as_ref()) {
    Some(role) => match role.common_name {
      RoleType::Admin => 100,
      RoleType::SeniorSupervisor => 40,
      RoleType::Supervisor => 10,
      RoleType::Operator => 1,
    },
    None => 0,
  };

  Ok(weight)
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn dedupe_base_ids(mut base_ids: Vec<Uuid>) -> Vec<Uuid> {
  base_ids.sort();
  base_ids.dedup();
  base_ids
}

/// Extract a UUID field from a JSON Value (the serialized entity snapshot).
fn extract_uuid_field(json: Option<&Value>, field: &str) -> Option<Uuid> {
  json
    .and_then(|v| v.get(field))
    .and_then(|v| v.as_str())
    .and_then(|s| Uuid::try_parse(s).ok())
}

/// Resolve a single storage_id → warehouse → base chain.
async fn resolve_storage_to_base<C: ConnectionTrait>(
  conn: &C,
  storage_id: Uuid,
) -> Result<Option<Uuid>, ApiError> {
  let storage: Option<storage::ModelEx> = storage::Entity::load()
    .filter_by_id(storage_id)
    .with(warehouse::Entity)
    .one(conn)
    .await?;

  Ok(storage.and_then(|storage| {
    storage
      .warehouse
      .as_ref()
      .map(|warehouse| warehouse.base_id)
  }))
}

/// Resolve multiple storage_ids → base_ids (deduped).
async fn resolve_storages_to_bases<C: ConnectionTrait>(
  conn: &C,
  storage_ids: &[Uuid],
) -> Result<Vec<Uuid>, ApiError> {
  if storage_ids.is_empty() {
    return Ok(vec![]);
  }

  let storages: Vec<storage::ModelEx> = storage::Entity::load()
    .filter(storage::Column::Id.is_in(storage_ids.iter().copied()))
    .with(warehouse::Entity)
    .all(conn)
    .await?;

  Ok(dedupe_base_ids(
    storages
      .into_iter()
      .filter_map(|storage| {
        storage
          .warehouse
          .as_ref()
          .map(|warehouse| warehouse.base_id)
      })
      .collect(),
  ))
}

/// Resolve via a single storage FK field on the entity.
async fn resolve_via_storage<C: ConnectionTrait>(
  conn: &C,
  new_values: Option<&Value>,
  storage_field: &str,
) -> Result<Vec<Uuid>, ApiError> {
  match extract_uuid_field(new_values, storage_field) {
    Some(storage_id) => match resolve_storage_to_base(conn, storage_id).await? {
      Some(base_id) => Ok(vec![base_id]),
      None => Ok(vec![]),
    },
    None => {
      // No new_values (update/delete) — can't resolve without loading entity.
      // For items, the audit log from creation already has routing.
      // Updates/deletes inherit the same routing scope.
      Ok(vec![])
    }
  }
}

/// Resolve reconciliation via warehouse_id on the document.
async fn resolve_via_warehouse<C: ConnectionTrait>(
  conn: &C,
  new_values: Option<&Value>,
  record_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  if let Some(warehouse_id) = extract_uuid_field(new_values, "warehouse_id") {
    return resolve_warehouse_to_base(conn, warehouse_id).await;
  }

  let reconciliation: Option<inventory_reconciliation::ModelEx> =
    inventory_reconciliation::Entity::load()
      .filter_by_id(record_id)
      .one(conn)
      .await?;

  match reconciliation {
    Some(reconciliation) => resolve_warehouse_to_base(conn, reconciliation.warehouse_id).await,
    None => Ok(vec![]),
  }
}

async fn resolve_warehouse_to_base<C: ConnectionTrait>(
  conn: &C,
  warehouse_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  let warehouse: Option<warehouse::ModelEx> = warehouse::Entity::load()
    .filter_by_id(warehouse_id)
    .one(conn)
    .await?;

  Ok(
    warehouse
      .map(|warehouse| vec![warehouse.base_id])
      .unwrap_or_default(),
  )
}

async fn resolve_storage_item_bases<C: ConnectionTrait>(
  conn: &C,
  table: AuditTable,
  record_id: Uuid,
  new_values: Option<&Value>,
) -> Result<Vec<Uuid>, ApiError> {
  match table.storage_item_route_kind() {
    Some(StorageItemRouteKind::SingleStorage) => {
      resolve_single_storage_item_bases(conn, record_id, new_values).await
    }
    Some(StorageItemRouteKind::PhysicalTransfer) => {
      resolve_physical_transfer_item_bases(conn, record_id, new_values).await
    }
    None => unreachable!("non-storage-item table passed to resolve_storage_item_bases"),
  }
}

async fn resolve_single_storage_item_bases<C: ConnectionTrait>(
  conn: &C,
  record_id: Uuid,
  new_values: Option<&Value>,
) -> Result<Vec<Uuid>, ApiError> {
  let _ = record_id;
  resolve_via_storage(conn, new_values, "storage_id").await
}

async fn resolve_physical_transfer_item_bases<C: ConnectionTrait>(
  conn: &C,
  record_id: Uuid,
  new_values: Option<&Value>,
) -> Result<Vec<Uuid>, ApiError> {
  let _ = record_id;
  let mut bases = Vec::new();
  bases.extend(resolve_via_storage(conn, new_values, "from_storage_id").await?);
  bases.extend(resolve_via_storage(conn, new_values, "to_storage_id").await?);
  Ok(dedupe_base_ids(bases))
}

async fn resolve_document_header_bases<C: ConnectionTrait>(
  conn: &C,
  table: AuditTable,
  record_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  match table.document_header_route_kind() {
    Some(DocumentHeaderRouteKind::Acceptance) => {
      resolve_acceptance_doc_bases(conn, record_id).await
    }
    Some(DocumentHeaderRouteKind::Dispatch) => resolve_dispatch_doc_bases(conn, record_id).await,
    Some(DocumentHeaderRouteKind::PhysicalTransfer) => {
      resolve_physical_transfer_doc_bases(conn, record_id).await
    }
    Some(DocumentHeaderRouteKind::OwnershipTransfer) => {
      resolve_ownership_doc_bases(conn, record_id).await
    }
    Some(DocumentHeaderRouteKind::Blending) => resolve_blending_doc_bases(conn, record_id).await,
    None => unreachable!("non-document-header table passed to resolve_document_header_bases"),
  }
}

async fn resolve_reconciliation_bases<C: ConnectionTrait>(
  conn: &C,
  new_values: Option<&Value>,
  record_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  resolve_via_warehouse(conn, new_values, record_id).await
}

async fn resolve_reference_bases<C: ConnectionTrait>(
  conn: &C,
  table: AuditTable,
  record_id: Uuid,
  new_values: Option<&Value>,
) -> Result<Vec<Uuid>, ApiError> {
  match table.reference_route_kind() {
    Some(ReferenceRouteKind::Storage) => {
      resolve_storage_reference_bases(conn, record_id, new_values).await
    }
    Some(ReferenceRouteKind::Warehouse) => {
      resolve_warehouse_reference_bases(conn, record_id, new_values).await
    }
    Some(ReferenceRouteKind::Base) => Ok(vec![record_id]),
    None => unreachable!("non-reference table passed to resolve_reference_bases"),
  }
}

async fn resolve_storage_reference_bases<C: ConnectionTrait>(
  conn: &C,
  record_id: Uuid,
  new_values: Option<&Value>,
) -> Result<Vec<Uuid>, ApiError> {
  if let Some(warehouse_id) = extract_uuid_field(new_values, "warehouse_id") {
    resolve_warehouse_to_base(conn, warehouse_id).await
  } else {
    match resolve_storage_to_base(conn, record_id).await? {
      Some(base_id) => Ok(vec![base_id]),
      None => Ok(vec![]),
    }
  }
}

async fn resolve_warehouse_reference_bases<C: ConnectionTrait>(
  conn: &C,
  record_id: Uuid,
  new_values: Option<&Value>,
) -> Result<Vec<Uuid>, ApiError> {
  if let Some(base_id) = extract_uuid_field(new_values, "base_id") {
    Ok(vec![base_id])
  } else {
    let warehouse: Option<warehouse::ModelEx> = warehouse::Entity::load()
      .filter_by_id(record_id)
      .one(conn)
      .await?;
    Ok(
      warehouse
        .map(|warehouse| vec![warehouse.base_id])
        .unwrap_or_default(),
    )
  }
}

// ---------------------------------------------------------------------------
// Document header resolvers — load headers with child rows, collect storage_ids
// ---------------------------------------------------------------------------

async fn resolve_acceptance_doc_bases<C: ConnectionTrait>(
  conn: &C,
  doc_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  let doc: Option<acceptance_document::ModelEx> = acceptance_document::Entity::load()
    .filter_by_id(doc_id)
    .with(acceptance_item::Entity)
    .one(conn)
    .await?;

  let storage_ids: Vec<Uuid> = doc
    .into_iter()
    .flat_map(|doc| doc.items.into_iter().map(|item| item.storage_id))
    .collect();

  resolve_storages_to_bases(conn, &storage_ids).await
}

async fn resolve_dispatch_doc_bases<C: ConnectionTrait>(
  conn: &C,
  doc_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  let doc: Option<dispatch_document::ModelEx> = dispatch_document::Entity::load()
    .filter_by_id(doc_id)
    .with(dispatch_item::Entity)
    .one(conn)
    .await?;

  let Some(doc) = doc else {
    return Ok(vec![]);
  };

  let storage_ids: Vec<Uuid> = doc.items.iter().map(|item| item.storage_id).collect();
  let mut base_ids = resolve_storages_to_bases(conn, &storage_ids).await?;

  if let Some(destination_base_id) = doc.destination_base_id {
    base_ids.push(destination_base_id);
  }

  Ok(dedupe_base_ids(base_ids))
}

async fn resolve_physical_transfer_doc_bases<C: ConnectionTrait>(
  conn: &C,
  doc_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  let doc: Option<physical_storage_transfer::ModelEx> = physical_storage_transfer::Entity::load()
    .filter_by_id(doc_id)
    .with(physical_transfer_item::Entity)
    .one(conn)
    .await?;

  let storage_ids: Vec<Uuid> = doc
    .into_iter()
    .flat_map(|doc| {
      doc
        .items
        .into_iter()
        .flat_map(|item| [item.from_storage_id, item.to_storage_id])
    })
    .collect();

  resolve_storages_to_bases(conn, &storage_ids).await
}

async fn resolve_ownership_doc_bases<C: ConnectionTrait>(
  conn: &C,
  doc_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  let doc: Option<ownership_transfer::ModelEx> = ownership_transfer::Entity::load()
    .filter_by_id(doc_id)
    .with(ownership_transfer_item::Entity)
    .one(conn)
    .await?;

  let storage_ids: Vec<Uuid> = doc
    .into_iter()
    .flat_map(|doc| doc.items.into_iter().map(|item| item.storage_id))
    .collect();

  resolve_storages_to_bases(conn, &storage_ids).await
}

async fn resolve_blending_doc_bases<C: ConnectionTrait>(
  conn: &C,
  doc_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  let doc: Option<blending_document::ModelEx> = blending_document::Entity::load()
    .filter_by_id(doc_id)
    .with(blending_component::Entity)
    .with(blending_result::Entity)
    .one(conn)
    .await?;

  let storage_ids: Vec<Uuid> = doc
    .into_iter()
    .flat_map(|doc| {
      doc
        .components
        .into_iter()
        .map(|component| component.storage_id)
        .chain(doc.results.into_iter().map(|result| result.storage_id))
    })
    .collect();

  resolve_storages_to_bases(conn, &storage_ids).await
}

// ---------------------------------------------------------------------------
// Waybill + measurement routing helpers
// ---------------------------------------------------------------------------

/// Resolve routing via a direct `base_id` field on the entity.
async fn resolve_via_base_id<Fut>(
  new_values: Option<&Value>,
  record_id: Uuid,
  fallback: impl FnOnce(Uuid) -> Fut,
) -> Result<Vec<Uuid>, ApiError>
where
  Fut: std::future::Future<Output = Result<Option<Uuid>, sea_orm::DbErr>>,
{
  if let Some(base_id) = extract_uuid_field(new_values, "base_id") {
    return Ok(vec![base_id]);
  }

  match fallback(record_id).await? {
    Some(base_id) => Ok(vec![base_id]),
    None => Ok(vec![]),
  }
}

async fn resolve_truck_waybill_base<C: ConnectionTrait>(
  conn: &C,
  waybill_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  Ok(
    load_truck_waybill_base_id(conn, waybill_id)
      .await?
      .into_iter()
      .collect(),
  )
}

async fn resolve_rail_waybill_base<C: ConnectionTrait>(
  conn: &C,
  waybill_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  Ok(
    load_rail_waybill_base_id(conn, waybill_id)
      .await?
      .into_iter()
      .collect(),
  )
}

async fn resolve_manifest_waybill_base<C: ConnectionTrait>(
  conn: &C,
  manifest_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  let manifest: Option<rail_wagon_manifest::ModelEx> = rail_wagon_manifest::Entity::load()
    .filter_by_id(manifest_id)
    .one(conn)
    .await?;

  match manifest {
    Some(manifest) => resolve_rail_waybill_base(conn, manifest.rail_waybill_id).await,
    None => Ok(vec![]),
  }
}

async fn resolve_transport_bases<C: ConnectionTrait>(
  conn: &C,
  table: AuditTable,
  record_id: Uuid,
  new_values: Option<&Value>,
) -> Result<Vec<Uuid>, ApiError> {
  match table.transport_route_kind() {
    Some(TransportRouteKind::TruckWaybill) => {
      resolve_truck_waybill_route(conn, new_values, record_id).await
    }
    Some(TransportRouteKind::RailWaybill) => {
      resolve_rail_waybill_route(conn, new_values, record_id).await
    }
    Some(TransportRouteKind::TruckWaybillItem) => {
      resolve_truck_waybill_item_base(conn, new_values, record_id).await
    }
    Some(TransportRouteKind::TruckWeightDoc) => {
      resolve_truck_weight_doc_base(conn, new_values, record_id).await
    }
    Some(TransportRouteKind::RailWagonManifest) => {
      resolve_rail_manifest_base(conn, new_values, record_id).await
    }
    Some(TransportRouteKind::RailWagonMeasurement) => {
      resolve_rail_wagon_measurement_base(conn, new_values, record_id).await
    }
    Some(TransportRouteKind::RailWagonWeight) => {
      resolve_rail_wagon_weight_base(conn, new_values, record_id).await
    }
    None => unreachable!("non-transport table passed to resolve_transport_bases"),
  }
}

async fn resolve_truck_waybill_route<C: ConnectionTrait>(
  conn: &C,
  new_values: Option<&Value>,
  record_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  resolve_via_base_id(new_values, record_id, |id| {
    load_truck_waybill_base_id(conn, id)
  })
  .await
}

async fn resolve_rail_waybill_route<C: ConnectionTrait>(
  conn: &C,
  new_values: Option<&Value>,
  record_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  resolve_via_base_id(new_values, record_id, |id| {
    load_rail_waybill_base_id(conn, id)
  })
  .await
}

async fn resolve_truck_waybill_item_base<C: ConnectionTrait>(
  conn: &C,
  new_values: Option<&Value>,
  record_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  let waybill_id = match extract_uuid_field(new_values, "truck_waybill_id") {
    Some(id) => id,
    None => {
      let item: Option<truck_waybill_item::ModelEx> = truck_waybill_item::Entity::load()
        .filter_by_id(record_id)
        .one(conn)
        .await?;
      match item {
        Some(item) => item.truck_waybill_id,
        None => return Ok(vec![]),
      }
    }
  };

  resolve_truck_waybill_base(conn, waybill_id).await
}

async fn resolve_truck_weight_doc_base<C: ConnectionTrait>(
  conn: &C,
  new_values: Option<&Value>,
  record_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  let waybill_id = match extract_uuid_field(new_values, "truck_waybill_id") {
    Some(id) => id,
    None => {
      let weight_doc: Option<truck_weight_doc::ModelEx> = truck_weight_doc::Entity::load()
        .filter_by_id(record_id)
        .one(conn)
        .await?;
      match weight_doc {
        Some(weight_doc) => weight_doc.truck_waybill_id,
        None => return Ok(vec![]),
      }
    }
  };

  resolve_truck_waybill_base(conn, waybill_id).await
}

async fn resolve_rail_wagon_measurement_base<C: ConnectionTrait>(
  conn: &C,
  new_values: Option<&Value>,
  record_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  let manifest_id = match extract_uuid_field(new_values, "wagon_manifest_id") {
    Some(id) => id,
    None => {
      let measurement: Option<rail_wagon_measurement::ModelEx> =
        rail_wagon_measurement::Entity::load()
          .filter_by_id(record_id)
          .one(conn)
          .await?;
      match measurement {
        Some(measurement) => measurement.wagon_manifest_id,
        None => return Ok(vec![]),
      }
    }
  };

  resolve_manifest_waybill_base(conn, manifest_id).await
}

async fn resolve_rail_wagon_weight_base<C: ConnectionTrait>(
  conn: &C,
  new_values: Option<&Value>,
  record_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  let manifest_id = match extract_uuid_field(new_values, "wagon_manifest_id") {
    Some(id) => id,
    None => {
      let weight: Option<rail_wagon_weight::ModelEx> = rail_wagon_weight::Entity::load()
        .filter_by_id(record_id)
        .one(conn)
        .await?;
      match weight {
        Some(weight) => weight.wagon_manifest_id,
        None => return Ok(vec![]),
      }
    }
  };

  resolve_manifest_waybill_base(conn, manifest_id).await
}

async fn load_truck_waybill_base_id<C: ConnectionTrait>(
  conn: &C,
  waybill_id: Uuid,
) -> Result<Option<Uuid>, sea_orm::DbErr> {
  let waybill: Option<truck_waybill::ModelEx> = truck_waybill::Entity::load()
    .filter_by_id(waybill_id)
    .one(conn)
    .await?;

  Ok(waybill.map(|waybill| waybill.base_id))
}

async fn load_rail_waybill_base_id<C: ConnectionTrait>(
  conn: &C,
  waybill_id: Uuid,
) -> Result<Option<Uuid>, sea_orm::DbErr> {
  let waybill: Option<rail_waybill::ModelEx> = rail_waybill::Entity::load()
    .filter_by_id(waybill_id)
    .one(conn)
    .await?;

  Ok(waybill.map(|waybill| waybill.base_id))
}

/// Resolve rail wagon manifest → parent rail waybill → base_id.
async fn resolve_rail_manifest_base<C: ConnectionTrait>(
  conn: &C,
  new_values: Option<&Value>,
  record_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  let waybill_id = match extract_uuid_field(new_values, "rail_waybill_id") {
    Some(id) => id,
    None => {
      let manifest: Option<rail_wagon_manifest::ModelEx> = rail_wagon_manifest::Entity::load()
        .filter_by_id(record_id)
        .one(conn)
        .await?;
      match manifest {
        Some(manifest) => manifest.rail_waybill_id,
        None => return Ok(vec![]),
      }
    }
  };

  resolve_rail_waybill_base(conn, waybill_id).await
}

/// Resolve dispatch_storage_measurement → parent dispatch document → routing.
async fn resolve_dispatch_measurement_base<C: ConnectionTrait>(
  conn: &C,
  new_values: Option<&Value>,
  record_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  let doc_id = match extract_uuid_field(new_values, "dispatch_doc_id") {
    Some(id) => id,
    None => {
      let measurement: Option<dispatch_storage_measurement::ModelEx> =
        dispatch_storage_measurement::Entity::load()
          .filter_by_id(record_id)
          .one(conn)
          .await?;
      match measurement {
        Some(measurement) => measurement.dispatch_doc_id,
        None => return Ok(vec![]),
      }
    }
  };

  resolve_dispatch_doc_bases(conn, doc_id).await
}

#[cfg(test)]
mod tests {
  use super::{
    AuditRouteProfile,
    AuditTable,
    AuditTableCategory,
    DocumentHeaderRouteKind,
    ReferenceRouteKind,
    StorageItemRouteKind,
    TransportRouteKind,
  };
  use crate::entities::{dispatch_document, local};

  #[test]
  fn audit_tables_use_canonical_entity_names() {
    assert_eq!(
      AuditTable::for_model::<dispatch_document::Model>(),
      Some(AuditTable::DispatchDocuments)
    );
    assert_eq!(
      AuditTable::DispatchDocuments.table_name(),
      "dispatch_documents"
    );

    assert_eq!(
      AuditTable::for_model::<local::Model>(),
      Some(AuditTable::Local)
    );
    assert_eq!(AuditTable::for_entity_name("unknown_table"), None);
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
}
