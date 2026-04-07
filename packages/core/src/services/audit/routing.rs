//! Routing envelope resolver — determines which bases are affected by an entity change.
//!
//! Called during audit log registration to populate `target_base_ids`.
//! Resolution chain: entity → storage_id → storage.warehouse_id → warehouse.base_id

use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use serde_json::Value;
use uuid::Uuid;

use crate::{
  api::ApiError,
  entities::{
    acceptance_item,
    blending_component,
    blending_result,
    dispatch_document,
    dispatch_item,
    dispatch_storage_measurement,
    inventory_reconciliation,
    ownership_transfer_item,
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
  enums::RoleType,
};

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Resolve the target base IDs for a given entity change.
/// Returns empty vec for global/broadcast tables (catalog, system).
pub async fn resolve_target_base_ids<C: ConnectionTrait>(
  conn: &C,
  table_name: &str,
  record_id: Uuid,
  new_values: Option<&Value>,
) -> Result<Vec<Uuid>, ApiError> {
  match table_name {
    // --- Item tables: resolve via storage_id ---
    "acceptance_items" => resolve_via_storage(conn, new_values, record_id, "storage_id").await,
    "dispatch_items" => resolve_via_storage(conn, new_values, record_id, "storage_id").await,
    "blending_components" => resolve_via_storage(conn, new_values, record_id, "storage_id").await,
    "blending_results" => resolve_via_storage(conn, new_values, record_id, "storage_id").await,
    "ownership_transfer_items" => {
      resolve_via_storage(conn, new_values, record_id, "storage_id").await
    }
    "inventory_adjustments" => resolve_via_storage(conn, new_values, record_id, "storage_id").await,

    // --- Physical transfer items: TWO storage references ---
    "physical_transfer_items" => {
      let mut bases = Vec::new();
      bases.extend(resolve_via_storage(conn, new_values, record_id, "from_storage_id").await?);
      bases.extend(resolve_via_storage(conn, new_values, record_id, "to_storage_id").await?);
      bases.sort();
      bases.dedup();
      Ok(bases)
    }

    // --- Document headers: resolve via their items ---
    "acceptance_documents" => resolve_acceptance_doc_bases(conn, record_id).await,
    "dispatch_documents" => resolve_dispatch_doc_bases(conn, record_id).await,
    "physical_storage_transfers" => resolve_physical_transfer_doc_bases(conn, record_id).await,
    "ownership_transfers" => resolve_ownership_doc_bases(conn, record_id).await,
    "blending_documents" => resolve_blending_doc_bases(conn, record_id).await,

    // --- Reconciliation: direct warehouse_id on document ---
    "inventory_reconciliations" => resolve_via_warehouse(conn, new_values, record_id).await,

    // --- Reference tables ---
    "storages" => {
      let wh_id = extract_uuid_field(new_values, "warehouse_id");
      if let Some(wh_id) = wh_id {
        resolve_warehouse_to_base(conn, wh_id).await
      } else {
        // Fallback: load from DB
        let s = storage::Entity::find_by_id(record_id).one(conn).await?;
        match s {
          Some(s) => resolve_warehouse_to_base(conn, s.warehouse_id).await,
          None => Ok(vec![]),
        }
      }
    }
    "warehouses" => {
      let base_id = extract_uuid_field(new_values, "base_id");
      if let Some(base_id) = base_id {
        Ok(vec![base_id])
      } else {
        let w = warehouse::Entity::find_by_id(record_id).one(conn).await?;
        Ok(w.map(|w| vec![w.base_id]).unwrap_or_default())
      }
    }
    "bases" => Ok(vec![record_id]),

    // --- Transport waybills: route via base_id on the waybill ---
    "truck_waybills" => {
      resolve_via_base_id(conn, new_values, record_id, |id| async move {
        let w = truck_waybill::Entity::find_by_id(id).one(conn).await?;
        Ok(w.map(|w| w.base_id))
      })
      .await
    }
    "rail_waybills" => {
      resolve_via_base_id(conn, new_values, record_id, |id| async move {
        let w = rail_waybill::Entity::find_by_id(id).one(conn).await?;
        Ok(w.map(|w| w.base_id))
      })
      .await
    }

    // --- Waybill children: resolve via parent waybill's base_id ---
    "truck_waybill_items" => {
      resolve_truck_child_base(conn, new_values, record_id, "truck_waybill_id").await
    }
    "truck_weight_docs" => {
      resolve_truck_child_base(conn, new_values, record_id, "truck_waybill_id").await
    }
    "rail_wagon_manifests" => resolve_rail_manifest_base(conn, new_values, record_id).await,
    "rail_wagon_measurements" => resolve_rail_nested_base(conn, record_id).await,
    "rail_wagon_weights" => resolve_rail_nested_base(conn, record_id).await,

    // --- Dispatch measurements: inherit from parent dispatch document ---
    "dispatch_storage_measurements" => {
      resolve_dispatch_measurement_base(conn, new_values, record_id).await
    }

    // --- Ledger entries: resolve via storage_id ---
    "inventory_ledger_entries" => {
      resolve_via_storage(conn, new_values, record_id, "storage_id").await
    }

    // --- Global tables (catalog, system): broadcast to all ---
    _ => Ok(vec![]),
  }
}

/// Resolve the role weight for an actor (user) for conflict resolution.
pub async fn resolve_role_weight<C: ConnectionTrait>(
  conn: &C,
  actor_id: Uuid,
) -> Result<i32, ApiError> {
  let user_with_role = user::Entity::find_by_id(actor_id)
    .find_also_related(role::Entity)
    .one(conn)
    .await?;

  let weight = match user_with_role {
    Some((_, Some(role))) => match role.common_name {
      RoleType::Admin => 100,
      RoleType::SeniorSupervisor => 40,
      RoleType::Supervisor => 10,
      RoleType::Operator => 1,
    },
    _ => 0,
  };

  Ok(weight)
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

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
  let s = storage::Entity::find_by_id(storage_id).one(conn).await?;
  match s {
    Some(s) => {
      let w = warehouse::Entity::find_by_id(s.warehouse_id)
        .one(conn)
        .await?;
      Ok(w.map(|w| w.base_id))
    }
    None => Ok(None),
  }
}

/// Resolve multiple storage_ids → base_ids (deduped).
async fn resolve_storages_to_bases<C: ConnectionTrait>(
  conn: &C,
  storage_ids: &[Uuid],
) -> Result<Vec<Uuid>, ApiError> {
  if storage_ids.is_empty() {
    return Ok(vec![]);
  }

  let storages = storage::Entity::find()
    .filter(storage::Column::Id.is_in(storage_ids.iter().copied()))
    .all(conn)
    .await?;

  let warehouse_ids: Vec<Uuid> = storages.iter().map(|s| s.warehouse_id).collect();
  if warehouse_ids.is_empty() {
    return Ok(vec![]);
  }

  let warehouses = warehouse::Entity::find()
    .filter(warehouse::Column::Id.is_in(warehouse_ids))
    .all(conn)
    .await?;

  let mut base_ids: Vec<Uuid> = warehouses.iter().map(|w| w.base_id).collect();
  base_ids.sort();
  base_ids.dedup();
  Ok(base_ids)
}

/// Resolve via a single storage FK field on the entity.
async fn resolve_via_storage<C: ConnectionTrait>(
  conn: &C,
  new_values: Option<&Value>,
  _record_id: Uuid,
  storage_field: &str,
) -> Result<Vec<Uuid>, ApiError> {
  let storage_id = extract_uuid_field(new_values, storage_field);
  if let Some(sid) = storage_id {
    match resolve_storage_to_base(conn, sid).await? {
      Some(base_id) => Ok(vec![base_id]),
      None => Ok(vec![]),
    }
  } else {
    // No new_values (update/delete) — can't resolve without loading entity.
    // For items, the audit log from creation already has routing.
    // Updates/deletes inherit the same routing scope.
    Ok(vec![])
  }
}

/// Resolve reconciliation via warehouse_id on the document.
async fn resolve_via_warehouse<C: ConnectionTrait>(
  conn: &C,
  new_values: Option<&Value>,
  record_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  let wh_id = extract_uuid_field(new_values, "warehouse_id");
  if let Some(wh_id) = wh_id {
    return resolve_warehouse_to_base(conn, wh_id).await;
  }
  // Fallback: load from DB
  let rec = inventory_reconciliation::Entity::find_by_id(record_id)
    .one(conn)
    .await?;
  match rec {
    Some(r) => resolve_warehouse_to_base(conn, r.warehouse_id).await,
    None => Ok(vec![]),
  }
}

async fn resolve_warehouse_to_base<C: ConnectionTrait>(
  conn: &C,
  warehouse_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  let w = warehouse::Entity::find_by_id(warehouse_id)
    .one(conn)
    .await?;
  Ok(w.map(|w| vec![w.base_id]).unwrap_or_default())
}

// ---------------------------------------------------------------------------
// Document header resolvers — load items, collect storage_ids, resolve bases
// ---------------------------------------------------------------------------

async fn resolve_acceptance_doc_bases<C: ConnectionTrait>(
  conn: &C,
  doc_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  let items = acceptance_item::Entity::find()
    .filter(acceptance_item::Column::AcceptanceDocId.eq(doc_id))
    .all(conn)
    .await?;
  let storage_ids: Vec<Uuid> = items.iter().map(|i| i.storage_id).collect();
  resolve_storages_to_bases(conn, &storage_ids).await
}

async fn resolve_dispatch_doc_bases<C: ConnectionTrait>(
  conn: &C,
  doc_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  let items = dispatch_item::Entity::find()
    .filter(dispatch_item::Column::DispatchDocId.eq(doc_id))
    .all(conn)
    .await?;
  let storage_ids: Vec<Uuid> = items.iter().map(|i| i.storage_id).collect();
  let mut bases = resolve_storages_to_bases(conn, &storage_ids).await?;

  // Also include destination_base_id if set
  let doc = dispatch_document::Entity::find_by_id(doc_id)
    .one(conn)
    .await?;
  if let Some(doc) = doc {
    if let Some(dest_base) = doc.destination_base_id {
      bases.push(dest_base);
      bases.sort();
      bases.dedup();
    }
  }

  Ok(bases)
}

async fn resolve_physical_transfer_doc_bases<C: ConnectionTrait>(
  conn: &C,
  doc_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  let items = physical_transfer_item::Entity::find()
    .filter(physical_transfer_item::Column::PhysicalTransferId.eq(doc_id))
    .all(conn)
    .await?;
  let mut storage_ids: Vec<Uuid> = Vec::new();
  for item in &items {
    storage_ids.push(item.from_storage_id);
    storage_ids.push(item.to_storage_id);
  }
  resolve_storages_to_bases(conn, &storage_ids).await
}

async fn resolve_ownership_doc_bases<C: ConnectionTrait>(
  conn: &C,
  doc_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  let items = ownership_transfer_item::Entity::find()
    .filter(ownership_transfer_item::Column::OwnershipTransferId.eq(doc_id))
    .all(conn)
    .await?;
  let storage_ids: Vec<Uuid> = items.iter().map(|i| i.storage_id).collect();
  resolve_storages_to_bases(conn, &storage_ids).await
}

async fn resolve_blending_doc_bases<C: ConnectionTrait>(
  conn: &C,
  doc_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  let components = blending_component::Entity::find()
    .filter(blending_component::Column::BlendingDocId.eq(doc_id))
    .all(conn)
    .await?;
  let results = blending_result::Entity::find()
    .filter(blending_result::Column::BlendingDocId.eq(doc_id))
    .all(conn)
    .await?;

  let mut storage_ids: Vec<Uuid> = Vec::new();
  for c in &components {
    storage_ids.push(c.storage_id);
  }
  for r in &results {
    storage_ids.push(r.storage_id);
  }

  resolve_storages_to_bases(conn, &storage_ids).await
}

// ---------------------------------------------------------------------------
// Waybill + measurement routing helpers
// ---------------------------------------------------------------------------

/// Resolve routing via a direct `base_id` field on the entity.
async fn resolve_via_base_id<C, Fut>(
  _conn: &C,
  new_values: Option<&Value>,
  record_id: Uuid,
  fallback: impl FnOnce(Uuid) -> Fut,
) -> Result<Vec<Uuid>, ApiError>
where
  C: ConnectionTrait,
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

/// Resolve truck waybill child (item or weight_doc) → parent waybill → base_id.
async fn resolve_truck_child_base<C: ConnectionTrait>(
  conn: &C,
  new_values: Option<&Value>,
  record_id: Uuid,
  parent_fk_field: &str,
) -> Result<Vec<Uuid>, ApiError> {
  let waybill_id = match extract_uuid_field(new_values, parent_fk_field) {
    Some(id) => id,
    None => {
      if let Some(item) = truck_waybill_item::Entity::find_by_id(record_id)
        .one(conn)
        .await?
      {
        item.truck_waybill_id
      } else if let Some(doc) = truck_weight_doc::Entity::find_by_id(record_id)
        .one(conn)
        .await?
      {
        doc.truck_waybill_id
      } else {
        return Ok(vec![]);
      }
    }
  };
  let waybill = truck_waybill::Entity::find_by_id(waybill_id)
    .one(conn)
    .await?;
  Ok(waybill.map(|w| vec![w.base_id]).unwrap_or_default())
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
      let manifest = rail_wagon_manifest::Entity::find_by_id(record_id)
        .one(conn)
        .await?;
      match manifest {
        Some(m) => m.rail_waybill_id,
        None => return Ok(vec![]),
      }
    }
  };
  let waybill = rail_waybill::Entity::find_by_id(waybill_id)
    .one(conn)
    .await?;
  Ok(waybill.map(|w| vec![w.base_id]).unwrap_or_default())
}

