use serde_json::Value;

use super::parse_doc_id;
use crate::common::integration::{
  assert_audit_log_targets,
  create_acceptance_via_api,
  query_audit_logs,
  seed_catalog_via_api,
  setup_central_via_api,
  temp_db_path,
};

#[tokio::test]
async fn audit_log_includes_storage_base_for_all_composite_items() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r1-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  let acc = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-R1-001",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "100.0",
  )
  .await;
  let doc_id = parse_doc_id(&acc);

  let logs = query_audit_logs(
    &client,
    &central.url,
    &central.token,
    Some("acceptance_documents"),
    Some(doc_id),
  )
  .await;
  assert!(
    !logs.is_empty(),
    "expected audit log for acceptance_documents/{doc_id}"
  );
  assert_audit_log_targets(&logs, "acceptance_documents", doc_id, catalog.base_alpha);

  let item_logs = query_audit_logs(
    &client,
    &central.url,
    &central.token,
    Some("acceptance_items"),
    None,
  )
  .await;
  let item_logs_for_doc: Vec<&Value> = item_logs
    .iter()
    .filter(|l| {
      l["newValuesJson"]
        .as_str()
        .map(|nv| nv.contains(&doc_id.to_string()))
        .unwrap_or(false)
    })
    .collect();
  assert!(!item_logs_for_doc.is_empty());
  for il in &item_logs_for_doc {
    let target = il["targetBaseIds"].as_str().unwrap_or("");
    assert!(
      target.contains(&catalog.base_alpha.to_string()),
      "item should target base_alpha, got: {target}"
    );
  }

  central.shutdown().await;
}
