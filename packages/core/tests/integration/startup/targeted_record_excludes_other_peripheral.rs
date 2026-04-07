//! Verifies that a record targeted at one Peripheral's base assignment is
//! excluded from the pull destined for a different Peripheral, while the
//! evaluated watermark still advances past the skipped entries.
//!
//! Topology: Central + 2 Peripherals (Peripheral-1 on base_1, Peripheral-2 on base_2).
//!
//! Property: after Peripheral-1 pushes a base_1-scoped record, Central holds it,
//! Peripheral-1 retains it, but Peripheral-2 does not receive it; the watermark
//! for Peripheral-2 nevertheless advances beyond the initial cursor.

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

  let (_pulled_for_p2, highest_evaluated_for_p2) = pull_from_central_to_target(
    &client,
    &central_url,
    &central_token,
    &p2_url,
    &p2_token,
    &[base_2],
  )
  .await;

  // P2 may receive global-table entries (bases, database_instances) but should NOT
  // receive the targeted idempotency log that was scoped to base_1 only.
  // The watermark should advance at least past the initial cursor.
  assert!(highest_evaluated_for_p2 > INITIAL_AUDIT_CURSOR);

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
