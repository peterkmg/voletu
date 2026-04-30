use std::time::Duration;

use uuid::Uuid;

use crate::common::integration::{
  api_get,
  assert_audit_log_targets,
  await_sync_cycle,
  create_reconciliation_via_api,
  query_audit_logs,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
};

const SYNC_TIMEOUT: Duration = Duration::from_secs(15);

#[tokio::test]
async fn targets_warehouse_base_in_audit_log_and_syncs_to_peripheral() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r12-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r12-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;

  let recon = create_reconciliation_via_api(
    &client,
    &central.url,
    &central.token,
    "RECON-001",
    catalog.contractor,
    catalog.warehouse_alpha,
  )
  .await;
  let recon_id = Uuid::parse_str(recon["id"].as_str().unwrap()).unwrap();

  let logs = query_audit_logs(
    &client,
    &central.url,
    &central.token,
    Some("inventory_reconciliations"),
    Some(recon_id),
  )
  .await;
  assert!(!logs.is_empty());
  assert_audit_log_targets(
    &logs,
    "inventory_reconciliations",
    recon_id,
    catalog.base_alpha,
  );

  await_sync_cycle(&client, &pa.url, &pa.token, SYNC_TIMEOUT).await;
  let recons = api_get(&client, &format!("{}/reconciliations", pa.url), &pa.token).await;
  assert!(
    recons
      .as_array()
      .unwrap()
      .iter()
      .any(|r| r["id"] == recon_id.to_string()),
    "PA should have reconciliation"
  );

  central.shutdown().await;
  pa.shutdown().await;
}
