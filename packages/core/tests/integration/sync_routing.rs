//! Routing integration tests — verify that audit log routing envelopes are correctly
//! populated and that sync distributes records to the right peripheral nodes.
//!
//! **All infrastructure and business operations use HTTP API exclusively.**
//! No direct DB access, no inject_targeted_sync_log — everything exercises the full stack:
//! spawn server → /node/initialize → seed catalog → create documents → push/pull → verify.

use std::time::Duration;

use reqwest::Client;
use serde_json::Value;
use uuid::Uuid;

use crate::common::sync_integration::{
  add_base_assignment_via_api,
  api_get,
  api_post,
  assert_audit_log_targets,
  create_acceptance_via_api,
  create_blending_via_api,
  create_dispatch_via_api,
  create_ownership_transfer_via_api,
  create_physical_transfer_via_api,
  create_reconciliation_via_api,
  dev_seed_via_api,
  execute_document_via_api,
  get_acceptance_composite_json,
  get_all_ledger_entries,
  get_composite_json,
  get_physical_transfer_composite_json,
  get_storages_for_base,
  hard_delete_via_api,
  has_catalog_entity,
  pull_from_central_to_target,
  pull_from_central_to_target_after,
  push_outbound_to_central,
  query_audit_logs,
  revert_document_via_api,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  soft_delete_via_api,
  temp_db_path,
};

const INITIAL_AUDIT_CURSOR: Uuid = Uuid::from_u128(1);

// ===========================================================================
// Test 1: Routing envelope population
// ===========================================================================
// Create an acceptance document on Central → verify audit logs have correct target_base_ids.

#[tokio::test]
async fn routing_envelope_populated_on_composite_document_create() {
  let client = Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r1-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  // Create acceptance with item referencing storage_alpha → should route to base_alpha
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

  // Verify document INSERT audit log has base_alpha in target_base_ids
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

  // Verify item audit logs also have correct routing
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

// ===========================================================================
// Test 2: Routing isolation (CORE)
// ===========================================================================
// Central has docs for base_alpha and base_beta.
// Peripheral A (base_alpha) gets ONLY alpha docs.
// Peripheral B (base_beta) gets ONLY beta docs.

#[tokio::test]
async fn routing_isolation_peripheral_gets_only_its_base_documents() {
  let client = Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r2-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  let pa = setup_peripheral_via_api(&client, &temp_db_path("r2-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;
  let pb = setup_peripheral_via_api(&client, &temp_db_path("r2-pb"), &central, &[
    catalog.base_beta
  ])
  .await;

  // Create two documents: one for alpha, one for beta
  let acc_alpha = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-ALPHA",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "50.0",
  )
  .await;
  let acc_alpha_id = parse_doc_id(&acc_alpha);

  let acc_beta = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-BETA",
    catalog.contractor,
    catalog.product,
    catalog.storage_beta,
    "75.0",
  )
  .await;
  let acc_beta_id = parse_doc_id(&acc_beta);

  // Pull to each peripheral
  let _ = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;
  let _ = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pb.url,
    &pb.token,
    &[catalog.base_beta],
  )
  .await;

  // PA: has alpha, NOT beta
  assert!(
    get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_alpha_id)
      .await
      .is_some(),
    "PA should have alpha doc"
  );
  assert!(
    get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_beta_id)
      .await
      .is_none(),
    "PA should NOT have beta doc"
  );

  // PB: has beta, NOT alpha
  assert!(
    get_acceptance_composite_json(&client, &pb.url, &pb.token, acc_beta_id)
      .await
      .is_some(),
    "PB should have beta doc"
  );
  assert!(
    get_acceptance_composite_json(&client, &pb.url, &pb.token, acc_alpha_id)
      .await
      .is_none(),
    "PB should NOT have alpha doc"
  );

  // Field parity: alpha on Central == alpha on PA
  let central_alpha =
    get_acceptance_composite_json(&client, &central.url, &central.token, acc_alpha_id)
      .await
      .unwrap();
  let pa_alpha = get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_alpha_id)
    .await
    .unwrap();
  assert_eq!(central_alpha["documentNumber"], pa_alpha["documentNumber"]);
  assert_eq!(
    central_alpha["items"][0]["acceptedAmount"],
    pa_alpha["items"][0]["acceptedAmount"]
  );

  central.shutdown().await;
  pa.shutdown().await;
  pb.shutdown().await;
}

// ===========================================================================
// Test 3: Cross-base physical transfer
// ===========================================================================
// Physical transfer from storage_alpha → storage_beta routes to BOTH bases.

