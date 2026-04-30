use std::time::Duration;

use uuid::Uuid;

use super::parse_doc_id;
use crate::common::integration::{
  add_base_assignment_via_api,
  await_sync_cycle,
  create_acceptance_via_api,
  create_physical_transfer_via_api,
  get_acceptance_composite_json,
  get_physical_transfer_composite_json,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
};

const SYNC_TIMEOUT: Duration = Duration::from_secs(15);

#[tokio::test]
async fn each_peripheral_receives_only_its_assigned_subset() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r19-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

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

  await_sync_cycle(&client, &p1.url, &p1.token, SYNC_TIMEOUT).await;
  await_sync_cycle(&client, &p2.url, &p2.token, SYNC_TIMEOUT).await;
  await_sync_cycle(&client, &p3.url, &p3.token, SYNC_TIMEOUT).await;

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
