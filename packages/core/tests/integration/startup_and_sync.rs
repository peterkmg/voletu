use std::time::Duration;

use reqwest::{Client, StatusCode};
use serde_json::Value;
use tokio::sync::oneshot;
use uuid::Uuid;
use voletu_core::{enums::NodeType, serve_api, DbConfig, DbParams, JwtConfig};

use crate::common::{
  payloads::{auth_login, node_initialize_replace_with_node_type},
  sync_integration::{
    api_get,
    create_local_transfer_and_ledger,
    ensure_shared_transfer_refs,
    get_highest_audit_log_id,
    get_ledger_entry_json,
    get_ownership_transfer_json,
    has_audit_record,
    inject_targeted_idempotency_log,
    inject_targeted_sync_log,
    prepare_node_db,
    pull_from_central_to_target,
    pull_from_central_to_target_after,
    push_outbound_to_central,
    register_remote_node_on_central,
    reserve_port,
    shutdown_server,
    spawn_server,
    temp_db_path,
    wait_for_health,
    wait_for_login_token,
  },
};

const INITIAL_AUDIT_CURSOR: Uuid = Uuid::from_u128(1);

#[tokio::test]
async fn initialize_endpoint_triggers_restart_and_reloads_node_configuration() {
  let tcp = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
  let port = tcp.local_addr().unwrap().port();
  drop(tcp);

  let db_path = std::env::temp_dir().join(format!("voletu-init-restart-{}.db", Uuid::now_v7()));
  let db_cfg = DbConfig::new(DbParams::sqlite(db_path.clone()), "integrationtestpass");
  let jwt_cfg = JwtConfig::default();

  let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
  let mut server_task = tokio::spawn(async move {
    serve_api(
      "127.0.0.1".to_string(),
      port.to_string(),
      db_cfg,
      jwt_cfg,
      shutdown_rx,
    )
    .await
  });

  let client = Client::new();
  let base_url = format!("http://127.0.0.1:{port}");

  wait_for_health(
    &client,
    &base_url,
    Duration::from_secs(10),
    &mut server_task,
  )
  .await;

  let admin_token =
    wait_for_login_token(&client, &base_url, "admin", "admin", Duration::from_secs(5)).await;

  let init_response = client
    .post(format!("{base_url}/node/initialize"))
    .header("idempotency-key", Uuid::now_v7().to_string())
    .bearer_auth(&admin_token)
    .json(
      &serde_json::from_str::<Value>(&node_initialize_replace_with_node_type(
        "root",
        "root-password",
        "Root User",
        "CENTRAL",
      ))
      .unwrap(),
    )
    .send()
    .await
    .unwrap();
  assert_eq!(init_response.status(), StatusCode::OK);

  let init_body: Value = init_response.json().await.unwrap();
  assert_eq!(init_body["data"]["message"], "Initialization completed");

  wait_for_health(
    &client,
    &base_url,
    Duration::from_secs(20),
    &mut server_task,
  )
  .await;

  let root_token = wait_for_login_token(
    &client,
    &base_url,
    "root",
    "root-password",
    Duration::from_secs(20),
  )
  .await;

  let old_admin_response = client
    .post(format!("{base_url}/auth/login"))
    .header("idempotency-key", Uuid::now_v7().to_string())
    .json(&serde_json::from_str::<Value>(&auth_login("admin", "admin")).unwrap())
    .send()
    .await
    .unwrap();
  assert_eq!(old_admin_response.status(), StatusCode::UNAUTHORIZED);

  let sync_status_response = client
    .get(format!("{base_url}/sync/status"))
    .bearer_auth(root_token)
    .send()
    .await
    .unwrap();
  assert_eq!(sync_status_response.status(), StatusCode::OK);
  let sync_status_body: Value = sync_status_response.json().await.unwrap();
  assert_eq!(sync_status_body["data"]["nodeType"], "CENTRAL");

  let _ = shutdown_tx.send(());

  let join_result = tokio::time::timeout(Duration::from_secs(10), server_task)
    .await
    .expect("server task should shut down in time")
    .expect("server task join should succeed");
  join_result.expect("serve_api should return Ok on shutdown");

  let _ = std::fs::remove_file(db_path);
}

