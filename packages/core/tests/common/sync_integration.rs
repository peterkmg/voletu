#![allow(dead_code)]

use std::{
  collections::HashMap,
  sync::{Mutex, OnceLock},
  time::Duration,
};

use reqwest::{Client, StatusCode};
use sea_orm::{
  ActiveModelTrait,
  ActiveValue::Set,
  DatabaseConnection,
  EntityTrait,
  IntoActiveModel,
};
use serde_json::{json, Value};
use tokio::sync::oneshot;
use uuid::Uuid;
use voletu_core::{
  context::audit::with_audit_context,
  db::init_database,
  entities::{
    base,
    company,
    database_instance,
    inventory_ledger_entry,
    local,
    ownership_transfer,
    ownership_transfer_item,
    product,
    product_group,
    product_type,
    storage,
    warehouse,
  },
  enums::{DocumentStatus, NodeType},
  serve_api,
  DbConfig,
  DbParams,
  JwtConfig,
};

const INITIAL_AUDIT_CURSOR: Uuid = Uuid::from_u128(1);

fn shared_db_keepalive() -> &'static Mutex<HashMap<String, DatabaseConnection>> {
  static REGISTRY: OnceLock<Mutex<HashMap<String, DatabaseConnection>>> = OnceLock::new();
  REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

async fn ensure_shared_memory_db_alive(db_name: &str) {
  let exists = {
    let registry = shared_db_keepalive()
      .lock()
      .expect("shared DB keepalive lock poisoned");
    registry.contains_key(db_name)
  };

  if exists {
    return;
  }

  let (db, _) = init_database(&db_cfg(db_name)).await.unwrap();

  let mut registry = shared_db_keepalive()
    .lock()
    .expect("shared DB keepalive lock poisoned");
  registry.entry(db_name.to_string()).or_insert(db);
}

#[derive(Clone, Copy)]
pub struct TransferFixtureRefs {
  pub contractor_a_id: Uuid,
  pub contractor_b_id: Uuid,
  pub product_type_id: Uuid,
  pub product_group_id: Uuid,
  pub product_id: Uuid,
  pub warehouse_id: Uuid,
  pub storage_id: Uuid,
}

fn db_cfg(db_name: &str) -> DbConfig {
  DbConfig::new(
    DbParams::sqlite_shared_memory(db_name.to_string()),
    "integrationtestpass",
  )
}

pub fn temp_db_path(prefix: &str) -> String {
  format!("{prefix}-{}", Uuid::now_v7())
}

pub fn reserve_port() -> u16 {
  let tcp = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
  let port = tcp.local_addr().unwrap().port();
  drop(tcp);
  port
}

pub async fn wait_for_health(
  client: &Client,
  base_url: &str,
  timeout: Duration,
  server_task: &mut tokio::task::JoinHandle<anyhow::Result<()>>,
) {
  let deadline = tokio::time::Instant::now() + timeout;
  loop {
    if server_task.is_finished() {
      let result = server_task
        .await
        .expect("server task join should succeed while waiting for health");
      panic!("server exited before becoming healthy: {result:?}");
    }

    if let Ok(response) = client.get(format!("{base_url}/health")).send().await {
      if response.status() == StatusCode::OK {
        return;
      }
    }

    assert!(
      tokio::time::Instant::now() < deadline,
      "server did not become healthy at {base_url} within {:?}",
      timeout
    );
    tokio::time::sleep(Duration::from_millis(100)).await;
  }
}

pub async fn wait_for_login_token(
  client: &Client,
  base_url: &str,
  username: &str,
  password: &str,
  timeout: Duration,
) -> String {
  let deadline = tokio::time::Instant::now() + timeout;
  let mut last_status: Option<StatusCode> = None;
  let mut last_body: Option<String> = None;
  loop {
    if let Ok(response) = client
      .post(format!("{base_url}/auth/login"))
      .header("idempotency-key", Uuid::now_v7().to_string())
      .json(&json!({"username": username, "password": password}))
      .send()
      .await
    {
      let status = response.status();
      let body_text = response.text().await.ok();

      last_status = Some(status);
      last_body = body_text.clone();

      if status == StatusCode::OK {
        let body: Value = serde_json::from_str(body_text.as_deref().unwrap_or("{}")).unwrap();
        return body["data"]["accessToken"].as_str().unwrap().to_string();
      }
    }

    assert!(
      tokio::time::Instant::now() < deadline,
      "login for '{username}' did not succeed within {:?}; last_status={:?}; last_body={:?}",
      timeout,
      last_status,
      last_body,
    );
    tokio::time::sleep(Duration::from_millis(120)).await;
  }
}

