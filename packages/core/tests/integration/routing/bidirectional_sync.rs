use std::time::Duration;

use super::parse_doc_id;
use crate::common::integration::{
  create_acceptance_via_api,
  get_acceptance_composite_json,
  poll_until,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
};

#[tokio::test]
async fn peripheral_to_central_to_peripheral() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r5-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  let pa = setup_peripheral_via_api(&client, &temp_db_path("r5-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;
  let pb = setup_peripheral_via_api(&client, &temp_db_path("r5-pb"), &central, &[
    catalog.base_alpha
  ])
  .await;

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

  poll_until(
    || {
      let c = client.clone();
      let url = pb.url.clone();
      let tok = pb.token.clone();
      async move {
        get_acceptance_composite_json(&c, &url, &tok, acc_id)
          .await
          .is_some()
      }
    },
    Duration::from_secs(20),
    "acceptance should propagate from PA to PB",
  )
  .await;

  let central_acc =
    get_acceptance_composite_json(&client, &central.url, &central.token, acc_id).await;
  assert!(central_acc.is_some(), "Central should have doc after push");

  let pb_acc = get_acceptance_composite_json(&client, &pb.url, &pb.token, acc_id).await;
  assert!(
    pb_acc.is_some(),
    "PB should have doc from PA via Central relay"
  );

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