#[tokio::test]
async fn sync_integration_a_central_and_one_peripheral_reconstructs_to_parity() {
  let base_1 = Uuid::parse_str("00000000-0000-0000-0000-000000000111").unwrap();
  let central_db = temp_db_path("sync-a-central");
  let peripheral_db = temp_db_path("sync-a-peripheral");

  let peripheral_node_id = prepare_node_db(
    &peripheral_db,
    "Peripheral-1",
    NodeType::Peripheral,
    Some(base_1),
  )
  .await;
  let _central_node_id = prepare_node_db(&central_db, "Central", NodeType::Central, None).await;
  register_remote_node_on_central(&central_db, peripheral_node_id, "Peripheral-1", base_1).await;

  let central_port = reserve_port();
  let peripheral_port = reserve_port();
  let (central_shutdown_tx, mut central_task) = spawn_server(&central_db, central_port).await;
  let (peripheral_shutdown_tx, mut peripheral_task) =
    spawn_server(&peripheral_db, peripheral_port).await;

  let client = Client::new();
  let central_url = format!("http://127.0.0.1:{central_port}");
  let peripheral_url = format!("http://127.0.0.1:{peripheral_port}");

  wait_for_health(
    &client,
    &central_url,
    Duration::from_secs(10),
    &mut central_task,
  )
  .await;
  wait_for_health(
    &client,
    &peripheral_url,
    Duration::from_secs(10),
    &mut peripheral_task,
  )
  .await;

  let central_token = wait_for_login_token(
    &client,
    &central_url,
    "admin",
    "admin",
    Duration::from_secs(5),
  )
  .await;
  let peripheral_token = wait_for_login_token(
    &client,
    &peripheral_url,
    "admin",
    "admin",
    Duration::from_secs(5),
  )
  .await;

  let record_id = Uuid::now_v7();
  inject_targeted_idempotency_log(
    &client,
    &peripheral_url,
    &peripheral_token,
    peripheral_node_id,
    record_id,
    "sync-a-request",
    &base_1.to_string(),
  )
  .await;

  let pushed = push_outbound_to_central(
    &client,
    &peripheral_url,
    &peripheral_token,
    &central_url,
    &central_token,
    INITIAL_AUDIT_CURSOR,
  )
  .await;
  assert!(pushed > 0);

  let _ = pull_from_central_to_target(
    &client,
    &central_url,
    &central_token,
    &peripheral_url,
    &peripheral_token,
    peripheral_node_id,
  )
  .await;

  assert!(
    has_audit_record(
      &client,
      &central_url,
      &central_token,
      "audit_logs",
      record_id,
    )
    .await
  );
  assert!(
    has_audit_record(
      &client,
      &peripheral_url,
      &peripheral_token,
      "audit_logs",
      record_id,
    )
    .await
  );

  shutdown_server(central_shutdown_tx, central_task).await;
  shutdown_server(peripheral_shutdown_tx, peripheral_task).await;
}

