use std::sync::Arc;

use sea_orm::EntityTrait;
use voletu_core::{
  db::seed_defaults,
  entities::{
    acceptance_document,
    acceptance_item,
    base,
    blending_document,
    blending_result,
    dispatch_document,
    dispatch_item,
    inventory_ledger_entry,
    product_type,
    rail_wagon_manifest,
    rail_waybill,
    storage,
    truck_waybill,
    truck_waybill_item,
    warehouse,
  },
  services::SystemService,
};

use crate::common::{setup_db, test_config};

#[tokio::test]
async fn is_additive_across_repeated_runs() {
  let db = Arc::new(setup_db().await);
  let local = seed_defaults(&db).await.unwrap();
  let mut cfg = test_config();
  cfg.node.db_id = local.local_db_id;
  let service = SystemService::new(db, Arc::new(cfg));

  for iteration in 0..3 {
    service
      .dev_seed()
      .await
      .unwrap_or_else(|error| panic!("dev_seed run {iteration} failed: {error:?}"));
  }
}

#[tokio::test]
async fn uses_clean_minimal_display_names_across_repeated_runs() {
  let db = Arc::new(setup_db().await);
  let local = seed_defaults(&db).await.unwrap();
  let mut cfg = test_config();
  cfg.node.db_id = local.local_db_id;
  let service = SystemService::new(db.clone(), Arc::new(cfg));

  service.dev_seed().await.unwrap();
  service.dev_seed().await.unwrap();

  let product_types = product_type::Entity::find().all(db.as_ref()).await.unwrap();
  let names = product_types
    .iter()
    .map(|product_type| product_type.common_name.as_str())
    .collect::<Vec<_>>();

  assert!(names.contains(&"Crude Oil"));
  assert!(names.contains(&"Crude Oil 2"));
  assert!(
    names
      .iter()
      .all(|name| !has_legacy_seed_entropy_suffix(name)),
    "seeded names should not expose random three-digit and run hash suffixes: {names:?}"
  );
}

#[tokio::test]
async fn persists_nested_reference_and_document_graphs() {
  let db = Arc::new(setup_db().await);
  let local = seed_defaults(&db).await.unwrap();
  let mut cfg = test_config();
  cfg.node.db_id = local.local_db_id;
  let service = SystemService::new(db.clone(), Arc::new(cfg));

  service.dev_seed().await.unwrap();

  let bases = base::Entity::load()
    .with(warehouse::Entity)
    .with((warehouse::Entity, storage::Entity))
    .all(db.as_ref())
    .await
    .unwrap();
  assert!(bases.iter().any(|base| {
    !base.warehouses.is_empty()
      && base
        .warehouses
        .iter()
        .any(|warehouse| !warehouse.storages.is_empty())
  }));

  let truck_waybills = truck_waybill::Entity::load()
    .with(truck_waybill_item::Entity)
    .all(db.as_ref())
    .await
    .unwrap();
  assert!(truck_waybills
    .iter()
    .any(|waybill| !waybill.items.is_empty()));

  let rail_waybills = rail_waybill::Entity::load()
    .with(rail_wagon_manifest::Entity)
    .all(db.as_ref())
    .await
    .unwrap();
  assert!(rail_waybills
    .iter()
    .any(|waybill| !waybill.wagon_manifests.is_empty()));

  let dispatches = dispatch_document::Entity::load()
    .with(dispatch_item::Entity)
    .all(db.as_ref())
    .await
    .unwrap();
  assert!(dispatches.iter().any(|document| !document.items.is_empty()));

  let acceptances = acceptance_document::Entity::load()
    .with(acceptance_item::Entity)
    .all(db.as_ref())
    .await
    .unwrap();
  assert!(acceptances
    .iter()
    .any(|document| !document.items.is_empty()));

  let blends = blending_document::Entity::load()
    .with(blending_result::Entity)
    .all(db.as_ref())
    .await
    .unwrap();
  assert!(blends.iter().any(|document| !document.results.is_empty()));

  let ledger_entries = inventory_ledger_entry::Entity::load()
    .all(db.as_ref())
    .await
    .unwrap();
  assert!(!ledger_entries.is_empty());
}

fn has_legacy_seed_entropy_suffix(name: &str) -> bool {
  let mut parts = name.split_whitespace().rev();
  let Some(run_hash) = parts.next() else {
    return false;
  };
  let Some(random_number) = parts.next() else {
    return false;
  };

  run_hash.len() == 8
    && run_hash.chars().all(|ch| ch.is_ascii_hexdigit())
    && random_number.len() == 3
    && random_number.chars().all(|ch| ch.is_ascii_digit())
}
