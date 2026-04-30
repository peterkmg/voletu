use std::time::Duration;

use super::parse_doc_id;
use crate::common::integration::{
  await_sync_cycle,
  create_acceptance_via_api,
  get_acceptance_composite_json,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
};

#[tokio::test]
async fn acceptance_scoped_to_alpha_absent_on_beta_peripheral() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("s3-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  let pa = setup_peripheral_via_api(&client, &temp_db_path("s3-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;
  let pb = setup_peripheral_via_api(&client, &temp_db_path("s3-pb"), &central, &[
    catalog.base_beta
  ])
  .await;

  let acc = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-ALPHA-ONLY",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "100.0",
  )
  .await;
  let acc_id = parse_doc_id(&acc);

  await_sync_cycle(&client, &pa.url, &pa.token, Duration::from_secs(15)).await;
  await_sync_cycle(&client, &pb.url, &pb.token, Duration::from_secs(15)).await;

  assert!(
    get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_id)
      .await
      .is_some(),
    "PA (base_alpha) should have the alpha-scoped acceptance"
  );

  assert!(
    get_acceptance_composite_json(&client, &pb.url, &pb.token, acc_id)
      .await
      .is_none(),
    "PB (base_beta) should NOT have the alpha-scoped acceptance"
  );

  assert!(
    get_acceptance_composite_json(&client, &central.url, &central.token, acc_id)
      .await
      .is_some(),
    "Central should have the acceptance"
  );

  central.shutdown().await;
  pa.shutdown().await;
  pb.shutdown().await;
}