#[tokio::test]
async fn sync_integration_b_targeted_record_excludes_other_peripheral_and_advances_evaluated_watermark(
) {
  let base_1 = Uuid::parse_str("00000000-0000-0000-0000-000000000121").unwrap();
  let base_2 = Uuid::parse_str("00000000-0000-0000-0000-000000000122").unwrap();

  let central_db = temp_db_path("sync-b-central");
  let p1_db = temp_db_path("sync-b-p1");
  let p2_db = temp_db_path("sync-b-p2");

  let p1_node_id =
    prepare_node_db(&p1_db, "Peripheral-1", NodeType::Peripheral, Some(base_1)).await;
  let p2_node_id =
    prepare_node_db(&p2_db, "Peripheral-2", NodeType::Peripheral, Some(base_2)).await;
  let _central_node_id = prepare_node_db(&central_db, "Central", NodeType::Central, None).await;
  register_remote_node_on_central(&central_db, p1_node_id, "Peripheral-1", base_1).await;
  register_remote_node_on_central(&central_db, p2_node_id, "Peripheral-2", base_2).await;

  let central_port = reserve_port();
  let p1_port = reserve_port();
  let p2_port = reserve_port();
  let (central_shutdown_tx, mut central_task) = spawn_server(&central_db, central_port).await;
  let (p1_shutdown_tx, mut p1_task) = spawn_server(&p1_db, p1_port).await;
  let (p2_shutdown_tx, mut p2_task) = spawn_server(&p2_db, p2_port).await;

  let client = Client::new();
  let central_url = format!("http://127.0.0.1:{central_port}");
  let p1_url = format!("http://127.0.0.1:{p1_port}");
  let p2_url = format!("http://127.0.0.1:{p2_port}");

  wait_for_health(
    &client,
    &central_url,
    Duration::from_secs(10),
    &mut central_task,
  )
  .await;
  wait_for_health(&client, &p1_url, Duration::from_secs(10), &mut p1_task).await;
  wait_for_health(&client, &p2_url, Duration::from_secs(10), &mut p2_task).await;

  let central_token = wait_for_login_token(
    &client,
    &central_url,
    "admin",
    "admin",
    Duration::from_secs(5),
  )
  .await;
  let p1_token =
    wait_for_login_token(&client, &p1_url, "admin", "admin", Duration::from_secs(5)).await;
  let p2_token =
    wait_for_login_token(&client, &p2_url, "admin", "admin", Duration::from_secs(5)).await;

  let record_id = Uuid::now_v7();
  inject_targeted_idempotency_log(
    &client,
    &p1_url,
    &p1_token,
    p1_node_id,
    record_id,
    "sync-b-request",
    &base_1.to_string(),
  )
  .await;

  let pushed = push_outbound_to_central(
    &client,
    &p1_url,
    &p1_token,
    &central_url,
    &central_token,
    INITIAL_AUDIT_CURSOR,
  )
  .await;
  assert!(pushed > 0);

  let (pulled_for_p2, highest_evaluated_for_p2) = pull_from_central_to_target(
    &client,
    &central_url,
    &central_token,
    &p2_url,
    &p2_token,
    p2_node_id,
  )
  .await;

  assert_eq!(pulled_for_p2, 0);
  let central_status = api_get(
    &client,
    &format!("{central_url}/sync/status"),
    &central_token,
  )
  .await;
  let central_highest =
    Uuid::parse_str(central_status["highestAuditLogId"].as_str().unwrap()).unwrap();
  assert_eq!(highest_evaluated_for_p2, central_highest);

  assert!(
    has_audit_record(
      &client,
      &central_url,
      &central_token,
      "audit_logs",
      record_id,
    )
    .await
  );
  assert!(has_audit_record(&client, &p1_url, &p1_token, "audit_logs", record_id).await);
  assert!(!has_audit_record(&client, &p2_url, &p2_token, "audit_logs", record_id).await);

  shutdown_server(central_shutdown_tx, central_task).await;
  shutdown_server(p1_shutdown_tx, p1_task).await;
  shutdown_server(p2_shutdown_tx, p2_task).await;
}

