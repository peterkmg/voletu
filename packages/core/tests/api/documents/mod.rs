use std::sync::Arc;

use uuid::Uuid;

use crate::common::catalog_seed::seed_inventory_catalog;

pub(crate) struct InventoryContext {
  pub contractor_id: Uuid,
  pub product_id: Uuid,
  pub second_product_id: Uuid,
  pub base_id: Uuid,
  pub storage_a: Uuid,
  pub storage_b: Uuid,
  pub warehouse_id: Uuid,
}

pub(crate) async fn seed_inventory_context(
  db: &Arc<sea_orm::DatabaseConnection>,
) -> InventoryContext {
  let catalog = seed_inventory_catalog(db).await;
  InventoryContext {
    contractor_id: catalog.contractor_a_id,
    product_id: catalog.product_a_id,
    second_product_id: catalog.product_b_id,
    base_id: catalog.base_id,
    storage_a: catalog.storage_a_id,
    storage_b: catalog.storage_b_id,
    warehouse_id: catalog.warehouse_id,
  }
}

mod acceptance;
mod blending;
mod dispatch;
mod operations;
mod pagination;
mod query;
mod reconciliation;
mod transfers;
mod validation;