pub async fn prepare_node_db(
  db_name: &str,
  common_name: &str,
  node_type: NodeType,
  base_id: Option<Uuid>,
) -> Uuid {
  ensure_shared_memory_db_alive(db_name).await;
  let (db, node_cfg) = init_database(&db_cfg(db_name)).await.unwrap();
  let local_row = local::Entity::find_by_id(1)
    .one(&db)
    .await
    .unwrap()
    .unwrap();

  if let Some(base_id) = base_id {
    with_audit_context(Uuid::now_v7(), node_cfg.db_id, || async {
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
    })
    .await;
  }

  with_audit_context(Uuid::now_v7(), node_cfg.db_id, || async {
    let model = database_instance::Entity::find_by_id(local_row.local_db_id)
      .one(&db)
      .await
      .unwrap()
      .unwrap();
    let mut am = model.into_active_model();
    am.common_name = Set(common_name.to_string());
    am.node_type = Set(node_type);
    am.base_id = Set(base_id);
    am.update(&db).await.unwrap();
  })
  .await;

  local_row.local_db_id
}

pub async fn register_remote_node_on_central(
  central_db_name: &str,
  node_id: Uuid,
  common_name: &str,
  base_id: Uuid,
) {
  ensure_shared_memory_db_alive(central_db_name).await;
  let (db, central_node_cfg) = init_database(&db_cfg(central_db_name)).await.unwrap();

  with_audit_context(Uuid::now_v7(), central_node_cfg.db_id, || async {
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

    if database_instance::Entity::find_by_id(node_id)
      .one(&db)
      .await
      .unwrap()
      .is_none()
    {
      database_instance::ActiveModel {
        id: Set(node_id),
        common_name: Set(common_name.to_string()),
        node_type: Set(NodeType::Peripheral),
        base_id: Set(Some(base_id)),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();
    }
  })
  .await;
}

pub async fn spawn_server(
  db_name: &str,
  port: u16,
) -> (
  oneshot::Sender<()>,
  tokio::task::JoinHandle<anyhow::Result<()>>,
) {
  ensure_shared_memory_db_alive(db_name).await;
  let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
  let db_cfg = db_cfg(db_name);
  let jwt_cfg = JwtConfig::default();
  let server_task = tokio::spawn(async move {
    serve_api(
      "127.0.0.1".to_string(),
      port.to_string(),
      db_cfg,
      jwt_cfg,
      shutdown_rx,
    )
    .await
  });
  (shutdown_tx, server_task)
}

pub async fn api_get(client: &Client, url: &str, token: &str) -> Value {
  let response = client.get(url).bearer_auth(token).send().await.unwrap();
  assert_eq!(response.status(), StatusCode::OK);
  let body: Value = response.json().await.unwrap();
  assert_eq!(body["success"], Value::Bool(true));
  body["data"].clone()
}

pub async fn api_post(client: &Client, url: &str, token: &str, payload: Value) -> Value {
  let response = client
    .post(url)
    .bearer_auth(token)
    .header("idempotency-key", Uuid::now_v7().to_string())
    .json(&payload)
    .send()
    .await
    .unwrap();
  assert_eq!(response.status(), StatusCode::OK);
  let body: Value = response.json().await.unwrap();
  assert_eq!(body["success"], Value::Bool(true));
  body["data"].clone()
}

pub async fn ensure_shared_transfer_refs(
  db_name: &str,
  local_node_id: Uuid,
  base_1: Uuid,
  base_2: Uuid,
) -> TransferFixtureRefs {
  ensure_shared_memory_db_alive(db_name).await;
  let refs = TransferFixtureRefs {
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
  refs: TransferFixtureRefs,
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
        status: Set(DocumentStatus::Posted),
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
  client: &Client,
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

pub async fn inject_targeted_sync_log(
  client: &Client,
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

pub async fn push_outbound_to_central(
  client: &Client,
  source_url: &str,
  source_token: &str,
  central_url: &str,
  central_token: &str,
  after_audit_log_id: Uuid,
) -> usize {
  let logs = api_get(
    client,
    &format!(
      "{source_url}/sync/outbound?afterAuditLogId={}&limit=1000",
      after_audit_log_id
    ),
    source_token,
  )
  .await;
  let logs = logs.as_array().unwrap().clone();

  if !logs.is_empty() {
    let _ = api_post(
      client,
      &format!("{central_url}/sync/push"),
      central_token,
      json!({ "logs": logs }),
    )
    .await;
  }

  logs.len()
}

pub async fn pull_from_central_to_target(
  client: &Client,
  central_url: &str,
  central_token: &str,
  target_url: &str,
  target_token: &str,
  target_node_id: Uuid,
) -> (usize, Uuid) {
  pull_from_central_to_target_after(
    client,
    central_url,
    central_token,
    target_url,
    target_token,
    target_node_id,
    INITIAL_AUDIT_CURSOR,
  )
  .await
}

pub async fn pull_from_central_to_target_after(
  client: &Client,
  central_url: &str,
  central_token: &str,
  target_url: &str,
  target_token: &str,
  target_node_id: Uuid,
  last_audit_log_id: Uuid,
) -> (usize, Uuid) {
  let data = api_get(
    client,
    &format!(
      "{central_url}/sync/pull?nodeId={target_node_id}&lastAuditLogId={}&limit=1000",
      last_audit_log_id
    ),
    central_token,
  )
  .await;

  let logs = data["logs"].as_array().unwrap().clone();
  let highest_evaluated = Uuid::parse_str(data["highestEvaluatedId"].as_str().unwrap()).unwrap();

  if !logs.is_empty() {
    let _ = api_post(
      client,
      &format!("{target_url}/sync/push"),
      target_token,
      json!({ "logs": logs }),
    )
    .await;
  }

  (logs.len(), highest_evaluated)
}

pub async fn get_highest_audit_log_id(client: &Client, node_url: &str, token: &str) -> Uuid {
  let status = api_get(client, &format!("{node_url}/sync/status"), token).await;
  Uuid::parse_str(status["highestAuditLogId"].as_str().unwrap()).unwrap()
}

pub async fn has_audit_record(
  client: &Client,
  node_url: &str,
  token: &str,
  table_name: &str,
  record_id: Uuid,
) -> bool {
  let logs = api_get(client, &format!("{node_url}/audit-logs"), token).await;
  logs
    .as_array()
    .unwrap()
    .iter()
    .any(|log| log["tableName"] == table_name && log["recordId"] == record_id.to_string())
}

pub async fn get_ownership_transfer_json(
  client: &Client,
  node_url: &str,
  token: &str,
  transfer_id: Uuid,
) -> Option<Value> {
  let transfers = api_get(client, &format!("{node_url}/ownership-transfers"), token).await;
  transfers
    .as_array()
    .unwrap()
    .iter()
    .find(|item| item["id"] == transfer_id.to_string())
    .cloned()
}

pub async fn get_ledger_entry_json(
  client: &Client,
  node_url: &str,
  token: &str,
  storage_id: Uuid,
  product_id: Uuid,
  contractor_id: Uuid,
) -> Option<Value> {
  let entry = api_post(
    client,
    &format!("{node_url}/ledger/query"),
    token,
    json!({
      "storageId": storage_id,
      "productId": product_id,
      "contractorId": contractor_id,
    }),
  )
  .await;

  if entry.is_null() {
    None
  } else {
    Some(entry)
  }
}

pub async fn shutdown_server(
  shutdown_tx: oneshot::Sender<()>,
  server_task: tokio::task::JoinHandle<anyhow::Result<()>>,
) {
  let _ = shutdown_tx.send(());
  let join_result = tokio::time::timeout(Duration::from_secs(10), server_task)
    .await
    .expect("server task should shut down in time")
    .expect("server task join should succeed");
  join_result.expect("serve_api should return Ok on shutdown");
}