#[tokio::test]
async fn sync_integration_c_shared_target_converges_across_central_and_two_peripherals() {
  let base_1 = Uuid::parse_str("00000000-0000-0000-0000-000000000131").unwrap();
  let base_2 = Uuid::parse_str("00000000-0000-0000-0000-000000000132").unwrap();

  let central_db = temp_db_path("sync-c-central");
  let p1_db = temp_db_path("sync-c-p1");
  let p2_db = temp_db_path("sync-c-p2");

  let p1_node_id =
    prepare_node_db(&p1_db, "Peripheral-1", NodeType::Peripheral, Some(base_1)).await;
  let p2_node_id =
    prepare_node_db(&p2_db, "Peripheral-2", NodeType::Peripheral, Some(base_2)).await;
  let _central_node_id = prepare_node_db(&central_db, "Central", NodeType::Central, None).await;
  register_remote_node_on_central(&central_db, p1_node_id, "Peripheral-1", base_1).await;
  register_remote_node_on_central(&central_db, p2_node_id, "Peripheral-2", base_2).await;

  let central_port = reserve_port();
  let p1_port = reserve_port();
  let p2_port = reserve_port();
  let (central_shutdown_tx, mut central_task) = spawn_server(&central_db, central_port).await;
  let (p1_shutdown_tx, mut p1_task) = spawn_server(&p1_db, p1_port).await;
  let (p2_shutdown_tx, mut p2_task) = spawn_server(&p2_db, p2_port).await;

  let client = Client::new();
  let central_url = format!("http://127.0.0.1:{central_port}");
  let p1_url = format!("http://127.0.0.1:{p1_port}");
  let p2_url = format!("http://127.0.0.1:{p2_port}");

  wait_for_health(
    &client,
    &central_url,
    Duration::from_secs(10),
    &mut central_task,
  )
  .await;
  wait_for_health(&client, &p1_url, Duration::from_secs(10), &mut p1_task).await;
  wait_for_health(&client, &p2_url, Duration::from_secs(10), &mut p2_task).await;

  let central_token = wait_for_login_token(
    &client,
    &central_url,
    "admin",
    "admin",
    Duration::from_secs(5),
  )
  .await;
  let p1_token =
    wait_for_login_token(&client, &p1_url, "admin", "admin", Duration::from_secs(5)).await;
  let p2_token =
    wait_for_login_token(&client, &p2_url, "admin", "admin", Duration::from_secs(5)).await;

  let record_id = Uuid::now_v7();
  inject_targeted_idempotency_log(
    &client,
    &p1_url,
    &p1_token,
    p1_node_id,
    record_id,
    "sync-c-request",
    &format!("{},{}", base_1, base_2),
  )
  .await;

  let pushed = push_outbound_to_central(
    &client,
    &p1_url,
    &p1_token,
    &central_url,
    &central_token,
    INITIAL_AUDIT_CURSOR,
  )
  .await;
  assert!(pushed > 0);

  let (pulled_for_p2, _) = pull_from_central_to_target(
    &client,
    &central_url,
    &central_token,
    &p2_url,
    &p2_token,
    p2_node_id,
  )
  .await;
  assert!(pulled_for_p2 > 0);

  let _ = pull_from_central_to_target(
    &client,
    &central_url,
    &central_token,
    &p1_url,
    &p1_token,
    p1_node_id,
  )
  .await;

  assert!(
    has_audit_record(
      &client,
      &central_url,
      &central_token,
      "audit_logs",
      record_id,
    )
    .await
  );
  assert!(has_audit_record(&client, &p1_url, &p1_token, "audit_logs", record_id).await);
  assert!(has_audit_record(&client, &p2_url, &p2_token, "audit_logs", record_id).await);

  shutdown_server(central_shutdown_tx, central_task).await;
  shutdown_server(p1_shutdown_tx, p1_task).await;
  shutdown_server(p2_shutdown_tx, p2_task).await;
}

