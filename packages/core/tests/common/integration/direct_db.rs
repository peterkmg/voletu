use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
use serde_json::{json, Value};
use uuid::Uuid;
use voletu_core::{
  context::audit::with_audit_context,
  db::init_database,
  entities::{
    base, company, inventory_ledger_entry, ownership_transfer, ownership_transfer_item, product,
    product_group, product_type, storage, warehouse,
  },
  enums::DocumentStatus,
};

use super::{
  api_client::ensure_shared_memory_db_alive,
  api_post,
  server::db_cfg,
};

#[derive(Clone, Copy)]
pub struct TransferCatalogRefs {
  pub contractor_a_id: Uuid,
  pub contractor_b_id: Uuid,
  pub product_type_id: Uuid,
  pub product_group_id: Uuid,
  pub product_id: Uuid,
  pub warehouse_id: Uuid,
  pub storage_id: Uuid,
}

pub async fn ensure_shared_transfer_catalog(
  db_name: &str,
  local_node_id: Uuid,
  base_1: Uuid,
  base_2: Uuid,
) -> TransferCatalogRefs {
  ensure_shared_memory_db_alive(db_name).await;
  let refs = TransferCatalogRefs {
    contractor_a_id: Uuid::parse_str("00000000-0000-0000-0000-00000000aa01").unwrap(),
    contractor_b_id: Uuid::parse_str("00000000-0000-0000-0000-00000000aa02").unwrap(),
    product_type_id: Uuid::parse_str("00000000-0000-0000-0000-00000000bb01").unwrap(),
    product_group_id: Uuid::parse_str("00000000-0000-0000-0000-00000000bb02").unwrap(),
    product_id: Uuid::parse_str("00000000-0000-0000-0000-00000000bb03").unwrap(),
    warehouse_id: Uuid::parse_str("00000000-0000-0000-0000-00000000cc01").unwrap(),
    storage_id: Uuid::parse_str("00000000-0000-0000-0000-00000000cc02").unwrap(),
  };

  let (db, _) = init_database(&db_cfg(db_name)).await.unwrap();
  with_audit_context(Uuid::now_v7(), local_node_id, || async {
    for base_id in [base_1, base_2] {
      if base::Entity::find_by_id(base_id)
        .one(&db)
        .await
        .unwrap()
        .is_none()
      {
        base::ActiveModel {
          id: Set(base_id),
          common_name: Set(format!("Base-{base_id}")),
          long_name: Set(None),
          ..Default::default()
        }
        .insert(&db)
        .await
        .unwrap();
      }
    }

    if company::Entity::find_by_id(refs.contractor_a_id)
      .one(&db)
      .await
      .unwrap()
      .is_none()
    {
      company::ActiveModel {
        id: Set(refs.contractor_a_id),
        common_name: Set("Transfer-Contractor-A".to_string()),
        legal_name: Set(None),
        is_contractor: Set(true),
        is_exporter: Set(false),
        is_manufacturer: Set(false),
        is_sender: Set(false),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();
    }

    if company::Entity::find_by_id(refs.contractor_b_id)
      .one(&db)
      .await
      .unwrap()
      .is_none()
    {
      company::ActiveModel {
        id: Set(refs.contractor_b_id),
        common_name: Set("Transfer-Contractor-B".to_string()),
        legal_name: Set(None),
        is_contractor: Set(true),
        is_exporter: Set(false),
        is_manufacturer: Set(false),
        is_sender: Set(false),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();
    }

    if product_type::Entity::find_by_id(refs.product_type_id)
      .one(&db)
      .await
      .unwrap()
      .is_none()
    {
      product_type::ActiveModel {
        id: Set(refs.product_type_id),
        common_name: Set("Transfer-PT".to_string()),
        long_name: Set(None),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();
    }

    if product_group::Entity::find_by_id(refs.product_group_id)
      .one(&db)
      .await
      .unwrap()
      .is_none()
    {
      product_group::ActiveModel {
        id: Set(refs.product_group_id),
        product_type_id: Set(refs.product_type_id),
        common_name: Set("Transfer-PG".to_string()),
        long_name: Set(None),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();
    }

    if product::Entity::find_by_id(refs.product_id)
      .one(&db)
      .await
      .unwrap()
      .is_none()
    {
      product::ActiveModel {
        id: Set(refs.product_id),
        product_group_id: Set(refs.product_group_id),
        manufacturer_id: Set(None),
        common_name: Set("Transfer-Product".to_string()),
        long_name: Set(None),
        add_identification: Set(None),
        is_component: Set(true),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();
    }

    if warehouse::Entity::find_by_id(refs.warehouse_id)
      .one(&db)
      .await
      .unwrap()
      .is_none()
    {
      warehouse::ActiveModel {
        id: Set(refs.warehouse_id),
        base_id: Set(base_1),
        common_name: Set("Transfer-WH".to_string()),
        long_name: Set(None),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();
    }

    if storage::Entity::find_by_id(refs.storage_id)
      .one(&db)
      .await
      .unwrap()
      .is_none()
    {
      storage::ActiveModel {
        id: Set(refs.storage_id),
        warehouse_id: Set(refs.warehouse_id),
        common_name: Set("Transfer-Storage".to_string()),
        long_name: Set(None),
        capacity: Set(None),
        is_type_specific: Set(false),
        product_type_id: Set(None),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();
    }
  })
  .await;

  refs
}

pub async fn create_local_transfer_and_ledger(
  db_name: &str,
  local_node_id: Uuid,
  refs: TransferCatalogRefs,
  transfer_id: Uuid,
  ledger_entry_id: Uuid,
  amount: i64,
) -> (Value, Uuid, Value, Value) {
  ensure_shared_memory_db_alive(db_name).await;
  let (db, _) = init_database(&db_cfg(db_name)).await.unwrap();

  let (transfer, transfer_item, ledger_entry) =
    with_audit_context(Uuid::now_v7(), local_node_id, || async {
      let transfer = ownership_transfer::ActiveModel {
        id: Set(transfer_id),
        date: Set(chrono::Utc::now()),
        status: Set(DocumentStatus::Executed),
        version: Set(1),
        executed_at: Set(Some(chrono::Utc::now())),
        executed_by: Set(Some(local_node_id)),
        reverted_at: Set(None),
        reverted_by: Set(None),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();

      let transfer_item = ownership_transfer_item::ActiveModel {
        ownership_transfer_id: Set(transfer.id),
        storage_id: Set(refs.storage_id),
        product_id: Set(refs.product_id),
        from_contractor_id: Set(refs.contractor_a_id),
        to_contractor_id: Set(refs.contractor_b_id),
        amount: Set(amount.into()),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();

      let ledger_entry = inventory_ledger_entry::ActiveModel {
        id: Set(ledger_entry_id),
        storage_id: Set(refs.storage_id),
        product_id: Set(refs.product_id),
        contractor_id: Set(refs.contractor_b_id),
        current_amount: Set(amount.into()),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();

      (transfer, transfer_item, ledger_entry)
    })
    .await;

  (
    serde_json::to_value(&transfer).unwrap(),
    transfer_item.id,
    serde_json::to_value(&transfer_item).unwrap(),
    serde_json::to_value(&ledger_entry).unwrap(),
  )
}

pub async fn inject_targeted_idempotency_log(
  client: &reqwest::Client,
  node_url: &str,
  token: &str,
  origin_db_id: Uuid,
  record_id: Uuid,
  request_key: &str,
  target_base_ids: &str,
) {
  let snapshot = json!({
    "id": record_id,
    "request_key": request_key,
    "created_at": "2026-01-10T00:00:00Z"
  });

  let _ = api_post(
    client,
    &format!("{node_url}/sync/push"),
    token,
    json!({
      "logs": [{
        "id": Uuid::now_v7(),
        "tableName": "audit_logs",
        "recordId": record_id,
        "action": "INSERT",
        "oldValuesJson": null,
        "newValuesJson": serde_json::to_string(&snapshot).unwrap(),
        "targetBaseIds": target_base_ids,
        "userRoleWeight": 10,
        "userId": origin_db_id,
        "timestamp": "2026-01-10T00:00:01Z",
        "originDbId": origin_db_id,
      }]
    }),
  )
  .await;
}

#[allow(clippy::too_many_arguments)]
pub async fn inject_targeted_sync_log(
  client: &reqwest::Client,
  node_url: &str,
  token: &str,
  origin_db_id: Uuid,
  table_name: &str,
  record_id: Uuid,
  snapshot: &Value,
  target_base_ids: &str,
) {
  let _ = api_post(
    client,
    &format!("{node_url}/sync/push"),
    token,
    json!({
      "logs": [{
        "id": Uuid::now_v7(),
        "tableName": table_name,
        "recordId": record_id,
        "action": "UPDATE",
        "oldValuesJson": null,
        "newValuesJson": serde_json::to_string(snapshot).unwrap(),
        "targetBaseIds": target_base_ids,
        "userRoleWeight": 10,
        "userId": origin_db_id,
        "timestamp": "2026-01-10T00:00:01Z",
        "originDbId": origin_db_id,
      }]
    }),
  )
  .await;
}