#[tokio::test]
async fn cross_base_physical_transfer_routes_to_both_peripherals() {
  let client = Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r3-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  let pa = setup_peripheral_via_api(&client, &temp_db_path("r3-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;
  let pb = setup_peripheral_via_api(&client, &temp_db_path("r3-pb"), &central, &[
    catalog.base_beta
  ])
  .await;

  let transfer = create_physical_transfer_via_api(
    &client,
    &central.url,
    &central.token,
    "PHYS-CROSS",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    catalog.storage_beta,
    "200.0",
  )
  .await;
  let transfer_id = Uuid::parse_str(transfer["id"].as_str().unwrap()).unwrap();

  // Verify routing has BOTH bases
  let logs = query_audit_logs(
    &client,
    &central.url,
    &central.token,
    Some("physical_storage_transfers"),
    Some(transfer_id),
  )
  .await;
  assert!(!logs.is_empty());
  for log in &logs {
    let target = log["targetBaseIds"].as_str().unwrap_or("");
    assert!(
      target.contains(&catalog.base_alpha.to_string()),
      "should contain alpha, got: {target}"
    );
    assert!(
      target.contains(&catalog.base_beta.to_string()),
      "should contain beta, got: {target}"
    );
  }

  // Both peripherals get it
  let _ = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;
  let _ = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pb.url,
    &pb.token,
    &[catalog.base_beta],
  )
  .await;

  let pa_t = get_physical_transfer_composite_json(&client, &pa.url, &pa.token, transfer_id).await;
  let pb_t = get_physical_transfer_composite_json(&client, &pb.url, &pb.token, transfer_id).await;
  assert!(pa_t.is_some(), "PA should have cross-base transfer");
  assert!(pb_t.is_some(), "PB should have cross-base transfer");

  // Field parity
  let central_t =
    get_physical_transfer_composite_json(&client, &central.url, &central.token, transfer_id)
      .await
      .unwrap();
  assert_eq!(
    central_t["items"][0]["amount"],
    pa_t.unwrap()["items"][0]["amount"]
  );
  assert_eq!(
    central_t["items"][0]["amount"],
    pb_t.unwrap()["items"][0]["amount"]
  );

  central.shutdown().await;
  pa.shutdown().await;
  pb.shutdown().await;
}

// ===========================================================================
// Test 4: Catalog broadcast
// ===========================================================================
// Catalog entities reach ALL peripherals regardless of base assignment.

#[tokio::test]
async fn catalog_broadcast_reaches_all_peripherals() {
  let client = Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r4-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  let pa = setup_peripheral_via_api(&client, &temp_db_path("r4-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;
  let pb = setup_peripheral_via_api(&client, &temp_db_path("r4-pb"), &central, &[
    catalog.base_beta
  ])
  .await;

  // Pull with base assignments — catalog should come through as global
  let _ = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;
  let _ = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pb.url,
    &pb.token,
    &[catalog.base_beta],
  )
  .await;

  // Products are global — both have it
  assert!(
    has_catalog_entity(
      &client,
      &pa.url,
      &pa.token,
      "/catalog/products",
      catalog.product
    )
    .await
  );
  assert!(
    has_catalog_entity(
      &client,
      &pb.url,
      &pb.token,
      "/catalog/products",
      catalog.product
    )
    .await
  );

  // Companies are global
  assert!(
    has_catalog_entity(
      &client,
      &pa.url,
      &pa.token,
      "/catalog/companies",
      catalog.contractor
    )
    .await
  );
  assert!(
    has_catalog_entity(
      &client,
      &pb.url,
      &pb.token,
      "/catalog/companies",
      catalog.contractor
    )
    .await
  );

  // Bases are global — PA has beta's base, PB has alpha's base
  assert!(
    has_catalog_entity(
      &client,
      &pa.url,
      &pa.token,
      "/catalog/bases",
      catalog.base_beta
    )
    .await
  );
  assert!(
    has_catalog_entity(
      &client,
      &pb.url,
      &pb.token,
      "/catalog/bases",
      catalog.base_alpha
    )
    .await
  );

  // Storages are global — cross-base visibility
  assert!(
    has_catalog_entity(
      &client,
      &pa.url,
      &pa.token,
      "/catalog/storages",
      catalog.storage_beta
    )
    .await
  );
  assert!(
    has_catalog_entity(
      &client,
      &pb.url,
      &pb.token,
      "/catalog/storages",
      catalog.storage_alpha
    )
    .await
  );

  central.shutdown().await;
  pa.shutdown().await;
  pb.shutdown().await;
}

// ===========================================================================
// Test 5: Bidirectional sync (A → Central → B)
// ===========================================================================

#[tokio::test]
async fn bidirectional_sync_peripheral_to_central_to_peripheral() {
  let client = Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r5-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  // Both peripherals handle base_alpha
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r5-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;
  let pb = setup_peripheral_via_api(&client, &temp_db_path("r5-pb"), &central, &[
    catalog.base_alpha
  ])
  .await;

  // Sync catalog to PA so it can create documents
  let _ = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;

  // PA creates an acceptance document
  let acc = create_acceptance_via_api(
    &client,
    &pa.url,
    &pa.token,
    "ACC-PA-001",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "333.0",
  )
  .await;
  let acc_id = parse_doc_id(&acc);

  // Push PA → Central
  let pushed = push_outbound_to_central(
    &client,
    &pa.url,
    &pa.token,
    &central.url,
    &central.token,
    INITIAL_AUDIT_CURSOR,
  )
  .await;
  assert!(
    pushed > 0,
    "should push at least one log from PA to Central"
  );

  // Central now has it
  let central_acc =
    get_acceptance_composite_json(&client, &central.url, &central.token, acc_id).await;
  assert!(central_acc.is_some(), "Central should have doc after push");

  // Pull Central → PB
  let _ = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pb.url,
    &pb.token,
    &[catalog.base_alpha],
  )
  .await;

  // PB has the doc created on PA
  let pb_acc = get_acceptance_composite_json(&client, &pb.url, &pb.token, acc_id).await;
  assert!(
    pb_acc.is_some(),
    "PB should have doc from PA via Central relay"
  );

  // Field parity across all three nodes
  let pa_doc = get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_id)
    .await
    .unwrap();
  let pb_doc = pb_acc.unwrap();
  assert_eq!(pa_doc["documentNumber"], pb_doc["documentNumber"]);
  assert_eq!(
    pa_doc["items"][0]["acceptedAmount"],
    pb_doc["items"][0]["acceptedAmount"]
  );

  central.shutdown().await;
  pa.shutdown().await;
  pb.shutdown().await;
}

// ===========================================================================
// Test 6: Multi-base node
// ===========================================================================
// Peripheral A handles alpha+beta → gets docs for both, NOT gamma.

