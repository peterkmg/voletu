//! Verifies that a basic push/pull cycle between a Central node and one
//! Peripheral node reconstructs the data to parity on both sides.
//!
//! Topology: Central + 1 Peripheral (assigned to base_1).
//!
//! Property: after the Peripheral pushes a targeted idempotency log to Central
//! and Central pulls it back, both nodes contain the same audit record.

use std::time::Duration;

use reqwest::Client;
use uuid::Uuid;
use voletu_core::enums::NodeType;

use crate::common::integration::{
  has_audit_record,
  inject_targeted_idempotency_log,
  prepare_node_db,
  pull_from_central_to_target,
  push_outbound_to_central,
  register_remote_node_on_central,
  reserve_port,
  shutdown_server,
  spawn_server,
  temp_db_path,
  wait_for_health,
  wait_for_login_token,
};

const INITIAL_AUDIT_CURSOR: Uuid = Uuid::from_u128(1);

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
    &[base_1],
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
