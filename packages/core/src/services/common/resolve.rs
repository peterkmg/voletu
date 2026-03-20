use std::collections::{HashMap, HashSet};

use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::api::ApiError;
use crate::entities::{
  base, company, dispatch_document, port, product, product_group, product_type, rail_waybill,
  storage, truck_waybill, warehouse,
};

/// Collects FK UUIDs by target entity type.
#[derive(Default)]
pub struct FkIdCollector {
  pub company_ids: HashSet<Uuid>,
  pub product_ids: HashSet<Uuid>,
  pub product_group_ids: HashSet<Uuid>,
  pub product_type_ids: HashSet<Uuid>,
  pub base_ids: HashSet<Uuid>,
  pub warehouse_ids: HashSet<Uuid>,
  pub storage_ids: HashSet<Uuid>,
  pub port_ids: HashSet<Uuid>,
  pub truck_waybill_ids: HashSet<Uuid>,
  pub rail_waybill_ids: HashSet<Uuid>,
  pub dispatch_document_ids: HashSet<Uuid>,
}

/// Resolved name maps keyed by entity ID.
#[derive(Default)]
pub struct ResolvedNames {
  pub companies: HashMap<Uuid, String>,
  pub products: HashMap<Uuid, String>,
  pub product_groups: HashMap<Uuid, String>,
  pub product_types: HashMap<Uuid, String>,
  pub bases: HashMap<Uuid, String>,
  pub warehouses: HashMap<Uuid, String>,
  pub storages: HashMap<Uuid, String>,
  pub ports: HashMap<Uuid, String>,
  pub truck_waybills: HashMap<Uuid, String>,
  pub rail_waybills: HashMap<Uuid, String>,
  pub dispatch_documents: HashMap<Uuid, String>,
}

impl ResolvedNames {
  pub async fn fetch(
    db: &impl ConnectionTrait,
    collector: &FkIdCollector,
  ) -> Result<Self, ApiError> {
    let mut resolved = Self::default();

    if !collector.company_ids.is_empty() {
      let rows = company::Entity::find()
        .filter(company::Column::Id.is_in(collector.company_ids.iter().copied()))
        .all(db)
        .await?;
      for r in rows {
        resolved.companies.insert(r.id, r.common_name);
      }
    }

    if !collector.product_ids.is_empty() {
      let rows = product::Entity::find()
        .filter(product::Column::Id.is_in(collector.product_ids.iter().copied()))
        .all(db)
        .await?;
      for r in rows {
        resolved.products.insert(r.id, r.common_name);
      }
    }

    if !collector.product_group_ids.is_empty() {
      let rows = product_group::Entity::find()
        .filter(product_group::Column::Id.is_in(collector.product_group_ids.iter().copied()))
        .all(db)
        .await?;
      for r in rows {
        resolved.product_groups.insert(r.id, r.common_name);
      }
    }

    if !collector.product_type_ids.is_empty() {
      let rows = product_type::Entity::find()
        .filter(product_type::Column::Id.is_in(collector.product_type_ids.iter().copied()))
        .all(db)
        .await?;
      for r in rows {
        resolved.product_types.insert(r.id, r.common_name);
      }
    }

    if !collector.base_ids.is_empty() {
      let rows = base::Entity::find()
        .filter(base::Column::Id.is_in(collector.base_ids.iter().copied()))
        .all(db)
        .await?;
      for r in rows {
        resolved.bases.insert(r.id, r.common_name);
      }
    }

    if !collector.warehouse_ids.is_empty() {
      let rows = warehouse::Entity::find()
        .filter(warehouse::Column::Id.is_in(collector.warehouse_ids.iter().copied()))
        .all(db)
        .await?;
      for r in rows {
        resolved.warehouses.insert(r.id, r.common_name);
      }
    }

    if !collector.storage_ids.is_empty() {
      let rows = storage::Entity::find()
        .filter(storage::Column::Id.is_in(collector.storage_ids.iter().copied()))
        .all(db)
        .await?;
      for r in rows {
        resolved.storages.insert(r.id, r.common_name);
      }
    }

    if !collector.port_ids.is_empty() {
      let rows = port::Entity::find()
        .filter(port::Column::Id.is_in(collector.port_ids.iter().copied()))
        .all(db)
        .await?;
      for r in rows {
        resolved.ports.insert(r.id, r.common_name);
      }
    }

    if !collector.truck_waybill_ids.is_empty() {
      let rows = truck_waybill::Entity::find()
        .filter(truck_waybill::Column::Id.is_in(collector.truck_waybill_ids.iter().copied()))
        .all(db)
        .await?;
      for r in rows {
        resolved.truck_waybills.insert(r.id, r.document_number);
      }
    }

    if !collector.rail_waybill_ids.is_empty() {
      let rows = rail_waybill::Entity::find()
        .filter(rail_waybill::Column::Id.is_in(collector.rail_waybill_ids.iter().copied()))
        .all(db)
        .await?;
      for r in rows {
        resolved.rail_waybills.insert(r.id, r.document_number);
      }
    }

    if !collector.dispatch_document_ids.is_empty() {
      let rows = dispatch_document::Entity::find()
        .filter(
          dispatch_document::Column::Id
            .is_in(collector.dispatch_document_ids.iter().copied()),
        )
        .all(db)
        .await?;
      for r in rows {
        resolved
          .dispatch_documents
          .insert(r.id, r.document_number);
      }
    }

    Ok(resolved)
  }
}

/// Trait for DTOs that can declare and receive resolved FK names.
pub trait ResolveFkNames {
  fn collect_fk_ids(&self, collector: &mut FkIdCollector);
  fn apply_resolved_names(&mut self, resolved: &ResolvedNames);
}

/// Resolve FK names for a slice of DTOs by batch-fetching referenced names.
pub async fn resolve_names<T: ResolveFkNames>(
  db: &impl ConnectionTrait,
  items: &mut [T],
) -> Result<(), ApiError> {
  let mut collector = FkIdCollector::default();
  for item in items.iter() {
    item.collect_fk_ids(&mut collector);
  }
  let resolved = ResolvedNames::fetch(db, &collector).await?;
  for item in items.iter_mut() {
    item.apply_resolved_names(&resolved);
  }
  Ok(())
}