#[tokio::test]
async fn multi_base_node_pulls_all_assigned_bases() {
  let client = Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r6-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  // Create gamma base + warehouse + storage on Central
  let base_gamma = api_post(
    &client,
    &format!("{}/catalog/bases", central.url),
    &central.token,
    serde_json::json!({"commonName": "Base Gamma", "longName": null}),
  )
  .await;
  let base_gamma_id = Uuid::parse_str(base_gamma["id"].as_str().unwrap()).unwrap();
  let wh_gamma = api_post(
    &client,
    &format!("{}/catalog/warehouses", central.url),
    &central.token,
    serde_json::json!({"baseId": base_gamma_id, "commonName": "WH Gamma", "longName": null}),
  )
  .await;
  let wh_gamma_id = Uuid::parse_str(wh_gamma["id"].as_str().unwrap()).unwrap();
  let st_gamma = api_post(&client, &format!("{}/catalog/storages", central.url), &central.token,
    serde_json::json!({"warehouseId": wh_gamma_id, "commonName": "Tank Gamma", "longName": null, "capacity": null, "isTypeSpecific": false, "productTypeId": null})).await;
  let storage_gamma_id = Uuid::parse_str(st_gamma["id"].as_str().unwrap()).unwrap();

  // PA handles alpha + beta (not gamma) — set up with alpha, add beta via API
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r6-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;
  add_base_assignment_via_api(&client, &pa.url, &pa.token, catalog.base_beta).await;

  // Create documents for all three bases
  let acc_a = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-A",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "10.0",
  )
  .await;
  let acc_b = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-B",
    catalog.contractor,
    catalog.product,
    catalog.storage_beta,
    "20.0",
  )
  .await;
  let acc_g = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-G",
    catalog.contractor,
    catalog.product,
    storage_gamma_id,
    "30.0",
  )
  .await;
  let acc_a_id = parse_doc_id(&acc_a);
  let acc_b_id = parse_doc_id(&acc_b);
  let acc_g_id = parse_doc_id(&acc_g);

  // Pull with both assigned bases
  let _ = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha, catalog.base_beta],
  )
  .await;

  assert!(
    get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_a_id)
      .await
      .is_some(),
    "PA should have alpha doc"
  );
  assert!(
    get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_b_id)
      .await
      .is_some(),
    "PA should have beta doc"
  );
  assert!(
    get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_g_id)
      .await
      .is_none(),
    "PA should NOT have gamma doc"
  );

  central.shutdown().await;
  pa.shutdown().await;
}

// ===========================================================================
// Test 7: Catalog-only sync (no base assignment)
// ===========================================================================

#[tokio::test]
async fn catalog_only_sync_with_no_base_assignment() {
  let client = Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r7-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  // Create a document on Central
  let acc = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-CATONLY",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "99.0",
  )
  .await;
  let acc_id = parse_doc_id(&acc);

  // Peripheral with NO base assignments
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r7-pa"), &central, &[]).await;

  // Pull with empty base_ids → catalog only
  let (pulled, _) = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[],
  )
  .await;
  assert!(pulled > 0, "should pull catalog entities");

  assert!(
    has_catalog_entity(
      &client,
      &pa.url,
      &pa.token,
      "/catalog/products",
      catalog.product
    )
    .await,
    "should have product"
  );
  assert!(
    get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_id)
      .await
      .is_none(),
    "should NOT have acceptance doc"
  );

  central.shutdown().await;
  pa.shutdown().await;
}

// ===========================================================================
// Test 8: Data parity after full sync cycle
// ===========================================================================

#[tokio::test]
async fn data_parity_after_full_sync_cycle() {
  let client = Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r8-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r8-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;

  let acc = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-PAR-001",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "500.5",
  )
  .await;
  let acc_id = parse_doc_id(&acc);
  let transfer = create_physical_transfer_via_api(
    &client,
    &central.url,
    &central.token,
    "PHYS-PAR-001",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    catalog.storage_alpha,
    "100.25",
  )
  .await;
  let transfer_id = Uuid::parse_str(transfer["id"].as_str().unwrap()).unwrap();

  let _ = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;

  // Acceptance parity
  let c_acc = get_acceptance_composite_json(&client, &central.url, &central.token, acc_id)
    .await
    .unwrap();
  let p_acc = get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_id)
    .await
    .unwrap();
  assert_eq!(c_acc["id"], p_acc["id"]);
  assert_eq!(c_acc["documentNumber"], p_acc["documentNumber"]);
  assert_eq!(c_acc["dateAccepted"], p_acc["dateAccepted"]);
  assert_eq!(c_acc["contractorId"], p_acc["contractorId"]);
  assert_eq!(c_acc["status"], p_acc["status"]);
  let c_items = c_acc["items"].as_array().unwrap();
  let p_items = p_acc["items"].as_array().unwrap();
  assert_eq!(c_items.len(), p_items.len());
  for (ci, pi) in c_items.iter().zip(p_items.iter()) {
    assert_eq!(ci["id"], pi["id"]);
    assert_eq!(ci["productId"], pi["productId"]);
    assert_eq!(ci["storageId"], pi["storageId"]);
    assert_eq!(ci["acceptedAmount"], pi["acceptedAmount"]);
  }

  // Physical transfer parity
  let c_t =
    get_physical_transfer_composite_json(&client, &central.url, &central.token, transfer_id)
      .await
      .unwrap();
  let p_t = get_physical_transfer_composite_json(&client, &pa.url, &pa.token, transfer_id)
    .await
    .unwrap();
  assert_eq!(c_t["id"], p_t["id"]);
  assert_eq!(c_t["documentNumber"], p_t["documentNumber"]);
  assert_eq!(c_t["contractorId"], p_t["contractorId"]);
  for (ci, pi) in c_t["items"]
    .as_array()
    .unwrap()
    .iter()
    .zip(p_t["items"].as_array().unwrap().iter())
  {
    assert_eq!(ci["id"], pi["id"]);
    assert_eq!(ci["amount"], pi["amount"]);
    assert_eq!(ci["fromStorageId"], pi["fromStorageId"]);
    assert_eq!(ci["toStorageId"], pi["toStorageId"]);
  }

  central.shutdown().await;
  pa.shutdown().await;
}

