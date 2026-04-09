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
    "acceptance_items" => resolve_via_storage(conn, new_values, "storage_id").await,
    "dispatch_items" => resolve_via_storage(conn, new_values, "storage_id").await,
    "blending_components" => resolve_via_storage(conn, new_values, "storage_id").await,
    "blending_results" => resolve_via_storage(conn, new_values, "storage_id").await,
    "ownership_transfer_items" => resolve_via_storage(conn, new_values, "storage_id").await,
    "inventory_adjustments" => resolve_via_storage(conn, new_values, "storage_id").await,

    // --- Physical transfer items: TWO storage references ---
    "physical_transfer_items" => {
      let mut bases = Vec::new();
      bases.extend(resolve_via_storage(conn, new_values, "from_storage_id").await?);
      bases.extend(resolve_via_storage(conn, new_values, "to_storage_id").await?);
      Ok(dedupe_base_ids(bases))
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
      if let Some(warehouse_id) = extract_uuid_field(new_values, "warehouse_id") {
        resolve_warehouse_to_base(conn, warehouse_id).await
      } else {
        match resolve_storage_to_base(conn, record_id).await? {
          Some(base_id) => Ok(vec![base_id]),
          None => Ok(vec![]),
        }
      }
    }
    "warehouses" => {
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
    "bases" => Ok(vec![record_id]),

    // --- Transport waybills: route via base_id on the waybill ---
    "truck_waybills" => {
      resolve_via_base_id(new_values, record_id, |id| async move {
        let waybill: Option<truck_waybill::ModelEx> = truck_waybill::Entity::load()
          .filter_by_id(id)
          .one(conn)
          .await?;
        Ok(waybill.map(|waybill| waybill.base_id))
      })
      .await
    }
    "rail_waybills" => {
      resolve_via_base_id(new_values, record_id, |id| async move {
        let waybill: Option<rail_waybill::ModelEx> = rail_waybill::Entity::load()
          .filter_by_id(id)
          .one(conn)
          .await?;
        Ok(waybill.map(|waybill| waybill.base_id))
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
    "inventory_ledger_entries" => resolve_via_storage(conn, new_values, "storage_id").await,

    // --- Global tables (catalog, system): broadcast to all ---
    _ => Ok(vec![]),
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
  let waybill: Option<truck_waybill::ModelEx> = truck_waybill::Entity::load()
    .filter_by_id(waybill_id)
    .one(conn)
    .await?;

  Ok(
    waybill
      .map(|waybill| vec![waybill.base_id])
      .unwrap_or_default(),
  )
}

async fn resolve_rail_waybill_base<C: ConnectionTrait>(
  conn: &C,
  waybill_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  let waybill: Option<rail_waybill::ModelEx> = rail_waybill::Entity::load()
    .filter_by_id(waybill_id)
    .one(conn)
    .await?;

  Ok(
    waybill
      .map(|waybill| vec![waybill.base_id])
      .unwrap_or_default(),
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
      if let Some(item) = truck_waybill_item::Entity::load()
        .filter_by_id(record_id)
        .one(conn)
        .await?
      {
        item.truck_waybill_id
      } else if let Some(weight_doc) = truck_weight_doc::Entity::load()
        .filter_by_id(record_id)
        .one(conn)
        .await?
      {
        weight_doc.truck_waybill_id
      } else {
        return Ok(vec![]);
      }
    }
  };

  resolve_truck_waybill_base(conn, waybill_id).await
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

/// Resolve rail wagon measurement/weight → manifest → waybill → base_id.
async fn resolve_rail_nested_base(
  conn: &impl ConnectionTrait,
  record_id: Uuid,
) -> Result<Vec<Uuid>, ApiError> {
  if let Some(measurement) = rail_wagon_measurement::Entity::load()
    .filter_by_id(record_id)
    .one(conn)
    .await?
  {
    return resolve_manifest_waybill_base(conn, measurement.wagon_manifest_id).await;
  }

  if let Some(weight) = rail_wagon_weight::Entity::load()
    .filter_by_id(record_id)
    .one(conn)
    .await?
  {
    return resolve_manifest_waybill_base(conn, weight.wagon_manifest_id).await;
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
