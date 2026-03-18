use std::sync::Arc;

use sea_orm::{prelude::Decimal, ActiveModelTrait, ActiveValue::Set};
use uuid::Uuid;
use voletu_core::{
  context::audit::with_audit_context,
  entities::{
    base,
    company,
    database_instance,
    inventory_ledger_entry,
    product,
    product_group,
    product_type,
    storage,
    warehouse,
  },
  enums,
};

#[allow(dead_code)]
pub struct InventoryFixture {
  pub contractor_a_id: Uuid,
  pub contractor_b_id: Uuid,
  pub sender_id: Uuid,
  pub product_type_id: Uuid,
  pub product_group_id: Uuid,
  pub product_a_id: Uuid,
  pub product_b_id: Uuid,
  pub base_id: Uuid,
  pub warehouse_id: Uuid,
  pub storage_a_id: Uuid,
  pub storage_b_id: Uuid,
}

pub async fn seed_inventory_fixture(db: &Arc<sea_orm::DatabaseConnection>) -> InventoryFixture {
  let actor_id = Uuid::now_v7();
  let origin_db_id = Uuid::now_v7();

  with_audit_context(actor_id, origin_db_id, || async {
    let contractor_a = company::ActiveModel {
      common_name: Set("Contractor A".to_string()),
      legal_name: Set(None),
      is_contractor: Set(true),
      is_exporter: Set(false),
      is_manufacturer: Set(false),
      is_sender: Set(false),
      ..Default::default()
    }
    .insert(&**db)
    .await
    .unwrap();
    let contractor_b = company::ActiveModel {
      common_name: Set("Contractor B".to_string()),
      legal_name: Set(None),
      is_contractor: Set(true),
      is_exporter: Set(false),
      is_manufacturer: Set(false),
      is_sender: Set(false),
      ..Default::default()
    }
    .insert(&**db)
    .await
    .unwrap();
    let sender = company::ActiveModel {
      common_name: Set("Sender Co".to_string()),
      legal_name: Set(None),
      is_contractor: Set(false),
      is_exporter: Set(false),
      is_manufacturer: Set(false),
      is_sender: Set(true),
      ..Default::default()
    }
    .insert(&**db)
    .await
    .unwrap();

    let ptype = product_type::ActiveModel {
      common_name: Set("Fuel".to_string()),
      long_name: Set(None),
      ..Default::default()
    }
    .insert(&**db)
    .await
    .unwrap();
    let pgroup = product_group::ActiveModel {
      product_type_id: Set(ptype.id),
      common_name: Set("Diesel".to_string()),
      long_name: Set(None),
      ..Default::default()
    }
    .insert(&**db)
    .await
    .unwrap();
    let product_a = product::ActiveModel {
      product_group_id: Set(pgroup.id),
      manufacturer_id: Set(None),
      common_name: Set("Product A".to_string()),
      long_name: Set(None),
      add_identification: Set(None),
      is_component: Set(true),
      ..Default::default()
    }
    .insert(&**db)
    .await
    .unwrap();
    let product_b = product::ActiveModel {
      product_group_id: Set(pgroup.id),
      manufacturer_id: Set(None),
      common_name: Set("Product B".to_string()),
      long_name: Set(None),
      add_identification: Set(None),
      is_component: Set(false),
      ..Default::default()
    }
    .insert(&**db)
    .await
    .unwrap();

    let b = base::ActiveModel {
      common_name: Set("Base A".to_string()),
      long_name: Set(None),
      ..Default::default()
    }
    .insert(&**db)
    .await
    .unwrap();
    let wh = warehouse::ActiveModel {
      base_id: Set(b.id),
      common_name: Set("WH 1".to_string()),
      long_name: Set(None),
      ..Default::default()
    }
    .insert(&**db)
    .await
    .unwrap();
    let storage_a = storage::ActiveModel {
      warehouse_id: Set(wh.id),
      common_name: Set("Tank A".to_string()),
      long_name: Set(None),
      capacity: Set(None),
      is_type_specific: Set(false),
      product_type_id: Set(None),
      ..Default::default()
    }
    .insert(&**db)
    .await
    .unwrap();
    let storage_b = storage::ActiveModel {
      warehouse_id: Set(wh.id),
      common_name: Set("Tank B".to_string()),
      long_name: Set(None),
      capacity: Set(None),
      is_type_specific: Set(false),
      product_type_id: Set(None),
      ..Default::default()
    }
    .insert(&**db)
    .await
    .unwrap();

    InventoryFixture {
      contractor_a_id: contractor_a.id,
      contractor_b_id: contractor_b.id,
      sender_id: sender.id,
      product_type_id: ptype.id,
      product_group_id: pgroup.id,
      product_a_id: product_a.id,
      product_b_id: product_b.id,
      base_id: b.id,
      warehouse_id: wh.id,
      storage_a_id: storage_a.id,
      storage_b_id: storage_b.id,
    }
  })
  .await
}

pub async fn seed_sync_node(
  db: &Arc<sea_orm::DatabaseConnection>,
  base_id: Uuid,
  common_name: &str,
) -> Uuid {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    database_instance::ActiveModel {
      common_name: Set(common_name.to_string()),
      node_type: Set(enums::NodeType::Peripheral),
      base_id: Set(Some(base_id)),
      ..Default::default()
    }
    .insert(&**db)
    .await
    .unwrap()
    .id
  })
  .await
}

#[allow(dead_code)]
pub async fn seed_ledger_balance(
  db: &Arc<sea_orm::DatabaseConnection>,
  storage_id: Uuid,
  product_id: Uuid,
  contractor_id: Uuid,
  current_amount: Decimal,
) {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    inventory_ledger_entry::ActiveModel {
      storage_id: Set(storage_id),
      product_id: Set(product_id),
      contractor_id: Set(contractor_id),
      current_amount: Set(current_amount),
      ..Default::default()
    }
    .insert(&**db)
    .await
    .unwrap();
  })
  .await;
}