#[tokio::test]
async fn sync_integration_ledger_affected_transfer_targets_shared_then_local_scope() {
  let base_a = Uuid::parse_str("00000000-0000-0000-0000-000000000141").unwrap();
  let base_c = Uuid::parse_str("00000000-0000-0000-0000-000000000142").unwrap();

  let central_db = temp_db_path("sync-ledger-central");
  let a_db = temp_db_path("sync-ledger-a");
  let c_db = temp_db_path("sync-ledger-c");

  let a_node_id = prepare_node_db(&a_db, "Node-A", NodeType::Peripheral, Some(base_a)).await;
  let c_node_id = prepare_node_db(&c_db, "Node-C", NodeType::Peripheral, Some(base_c)).await;
  let _central_node_id =
    prepare_node_db(&central_db, "Node-B-Central", NodeType::Central, None).await;

  register_remote_node_on_central(&central_db, a_node_id, "Node-A", base_a).await;
  register_remote_node_on_central(&central_db, c_node_id, "Node-C", base_c).await;

  let refs_a = ensure_shared_transfer_refs(&a_db, a_node_id, base_a, base_c).await;
  let _refs_b = ensure_shared_transfer_refs(&central_db, a_node_id, base_a, base_c).await;
  let _refs_c = ensure_shared_transfer_refs(&c_db, c_node_id, base_a, base_c).await;

  let central_port = reserve_port();
  let a_port = reserve_port();
  let c_port = reserve_port();

  let (central_shutdown_tx, mut central_task) = spawn_server(&central_db, central_port).await;
  let (a_shutdown_tx, mut a_task) = spawn_server(&a_db, a_port).await;
  let (c_shutdown_tx, mut c_task) = spawn_server(&c_db, c_port).await;

  let client = Client::new();
  let central_url = format!("http://127.0.0.1:{central_port}");
  let a_url = format!("http://127.0.0.1:{a_port}");
  let c_url = format!("http://127.0.0.1:{c_port}");

  wait_for_health(
    &client,
    &central_url,
    Duration::from_secs(10),
    &mut central_task,
  )
  .await;
  wait_for_health(&client, &a_url, Duration::from_secs(10), &mut a_task).await;
  wait_for_health(&client, &c_url, Duration::from_secs(10), &mut c_task).await;

  let central_token = wait_for_login_token(
    &client,
    &central_url,
    "admin",
    "admin",
    Duration::from_secs(5),
  )
  .await;
  let a_token =
    wait_for_login_token(&client, &a_url, "admin", "admin", Duration::from_secs(5)).await;
  let c_token =
    wait_for_login_token(&client, &c_url, "admin", "admin", Duration::from_secs(5)).await;

  let transfer_1_id = Uuid::now_v7();
  let ledger_1_id = Uuid::now_v7();
  let (transfer_1_snapshot, transfer_1_item_id, transfer_1_item_snapshot, ledger_1_snapshot) =
    create_local_transfer_and_ledger(&a_db, a_node_id, refs_a, transfer_1_id, ledger_1_id, 120)
      .await;
  let outbound_cursor_1 = get_highest_audit_log_id(&client, &a_url, &a_token).await;

  inject_targeted_sync_log(
    &client,
    &a_url,
    &a_token,
    a_node_id,
    "ownership_transfers",
    transfer_1_id,
    &transfer_1_snapshot,
    &format!("{},{}", base_a, base_c),
  )
  .await;
  inject_targeted_sync_log(
    &client,
    &a_url,
    &a_token,
    a_node_id,
    "ownership_transfer_items",
    transfer_1_item_id,
    &transfer_1_item_snapshot,
    &format!("{},{}", base_a, base_c),
  )
  .await;
  inject_targeted_sync_log(
    &client,
    &a_url,
    &a_token,
    a_node_id,
    "inventory_ledger_entries",
    ledger_1_id,
    &ledger_1_snapshot,
    &format!("{},{}", base_a, base_c),
  )
  .await;

  let pushed_wave_1 = push_outbound_to_central(
    &client,
    &a_url,
    &a_token,
    &central_url,
    &central_token,
    outbound_cursor_1,
  )
  .await;
  assert!(pushed_wave_1 >= 2);

  let pulled_wave_1_for_c = pull_from_central_to_target(
    &client,
    &central_url,
    &central_token,
    &c_url,
    &c_token,
    c_node_id,
  )
  .await;
  assert!(pulled_wave_1_for_c.0 >= 2);

  let transfer_1_a = get_ownership_transfer_json(&client, &a_url, &a_token, transfer_1_id)
    .await
    .unwrap();
  let transfer_1_b =
    get_ownership_transfer_json(&client, &central_url, &central_token, transfer_1_id)
      .await
      .unwrap();
  let transfer_1_c = get_ownership_transfer_json(&client, &c_url, &c_token, transfer_1_id)
    .await
    .unwrap();
  assert_eq!(
    transfer_1_a["items"][0]["amount"],
    transfer_1_b["items"][0]["amount"]
  );
  assert_eq!(
    transfer_1_b["items"][0]["amount"],
    transfer_1_c["items"][0]["amount"]
  );

  let ledger_1_a = get_ledger_entry_json(
    &client,
    &a_url,
    &a_token,
    refs_a.storage_id,
    refs_a.product_id,
    refs_a.contractor_b_id,
  )
  .await
  .unwrap();
  let ledger_1_b = get_ledger_entry_json(
    &client,
    &central_url,
    &central_token,
    refs_a.storage_id,
    refs_a.product_id,
    refs_a.contractor_b_id,
  )
  .await
  .unwrap();
  let ledger_1_c = get_ledger_entry_json(
    &client,
    &c_url,
    &c_token,
    refs_a.storage_id,
    refs_a.product_id,
    refs_a.contractor_b_id,
  )
  .await
  .unwrap();
  assert_eq!(ledger_1_a["currentAmount"], ledger_1_b["currentAmount"]);
  assert_eq!(ledger_1_b["currentAmount"], ledger_1_c["currentAmount"]);

  let transfer_2_id = Uuid::now_v7();
  let ledger_2_id = Uuid::now_v7();
  let (transfer_2_snapshot, transfer_2_item_id, transfer_2_item_snapshot, ledger_2_snapshot) =
    create_local_transfer_and_ledger(&a_db, a_node_id, refs_a, transfer_2_id, ledger_2_id, 45)
      .await;
  let outbound_cursor_2 = get_highest_audit_log_id(&client, &a_url, &a_token).await;

  inject_targeted_sync_log(
    &client,
    &a_url,
    &a_token,
    a_node_id,
    "ownership_transfers",
    transfer_2_id,
    &transfer_2_snapshot,
    &base_a.to_string(),
  )
  .await;
  inject_targeted_sync_log(
    &client,
    &a_url,
    &a_token,
    a_node_id,
    "ownership_transfer_items",
    transfer_2_item_id,
    &transfer_2_item_snapshot,
    &base_a.to_string(),
  )
  .await;
  inject_targeted_sync_log(
    &client,
    &a_url,
    &a_token,
    a_node_id,
    "inventory_ledger_entries",
    ledger_2_id,
    &ledger_2_snapshot,
    &base_a.to_string(),
  )
  .await;

  let pushed_wave_2 = push_outbound_to_central(
    &client,
    &a_url,
    &a_token,
    &central_url,
    &central_token,
    outbound_cursor_2,
  )
  .await;
  assert!(pushed_wave_2 >= 2);

  let (pulled_wave_2_for_c, _) = pull_from_central_to_target_after(
    &client,
    &central_url,
    &central_token,
    &c_url,
    &c_token,
    c_node_id,
    pulled_wave_1_for_c.1,
  )
  .await;
  assert_eq!(pulled_wave_2_for_c, 0);

  let transfer_2_a = get_ownership_transfer_json(&client, &a_url, &a_token, transfer_2_id)
    .await
    .unwrap();
  let transfer_2_b =
    get_ownership_transfer_json(&client, &central_url, &central_token, transfer_2_id)
      .await
      .unwrap();
  let transfer_2_c = get_ownership_transfer_json(&client, &c_url, &c_token, transfer_2_id).await;
  assert_eq!(
    transfer_2_a["items"][0]["amount"],
    transfer_2_b["items"][0]["amount"]
  );
  assert!(transfer_2_c.is_none());

  let ledger_2_a = get_ledger_entry_json(
    &client,
    &a_url,
    &a_token,
    refs_a.storage_id,
    refs_a.product_id,
    refs_a.contractor_b_id,
  )
  .await
  .unwrap();
  let ledger_2_b = get_ledger_entry_json(
    &client,
    &central_url,
    &central_token,
    refs_a.storage_id,
    refs_a.product_id,
    refs_a.contractor_b_id,
  )
  .await
  .unwrap();
  let ledger_2_c = get_ledger_entry_json(
    &client,
    &c_url,
    &c_token,
    refs_a.storage_id,
    refs_a.product_id,
    refs_a.contractor_b_id,
  )
  .await;
  assert_eq!(ledger_2_a["currentAmount"], ledger_2_b["currentAmount"]);
  assert_eq!(
    ledger_2_c.unwrap()["currentAmount"],
    ledger_1_c["currentAmount"]
  );

  shutdown_server(central_shutdown_tx, central_task).await;
  shutdown_server(a_shutdown_tx, a_task).await;
  shutdown_server(c_shutdown_tx, c_task).await;
}