// ===========================================================================
// Test 9: Dispatch routing with destination_base_id
// ===========================================================================

#[tokio::test]
async fn dispatch_routing_includes_destination_base() {
  let client = Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r9-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r9-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;
  let pb = setup_peripheral_via_api(&client, &temp_db_path("r9-pb"), &central, &[
    catalog.base_beta
  ])
  .await;

  // Pre-fill inventory so dispatch validation passes
  api_post(&client, &format!("{}/acceptance/composite/save-and-execute", central.url), &central.token,
    serde_json::json!({
      "documentNumber": "ACC-PRE", "dateAccepted": "2026-01-14T10:00:00Z", "arrivalType": "TRUCK",
      "sourceEntity": null, "contractorId": catalog.contractor, "truckWaybillId": null, "railWaybillId": null, "transitDispatchId": null,
      "items": [{"productId": catalog.product, "storageId": catalog.storage_alpha, "acceptedAmount": "500.0"}]
    })).await;

  // Dispatch from alpha with destination = beta → routes to BOTH
  let dispatch = create_dispatch_via_api(
    &client,
    &central.url,
    &central.token,
    "DISP-DEST",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "50.0",
    Some(catalog.base_beta),
  )
  .await;
  let dispatch_id = parse_doc_id(&dispatch);

  let logs = query_audit_logs(
    &client,
    &central.url,
    &central.token,
    Some("dispatch_documents"),
    Some(dispatch_id),
  )
  .await;
  assert!(!logs.is_empty());
  for log in &logs {
    let target = log["targetBaseIds"].as_str().unwrap_or("");
    assert!(
      target.contains(&catalog.base_alpha.to_string()),
      "should contain alpha, got: {target}"
    );
    assert!(
      target.contains(&catalog.base_beta.to_string()),
      "should contain beta, got: {target}"
    );
  }

  let _ = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;
  let _ = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pb.url,
    &pb.token,
    &[catalog.base_beta],
  )
  .await;
  assert!(get_composite_json(
    &client,
    &pa.url,
    &pa.token,
    "/dispatch/composite/{id}",
    dispatch_id
  )
  .await
  .is_some());
  assert!(get_composite_json(
    &client,
    &pb.url,
    &pb.token,
    "/dispatch/composite/{id}",
    dispatch_id
  )
  .await
  .is_some());

  central.shutdown().await;
  pa.shutdown().await;
  pb.shutdown().await;
}

// ===========================================================================
// Test 10: Blending routing across bases
// ===========================================================================

#[tokio::test]
async fn blending_routing_spans_component_and_result_bases() {
  let client = Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r10-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r10-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;
  let pb = setup_peripheral_via_api(&client, &temp_db_path("r10-pb"), &central, &[
    catalog.base_beta
  ])
  .await;

  let blending = create_blending_via_api(
    &client,
    &central.url,
    &central.token,
    "BLEND-CROSS",
    catalog.contractor,
    catalog.product_b,
    catalog.storage_alpha,
    catalog.product,
    "40.0",
    catalog.storage_beta,
    "38.0",
  )
  .await;
  let blending_id = Uuid::parse_str(
    blending["document"]["id"]
      .as_str()
      .unwrap_or(blending["id"].as_str().unwrap_or("")),
  )
  .unwrap();

  let logs = query_audit_logs(
    &client,
    &central.url,
    &central.token,
    Some("blending_documents"),
    Some(blending_id),
  )
  .await;
  assert!(!logs.is_empty());
  for log in &logs {
    let target = log["targetBaseIds"].as_str().unwrap_or("");
    assert!(
      target.contains(&catalog.base_alpha.to_string()),
      "should contain alpha (component), got: {target}"
    );
    assert!(
      target.contains(&catalog.base_beta.to_string()),
      "should contain beta (result), got: {target}"
    );
  }

  let _ = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;
  let _ = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pb.url,
    &pb.token,
    &[catalog.base_beta],
  )
  .await;
  assert!(get_composite_json(
    &client,
    &pa.url,
    &pa.token,
    "/blending/composite/{id}",
    blending_id
  )
  .await
  .is_some());
  assert!(get_composite_json(
    &client,
    &pb.url,
    &pb.token,
    "/blending/composite/{id}",
    blending_id
  )
  .await
  .is_some());

  central.shutdown().await;
  pa.shutdown().await;
  pb.shutdown().await;
}

// ===========================================================================
// Test 11: Ownership transfer routing
// ===========================================================================

#[tokio::test]
async fn ownership_transfer_routing_via_storage() {
  let client = Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r11-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r11-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;
  let pb = setup_peripheral_via_api(&client, &temp_db_path("r11-pb"), &central, &[
    catalog.base_beta
  ])
  .await;

  let transfer = create_ownership_transfer_via_api(
    &client,
    &central.url,
    &central.token,
    catalog.storage_alpha,
    catalog.product,
    catalog.contractor,
    catalog.contractor_b,
    "100.0",
  )
  .await;
  let transfer_id = Uuid::parse_str(transfer["id"].as_str().unwrap()).unwrap();

  let logs = query_audit_logs(
    &client,
    &central.url,
    &central.token,
    Some("ownership_transfers"),
    Some(transfer_id),
  )
  .await;
  assert!(!logs.is_empty());
  assert_audit_log_targets(
    &logs,
    "ownership_transfers",
    transfer_id,
    catalog.base_alpha,
  );

  let _ = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;
  let _ = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pb.url,
    &pb.token,
    &[catalog.base_beta],
  )
  .await;
  assert!(get_composite_json(
    &client,
    &pa.url,
    &pa.token,
    "/ownership-transfers/composite/{id}",
    transfer_id
  )
  .await
  .is_some());
  assert!(get_composite_json(
    &client,
    &pb.url,
    &pb.token,
    "/ownership-transfers/composite/{id}",
    transfer_id
  )
  .await
  .is_none());

  central.shutdown().await;
  pa.shutdown().await;
  pb.shutdown().await;
}

