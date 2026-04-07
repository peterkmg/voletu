use std::time::Duration;

use reqwest::Client;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, IntoActiveModel};
use serde_json::json;
use uuid::Uuid;
use voletu_core::{
  context::audit::with_audit_context,
  db::init_database,
  entities::{base, database_instance, local, node_base_assignment},
  enums::NodeType,
};

use super::{
  api_client::ensure_shared_memory_db_alive,
  api_post, pull_from_central_to_target_after,
  server::{
    db_cfg, reserve_port, spawn_server, wait_for_health, wait_for_login_token, NodeHandle,
  },
  sync_operations::INITIAL_AUDIT_CURSOR,
};

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
  let local_db_id = local_row.local_db_id;

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
    let model = database_instance::Entity::find_by_id(local_db_id)
      .one(&db)
      .await
      .unwrap()
      .unwrap();
    let mut am = model.into_active_model();
    am.common_name = Set(common_name.to_string());
    am.node_type = Set(node_type);
    am.base_id = Set(base_id);
    am.update(&db).await.unwrap();

    let mut local_am = local_row.into_active_model();
    local_am.is_initialized = Set(true);
    local_am.update(&db).await.unwrap();

    // Create node_base_assignment for multi-base pull filtering
    if let Some(base_id) = base_id {
      node_base_assignment::ActiveModel {
        node_id: Set(local_db_id),
        base_id: Set(base_id),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();
    }
  })
  .await;

  local_db_id
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

/// Add a base assignment to a peripheral node's DB.
/// Creates the base entity if it doesn't exist, then inserts a node_base_assignment row.
pub async fn add_base_assignment(db_name: &str, node_id: Uuid, base_id: Uuid, base_name: &str) {
  ensure_shared_memory_db_alive(db_name).await;
  let (db, _) = init_database(&db_cfg(db_name)).await.unwrap();
  with_audit_context(Uuid::now_v7(), node_id, || async {
    if base::Entity::find_by_id(base_id)
      .one(&db)
      .await
      .unwrap()
      .is_none()
    {
      base::ActiveModel {
        id: Set(base_id),
        common_name: Set(base_name.to_string()),
        long_name: Set(None),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();
    }
    node_base_assignment::ActiveModel {
      node_id: Set(node_id),
      base_id: Set(base_id),
      ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();
  })
  .await;
}

/// Add a base assignment to a running node via HTTP API (POST /node/bases).
pub async fn add_base_assignment_via_api(
  client: &Client,
  base_url: &str,
  token: &str,
  base_id: Uuid,
) {
  api_post(
    client,
    &format!("{base_url}/node/bases"),
    token,
    json!({ "baseId": base_id }),
  )
  .await;
}

/// Spawn a server and initialize it as Central via API only.
pub async fn setup_central_via_api(client: &Client, db_name: &str) -> NodeHandle {
  let port = reserve_port();
  let (shutdown_tx, mut task) = spawn_server(db_name, port).await;
  let url = format!("http://127.0.0.1:{port}");
  wait_for_health(client, &url, Duration::from_secs(10), &mut task).await;

  let bootstrap_token =
    wait_for_login_token(client, &url, "admin", "admin", Duration::from_secs(5)).await;

  api_post(
    client,
    &format!("{url}/node/initialize"),
    &bootstrap_token,
    json!({
      "nodeType": "CENTRAL",
      "nodeName": "Central",
      "centralApiUrl": null,
      "newUsername": "root",
      "newPassword": "rootpass",
      "fullname": "Root Admin",
    }),
  )
  .await;

  wait_for_health(client, &url, Duration::from_secs(20), &mut task).await;
  let token = wait_for_login_token(client, &url, "root", "rootpass", Duration::from_secs(10)).await;

  NodeHandle {
    url,
    token,
    shutdown_tx,
    task,
  }
}

/// Spawn a server and initialize it as a Peripheral via API only.
/// Pulls catalog from Central, then adds base assignments.
pub async fn setup_peripheral_via_api(
  client: &Client,
  db_name: &str,
  central: &NodeHandle,
  base_ids: &[Uuid],
) -> NodeHandle {
  let port = reserve_port();
  let (shutdown_tx, mut task) = spawn_server(db_name, port).await;
  let url = format!("http://127.0.0.1:{port}");
  wait_for_health(client, &url, Duration::from_secs(10), &mut task).await;

  let bootstrap_token =
    wait_for_login_token(client, &url, "admin", "admin", Duration::from_secs(5)).await;

  api_post(
    client,
    &format!("{url}/node/initialize"),
    &bootstrap_token,
    json!({
      "nodeType": "PERIPHERAL",
      "nodeName": null,
      "centralApiUrl": central.url,
      "newUsername": "root",
      "newPassword": "rootpass",
      "fullname": "Root Admin",
    }),
  )
  .await;

  wait_for_health(client, &url, Duration::from_secs(20), &mut task).await;
  let token = wait_for_login_token(client, &url, "root", "rootpass", Duration::from_secs(10)).await;

  // Pull catalog from Central so base entities exist on this peripheral.
  // Loop until all catalog audit logs are consumed (large seeds may exceed single batch).
  let mut cursor = INITIAL_AUDIT_CURSOR;
  loop {
    let (pulled, new_cursor) = pull_from_central_to_target_after(
      client,
      &central.url,
      &central.token,
      &url,
      &token,
      &[],
      cursor,
    )
    .await;
    if pulled == 0 || new_cursor == cursor {
      break;
    }
    cursor = new_cursor;
  }

  // Add base assignments via API
  for base_id in base_ids {
    add_base_assignment_via_api(client, &url, &token, *base_id).await;
  }

  NodeHandle {
    url,
    token,
    shutdown_tx,
    task,
  }
}
