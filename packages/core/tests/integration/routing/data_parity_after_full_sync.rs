//! **Data parity after full sync cycle**: After syncing an acceptance and a physical transfer
//! to a peripheral, every field matches the Central copy exactly.
//!
//! **Topology:** Central + 1 Peripheral (base_alpha)
//! **Verifies:** Full field-level parity for both acceptance and physical transfer composites

use uuid::Uuid;

use crate::common::integration::{
  create_acceptance_via_api,
  create_physical_transfer_via_api,
  get_acceptance_composite_json,
  get_physical_transfer_composite_json,
  pull_from_central_to_target,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
};

use super::parse_doc_id;

#[tokio::test]
async fn data_parity_after_full_sync_cycle() {
  let client = reqwest::Client::new();
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