/// Resolve rail wagon measurement/weight → manifest → waybill → base_id.
async fn resolve_rail_nested_base(
  conn: &impl ConnectionTrait,
  record_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  // Try measurement first, then weight
  if let Some(m) = rail_wagon_measurement::Entity::find_by_id(record_id)
    .one(conn)
    .await?
  {
    let manifest = rail_wagon_manifest::Entity::find_by_id(m.wagon_manifest_id)
      .one(conn)
      .await?;
    if let Some(manifest) = manifest {
      let waybill = rail_waybill::Entity::find_by_id(manifest.rail_waybill_id)
        .one(conn)
        .await?;
      return Ok(waybill.map(|w| vec![w.base_id]).unwrap_or_default());
    }
  }
  if let Some(w) = rail_wagon_weight::Entity::find_by_id(record_id)
    .one(conn)
    .await?
  {
    let manifest = rail_wagon_manifest::Entity::find_by_id(w.wagon_manifest_id)
      .one(conn)
      .await?;
    if let Some(manifest) = manifest {
      let waybill = rail_waybill::Entity::find_by_id(manifest.rail_waybill_id)
        .one(conn)
        .await?;
      return Ok(waybill.map(|w| vec![w.base_id]).unwrap_or_default());
    }
  }
  Ok(vec![])
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
      let m = dispatch_storage_measurement::Entity::find_by_id(record_id)
        .one(conn)
        .await?;
      match m {
        Some(m) => m.dispatch_doc_id,
        None => return Ok(vec![]),
      }
    }
  };
  resolve_dispatch_doc_bases(conn, doc_id).await
}