// ===========================================================================
// Test 12: Reconciliation routing via warehouse_id
// ===========================================================================

#[tokio::test]
async fn reconciliation_routing_via_warehouse() {
  let client = Client::new();
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

  let _ = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;
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

// ===========================================================================
// Test 13: Execute produces correctly routed UPDATE audit log
// ===========================================================================

#[tokio::test]
async fn execute_produces_routed_update_audit_log() {
  let client = Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r13-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r13-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;

  // Create draft, then execute
  let acc = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-EXEC",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "200.0",
  )
  .await;
  let acc_id = parse_doc_id(&acc);
  execute_document_via_api(
    &client,
    &central.url,
    &central.token,
    "/acceptance/execute/{id}",
    acc_id,
  )
  .await;

  // Both INSERT and UPDATE audit logs should have routing
  let all_logs = query_audit_logs(
    &client,
    &central.url,
    &central.token,
    Some("acceptance_documents"),
    Some(acc_id),
  )
  .await;
  assert!(
    all_logs.len() >= 2,
    "expected INSERT + UPDATE audit logs, got {}",
    all_logs.len()
  );
  let base_str = catalog.base_alpha.to_string();
  for log in &all_logs {
    let action = log["action"].as_str().unwrap_or("");
    let target = log["targetBaseIds"].as_str().unwrap_or("");
    assert!(
      target.contains(&base_str),
      "action={action} should target base_alpha, got: '{target}'"
    );
  }

  // Peripheral gets the executed document
  let _ = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;
  let pa_acc = get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_id).await;
  assert!(pa_acc.is_some());
  assert_eq!(pa_acc.unwrap()["status"], "EXECUTED");

  central.shutdown().await;
  pa.shutdown().await;
}

// ===========================================================================
// Test 14: Incremental pull with watermark advancement
// ===========================================================================

#[tokio::test]
async fn incremental_pull_advances_watermark_correctly() {
  let client = Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r14-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r14-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;

  let _acc1 = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-INC-1",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "10.0",
  )
  .await;

  // First pull — catalog + first doc
  let (pull1_count, pull1_cursor) = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;
  assert!(pull1_count > 0);

  // Create second doc
  let acc2 = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-INC-2",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "20.0",
  )
  .await;
  let acc2_id = parse_doc_id(&acc2);

  // Incremental pull from cursor — only new data
  let (pull2_count, _) = pull_from_central_to_target_after(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
    pull1_cursor,
  )
  .await;
  assert!(pull2_count > 0, "incremental pull should return new data");
  assert!(
    pull2_count < pull1_count,
    "incremental should be smaller than initial"
  );
  assert!(
    get_acceptance_composite_json(&client, &pa.url, &pa.token, acc2_id)
      .await
      .is_some()
  );

  central.shutdown().await;
  pa.shutdown().await;
}

// ===========================================================================
// Test 15: Full seed → sync → ledger parity
// ===========================================================================
// Seeds Central via POST /dev/seed, syncs to Peripheral, verifies:
// - All catalog entities present on Peripheral
// - Ledger entries for the assigned base match exactly
// - Ledger entries for other bases do NOT exist on Peripheral
// - At least some business documents exist on Peripheral

