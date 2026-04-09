use sea_orm::{ActiveValue::Set, EntityLoaderTrait, IntoActiveModel};
use uuid::Uuid;
use voletu_core::{
  context::audit::with_audit_context,
  entities::{base, storage, warehouse},
};

use crate::common::setup_db;

#[tokio::test]
async fn active_model_ex_save_persists_nested_base_warehouse_storage_graph() {
  let db = setup_db().await;

  let graph = base::ActiveModelEx {
    common_name: Set("Spike Base".to_string()),
    long_name: Set(Some("Spike Base Long".to_string())),
    warehouses: vec![warehouse::ActiveModelEx {
      common_name: Set("Spike Warehouse".to_string()),
      long_name: Set(None),
      storages: vec![storage::ActiveModelEx {
        common_name: Set("Spike Tank".to_string()),
        long_name: Set(None),
        capacity: Set(None),
        is_type_specific: Set(false),
        product_type_id: Set(None),
        ..Default::default()
      }]
      .into(),
      ..Default::default()
    }]
    .into(),
    ..Default::default()
  };

  let saved = with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    graph.save(&db).await.unwrap()
  })
  .await;
  let saved_base_id = saved.id.unwrap();

  let loaded = base::Entity::load()
    .filter_by_id(saved_base_id)
    .with(warehouse::Entity)
    .with((warehouse::Entity, storage::Entity))
    .one(&db)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(loaded.common_name, "Spike Base");
  assert_eq!(loaded.warehouses.len(), 1);
  assert_eq!(loaded.warehouses[0].common_name, "Spike Warehouse");
  assert_eq!(loaded.warehouses[0].storages.len(), 1);
  assert_eq!(loaded.warehouses[0].storages[0].common_name, "Spike Tank");
  assert_eq!(loaded.warehouses[0].base_id, saved_base_id);
  assert_eq!(
    loaded.warehouses[0].storages[0].warehouse_id,
    loaded.warehouses[0].id
  );
}

#[tokio::test]
async fn model_ex_into_active_model_can_append_related_storage_and_save_it() {
  let db = setup_db().await;

  let saved = with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    base::ActiveModelEx {
      common_name: Set("Reload Base".to_string()),
      long_name: Set(None),
      warehouses: vec![warehouse::ActiveModelEx {
        common_name: Set("Reload Warehouse".to_string()),
        long_name: Set(None),
        storages: vec![storage::ActiveModelEx {
          common_name: Set("Existing Tank".to_string()),
          long_name: Set(None),
          capacity: Set(None),
          is_type_specific: Set(false),
          product_type_id: Set(None),
          ..Default::default()
        }]
        .into(),
        ..Default::default()
      }]
      .into(),
      ..Default::default()
    }
    .save(&db)
    .await
    .unwrap()
  })
  .await;
  let saved_base_id = saved.id.clone().unwrap();

  let loaded = base::Entity::load()
    .filter_by_id(saved_base_id)
    .with(warehouse::Entity)
    .with((warehouse::Entity, storage::Entity))
    .one(&db)
    .await
    .unwrap()
    .unwrap();

  let mut active: base::ActiveModelEx = loaded.into_active_model();
  active.warehouses[0]
    .storages
    .append(storage::ActiveModelEx {
      common_name: Set("Added Tank".to_string()),
      long_name: Set(None),
      capacity: Set(None),
      is_type_specific: Set(false),
      product_type_id: Set(None),
      ..Default::default()
    });

  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    active.save(&db).await.unwrap()
  })
  .await;

  let reloaded = base::Entity::load()
    .filter_by_id(saved_base_id)
    .with(warehouse::Entity)
    .with((warehouse::Entity, storage::Entity))
    .one(&db)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(reloaded.warehouses.len(), 1);
  assert_eq!(reloaded.warehouses[0].storages.len(), 2);
  assert!(reloaded.warehouses[0]
    .storages
    .iter()
    .any(|storage| storage.common_name == "Added Tank"));
}
