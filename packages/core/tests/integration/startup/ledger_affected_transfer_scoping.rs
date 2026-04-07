//! Verifies that ledger-affected ownership transfers propagate with correct
//! scope narrowing across a three-node topology. The first wave uses a shared
//! target (both bases), so all nodes converge. The second wave uses a local-only
//! target (base_a only), so Node-C does not receive it.
//!
//! Topology: Central (Node-B) + 2 Peripherals (Node-A on base_a, Node-C on base_c).
//!
//! Property: shared-scope transfers and ledger entries reach all three nodes;
//! local-scope transfers and ledger entries reach only Central and the originating
//! Peripheral, while the other Peripheral's ledger remains at the prior value.

use std::time::Duration;

use reqwest::Client;
use uuid::Uuid;
use voletu_core::enums::NodeType;

use crate::common::integration::{
  create_local_transfer_and_ledger, ensure_shared_transfer_catalog, get_highest_audit_log_id,
  get_ledger_entry_json, get_ownership_transfer_json, inject_targeted_sync_log, prepare_node_db,
  pull_from_central_to_target, pull_from_central_to_target_after, push_outbound_to_central,
  register_remote_node_on_central, reserve_port, shutdown_server, spawn_server, temp_db_path,
  wait_for_health, wait_for_login_token,
};

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

  let refs_a = ensure_shared_transfer_catalog(&a_db, a_node_id, base_a, base_c).await;
  let _refs_b = ensure_shared_transfer_catalog(&central_db, a_node_id, base_a, base_c).await;
  let _refs_c = ensure_shared_transfer_catalog(&c_db, c_node_id, base_a, base_c).await;

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
    &[base_c],
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
    &[base_c],
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