#[tokio::test]
async fn seeded_database_syncs_to_peripheral_with_ledger_parity() {
  let client = Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r15-central")).await;

  // Seed Central with realistic data
  let seed_result = dev_seed_via_api(&client, &central.url, &central.token).await;
  let seeded_bases: usize = seed_result["bases"].as_u64().unwrap_or(0) as usize;
  assert!(
    seeded_bases >= 5,
    "seed should create at least 5 bases, got {seeded_bases}"
  );
  let seeded_ledger_entries: usize = seed_result["ledger_entries"].as_u64().unwrap_or(0) as usize;
  assert!(
    seeded_ledger_entries > 0,
    "seed should create ledger entries"
  );

  // Pick the first base from the catalog
  let bases = api_get(
    &client,
    &format!("{}/catalog/bases", central.url),
    &central.token,
  )
  .await;
  let all_bases: Vec<Uuid> = bases
    .as_array()
    .unwrap()
    .iter()
    .filter_map(|b| b["id"].as_str().and_then(|s| Uuid::parse_str(s).ok()))
    .collect();
  assert!(!all_bases.is_empty());
  let target_base_id = all_bases[0];

  // Determine which storages belong to this base
  let base_storage_ids =
    get_storages_for_base(&client, &central.url, &central.token, target_base_id).await;
  assert!(
    !base_storage_ids.is_empty(),
    "target base should have storages"
  );

  // Get Central ledger state (ground truth)
  let central_ledger = get_all_ledger_entries(&client, &central.url, &central.token).await;
  assert!(
    !central_ledger.is_empty(),
    "Central should have ledger entries after seed"
  );

  // Partition ledger by base scope
  let central_ledger_for_base: Vec<&Value> = central_ledger
    .iter()
    .filter(|entry| {
      entry["storageId"]
        .as_str()
        .and_then(|s| Uuid::parse_str(s).ok())
        .map(|sid| base_storage_ids.contains(&sid))
        .unwrap_or(false)
    })
    .collect();
  let central_ledger_other: Vec<&Value> = central_ledger
    .iter()
    .filter(|entry| {
      entry["storageId"]
        .as_str()
        .and_then(|s| Uuid::parse_str(s).ok())
        .map(|sid| !base_storage_ids.contains(&sid))
        .unwrap_or(true)
    })
    .collect();
  assert!(
    !central_ledger_for_base.is_empty(),
    "target base should have ledger entries"
  );

  // Setup Peripheral with this base and pull ALL data via incremental sync
  let peripheral = setup_peripheral_via_api(&client, &temp_db_path("r15-periph"), &central, &[
    target_base_id,
  ])
  .await;

  // Do repeated pulls until we've consumed all audit logs
  // (seed creates many entries, may require multiple pull batches)
  let mut total_pulled = 0usize;
  let mut cursor = Uuid::from_u128(1);
  loop {
    let (pulled, new_cursor) = pull_from_central_to_target_after(
      &client,
      &central.url,
      &central.token,
      &peripheral.url,
      &peripheral.token,
      &[target_base_id],
      cursor,
    )
    .await;
    total_pulled += pulled;
    if pulled == 0 || new_cursor == cursor {
      break;
    }
    cursor = new_cursor;
  }
  assert!(total_pulled > 0, "should have pulled data from Central");

  // --- Verify catalog entities present on Peripheral ---
  let periph_products = api_get(
    &client,
    &format!("{}/catalog/products", peripheral.url),
    &peripheral.token,
  )
  .await;
  let central_products = api_get(
    &client,
    &format!("{}/catalog/products", central.url),
    &central.token,
  )
  .await;
  assert_eq!(
    periph_products.as_array().unwrap().len(),
    central_products.as_array().unwrap().len(),
    "all products should sync (global catalog)"
  );

  let periph_companies = api_get(
    &client,
    &format!("{}/catalog/companies", peripheral.url),
    &peripheral.token,
  )
  .await;
  let central_companies = api_get(
    &client,
    &format!("{}/catalog/companies", central.url),
    &central.token,
  )
  .await;
  assert_eq!(
    periph_companies.as_array().unwrap().len(),
    central_companies.as_array().unwrap().len(),
    "all companies should sync (global catalog)"
  );

  // --- Verify ledger parity for assigned base ---
  let periph_ledger = get_all_ledger_entries(&client, &peripheral.url, &peripheral.token).await;

  // Every Central ledger entry for this base should exist on Peripheral with same amount
  for central_entry in &central_ledger_for_base {
    let storage_id = central_entry["storageId"].as_str().unwrap();
    let product_id = central_entry["productId"].as_str().unwrap();
    let contractor_id = central_entry["contractorId"].as_str().unwrap();
    let central_amount = &central_entry["currentAmount"];

    let periph_match = periph_ledger.iter().find(|pe| {
      pe["storageId"].as_str() == Some(storage_id)
        && pe["productId"].as_str() == Some(product_id)
        && pe["contractorId"].as_str() == Some(contractor_id)
    });

    assert!(
      periph_match.is_some(),
      "Peripheral missing ledger entry for storage={storage_id} product={product_id} contractor={contractor_id}"
    );
    assert_eq!(
      &periph_match.unwrap()["currentAmount"], central_amount,
      "ledger amount mismatch for storage={storage_id} product={product_id} contractor={contractor_id}"
    );
  }

  // Ledger entries for OTHER bases should NOT exist on Peripheral
  for other_entry in &central_ledger_other {
    let storage_id = other_entry["storageId"].as_str().unwrap();
    let product_id = other_entry["productId"].as_str().unwrap();
    let contractor_id = other_entry["contractorId"].as_str().unwrap();

    let should_not_exist = periph_ledger.iter().any(|pe| {
      pe["storageId"].as_str() == Some(storage_id)
        && pe["productId"].as_str() == Some(product_id)
        && pe["contractorId"].as_str() == Some(contractor_id)
    });

    assert!(
      !should_not_exist,
      "Peripheral should NOT have ledger entry for other-base storage={storage_id}"
    );
  }

  // --- Verify at least some business documents exist ---
  let periph_acceptance = api_get(
    &client,
    &format!("{}/acceptance", peripheral.url),
    &peripheral.token,
  )
  .await;
  assert!(
    periph_acceptance.as_array().unwrap().len() > 0,
    "Peripheral should have at least some acceptance documents after sync"
  );

  central.shutdown().await;
  peripheral.shutdown().await;
}

// ===========================================================================
// Helpers
// ===========================================================================

/// Extract document ID from composite response (handles both flattened and nested shapes).
fn parse_doc_id(response: &Value) -> Uuid {
  let id_str = response["id"]
    .as_str()
    .or_else(|| response["document"]["id"].as_str())
    .expect("response should have id or document.id");
  Uuid::parse_str(id_str).unwrap()
}

/// Pull all remaining logs from Central to target (loops until exhausted).
async fn pull_all(
  client: &Client,
  central_url: &str,
  central_token: &str,
  target_url: &str,
  target_token: &str,
  base_ids: &[Uuid],
) {
  let mut cursor = Uuid::from_u128(1);
  loop {
    let (pulled, new_cursor) = pull_from_central_to_target_after(
      client,
      central_url,
      central_token,
      target_url,
      target_token,
      base_ids,
      cursor,
    )
    .await;
    if pulled == 0 || new_cursor == cursor {
      break;
    }
    cursor = new_cursor;
  }
}

// ===========================================================================
// Test 16: Soft delete sync propagation
// ===========================================================================

#[tokio::test]
async fn soft_delete_propagates_via_sync() {
  let client = Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r16-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r16-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;

  // Create and sync a company
  let company = api_post(&client, &format!("{}/catalog/companies", central.url), &central.token,
    serde_json::json!({"commonName": "SoftDel Co", "legalName": null, "isContractor": true, "isExporter": false, "isManufacturer": false, "isSender": false})).await;
  let company_id = Uuid::parse_str(company["id"].as_str().unwrap()).unwrap();

  pull_all(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;
  assert!(
    has_catalog_entity(
      &client,
      &pa.url,
      &pa.token,
      "/catalog/companies",
      company_id
    )
    .await,
    "PA should have company"
  );

  // Soft-delete on Central
  soft_delete_via_api(
    &client,
    &central.url,
    &central.token,
    "/catalog/companies/{id}",
    company_id,
  )
  .await;

  // Verify soft-deleted on Central (no longer in active list)
  assert!(
    !has_catalog_entity(
      &client,
      &central.url,
      &central.token,
      "/catalog/companies",
      company_id
    )
    .await,
    "Central: company should be soft-deleted"
  );

  // Sync to PA
  pull_all(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;

  // PA should also not have it in active list
  assert!(
    !has_catalog_entity(
      &client,
      &pa.url,
      &pa.token,
      "/catalog/companies",
      company_id
    )
    .await,
    "PA: company should be soft-deleted after sync"
  );

  central.shutdown().await;
  pa.shutdown().await;
}

// ===========================================================================
// Test 17: Hard delete sync propagation
// ===========================================================================

#[tokio::test]
async fn hard_delete_propagates_via_sync() {
  let client = Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r17-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r17-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;

  // Create and sync a company
  let company = api_post(&client, &format!("{}/catalog/companies", central.url), &central.token,
    serde_json::json!({"commonName": "HardDel Co", "legalName": null, "isContractor": true, "isExporter": false, "isManufacturer": false, "isSender": false})).await;
  let company_id = Uuid::parse_str(company["id"].as_str().unwrap()).unwrap();

  pull_all(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;
  assert!(
    has_catalog_entity(
      &client,
      &pa.url,
      &pa.token,
      "/catalog/companies",
      company_id
    )
    .await
  );

  // Soft-delete first (required before hard-delete)
  soft_delete_via_api(
    &client,
    &central.url,
    &central.token,
    "/catalog/companies/{id}",
    company_id,
  )
  .await;
  // Hard-delete
  hard_delete_via_api(
    &client,
    &central.url,
    &central.token,
    "/catalog/companies/{id}/hard",
    company_id,
  )
  .await;

  // Sync to PA
  pull_all(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;

  // PA should not have it at all
  assert!(
    !has_catalog_entity(
      &client,
      &pa.url,
      &pa.token,
      "/catalog/companies",
      company_id
    )
    .await,
    "PA: company should be gone after hard delete sync"
  );

  central.shutdown().await;
  pa.shutdown().await;
}

// ===========================================================================
// Test 18: Full document lifecycle sync (Draft → Execute → Revert → Re-Execute)
// ===========================================================================

#[tokio::test]
async fn full_document_lifecycle_syncs_correctly() {
  let client = Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r18-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r18-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;

  // Create acceptance draft
  let acc = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-LIFE-001",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "300.0",
  )
  .await;
  let acc_id = parse_doc_id(&acc);

  // Step 1: Pull draft to PA → verify DRAFT
  pull_all(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;
  let pa_acc = get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_id)
    .await
    .unwrap();
  assert_eq!(pa_acc["status"], "DRAFT");

  // Step 2: Execute on Central → pull → verify EXECUTED
  execute_document_via_api(
    &client,
    &central.url,
    &central.token,
    "/acceptance/execute/{id}",
    acc_id,
  )
  .await;
  pull_all(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;
  let pa_acc = get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_id)
    .await
    .unwrap();
  assert_eq!(pa_acc["status"], "EXECUTED");

  // Step 3: Revert on Central → pull → verify DRAFT again
  revert_document_via_api(
    &client,
    &central.url,
    &central.token,
    "/acceptance/revert/{id}",
    acc_id,
  )
  .await;
  pull_all(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;
  let pa_acc = get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_id)
    .await
    .unwrap();
  assert_eq!(pa_acc["status"], "DRAFT");

  // Step 4: Re-execute → pull → verify EXECUTED again
  execute_document_via_api(
    &client,
    &central.url,
    &central.token,
    "/acceptance/execute/{id}",
    acc_id,
  )
  .await;
  pull_all(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;
  let pa_acc = get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_id)
    .await
    .unwrap();
  assert_eq!(pa_acc["status"], "EXECUTED");

  central.shutdown().await;
  pa.shutdown().await;
}

// ===========================================================================
// Test 19: Three-peripheral convergence
// ===========================================================================

#[tokio::test]
async fn three_peripheral_convergence_with_overlapping_bases() {
  let client = Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r19-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  // P1: alpha only, P2: beta only, P3: alpha+beta
  let p1 = setup_peripheral_via_api(&client, &temp_db_path("r19-p1"), &central, &[
    catalog.base_alpha
  ])
  .await;
  let p2 = setup_peripheral_via_api(&client, &temp_db_path("r19-p2"), &central, &[
    catalog.base_beta
  ])
  .await;
  let p3 = setup_peripheral_via_api(&client, &temp_db_path("r19-p3"), &central, &[
    catalog.base_alpha
  ])
  .await;
  add_base_assignment_via_api(&client, &p3.url, &p3.token, catalog.base_beta).await;

  // Doc for alpha only
  let acc_a = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-3P-A",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "10.0",
  )
  .await;
  let acc_a_id = parse_doc_id(&acc_a);

  // Doc for beta only
  let acc_b = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-3P-B",
    catalog.contractor,
    catalog.product,
    catalog.storage_beta,
    "20.0",
  )
  .await;
  let acc_b_id = parse_doc_id(&acc_b);

  // Cross-base physical transfer (alpha→beta)
  let cross = create_physical_transfer_via_api(
    &client,
    &central.url,
    &central.token,
    "PHYS-3P-CROSS",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    catalog.storage_beta,
    "5.0",
  )
  .await;
  let cross_id = Uuid::parse_str(cross["id"].as_str().unwrap()).unwrap();

  // Pull to all three
  pull_all(
    &client,
    &central.url,
    &central.token,
    &p1.url,
    &p1.token,
    &[catalog.base_alpha],
  )
  .await;
  pull_all(
    &client,
    &central.url,
    &central.token,
    &p2.url,
    &p2.token,
    &[catalog.base_beta],
  )
  .await;
  pull_all(
    &client,
    &central.url,
    &central.token,
    &p3.url,
    &p3.token,
    &[catalog.base_alpha, catalog.base_beta],
  )
  .await;

  // P1: alpha + cross, NOT beta
  assert!(
    get_acceptance_composite_json(&client, &p1.url, &p1.token, acc_a_id)
      .await
      .is_some()
  );
  assert!(
    get_acceptance_composite_json(&client, &p1.url, &p1.token, acc_b_id)
      .await
      .is_none()
  );
  assert!(
    get_physical_transfer_composite_json(&client, &p1.url, &p1.token, cross_id)
      .await
      .is_some()
  );

  // P2: beta + cross, NOT alpha
  assert!(
    get_acceptance_composite_json(&client, &p2.url, &p2.token, acc_a_id)
      .await
      .is_none()
  );
  assert!(
    get_acceptance_composite_json(&client, &p2.url, &p2.token, acc_b_id)
      .await
      .is_some()
  );
  assert!(
    get_physical_transfer_composite_json(&client, &p2.url, &p2.token, cross_id)
      .await
      .is_some()
  );

  // P3: ALL three documents
  assert!(
    get_acceptance_composite_json(&client, &p3.url, &p3.token, acc_a_id)
      .await
      .is_some()
  );
  assert!(
    get_acceptance_composite_json(&client, &p3.url, &p3.token, acc_b_id)
      .await
      .is_some()
  );
  assert!(
    get_physical_transfer_composite_json(&client, &p3.url, &p3.token, cross_id)
      .await
      .is_some()
  );

  central.shutdown().await;
  p1.shutdown().await;
  p2.shutdown().await;
  p3.shutdown().await;
}

// ===========================================================================
// Test 20: Soft delete undo propagates via sync
// ===========================================================================

#[tokio::test]
async fn soft_delete_undo_propagates_via_sync() {
  let client = Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r20-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r20-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;

  // Create company, sync, soft-delete, sync
  let company = api_post(&client, &format!("{}/catalog/companies", central.url), &central.token,
    serde_json::json!({"commonName": "Undo Co", "legalName": null, "isContractor": true, "isExporter": false, "isManufacturer": false, "isSender": false})).await;
  let company_id = Uuid::parse_str(company["id"].as_str().unwrap()).unwrap();

  pull_all(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;
  assert!(
    has_catalog_entity(
      &client,
      &pa.url,
      &pa.token,
      "/catalog/companies",
      company_id
    )
    .await
  );

  // Soft-delete, sync to PA
  soft_delete_via_api(
    &client,
    &central.url,
    &central.token,
    "/catalog/companies/{id}",
    company_id,
  )
  .await;
  pull_all(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;
  assert!(
    !has_catalog_entity(
      &client,
      &pa.url,
      &pa.token,
      "/catalog/companies",
      company_id
    )
    .await,
    "should be soft-deleted"
  );

  // Undo soft-delete (restore) on Central
  api_post(
    &client,
    &format!("{}/catalog/companies/{company_id}/restore", central.url),
    &central.token,
    serde_json::json!({}),
  )
  .await;

  // Sync to PA — entity should be active again
  pull_all(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;
  assert!(
    has_catalog_entity(
      &client,
      &pa.url,
      &pa.token,
      "/catalog/companies",
      company_id
    )
    .await,
    "should be restored after undo"
  );

  central.shutdown().await;
  pa.shutdown().await;
}

// ===========================================================================
// Test 21: Executed document with ledger effects syncs with ledger parity
// ===========================================================================

#[tokio::test]
async fn executed_document_syncs_with_ledger_parity() {
  let client = Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r21-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r21-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;

  // Create and execute acceptance (creates ledger entry)
  api_post(&client, &format!("{}/acceptance/composite/save-and-execute", central.url), &central.token,
    serde_json::json!({
      "documentNumber": "ACC-LED-001", "dateAccepted": "2026-01-15T10:00:00Z", "arrivalType": "TRUCK",
      "sourceEntity": null, "contractorId": catalog.contractor, "truckWaybillId": null, "railWaybillId": null, "transitDispatchId": null,
      "items": [{"productId": catalog.product, "storageId": catalog.storage_alpha, "acceptedAmount": "1234.56"}]
    })).await;

  // Check ledger on Central
  let central_ledger = get_all_ledger_entries(&client, &central.url, &central.token).await;
  let central_entry = central_ledger.iter().find(|e| {
    e["storageId"].as_str() == Some(&catalog.storage_alpha.to_string())
      && e["productId"].as_str() == Some(&catalog.product.to_string())
      && e["contractorId"].as_str() == Some(&catalog.contractor.to_string())
  });
  assert!(central_entry.is_some(), "Central should have ledger entry");
  let expected_amount = &central_entry.unwrap()["currentAmount"];

  // Sync to PA
  pull_all(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;

  // Verify ledger on PA matches
  let pa_ledger = get_all_ledger_entries(&client, &pa.url, &pa.token).await;
  let pa_entry = pa_ledger.iter().find(|e| {
    e["storageId"].as_str() == Some(&catalog.storage_alpha.to_string())
      && e["productId"].as_str() == Some(&catalog.product.to_string())
      && e["contractorId"].as_str() == Some(&catalog.contractor.to_string())
  });
  assert!(pa_entry.is_some(), "PA should have ledger entry after sync");
  assert_eq!(
    &pa_entry.unwrap()["currentAmount"],
    expected_amount,
    "ledger amount should match"
  );

  central.shutdown().await;
  pa.shutdown().await;
}
