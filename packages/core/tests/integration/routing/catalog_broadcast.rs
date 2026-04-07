//! **Catalog broadcast**: Catalog entities (products, companies, bases, storages) reach
//! all peripherals regardless of their base assignment.
//!
//! **Topology:** Central + 2 Peripherals (one base each)
//! **Verifies:** Global catalog entities are present on both peripherals, including cross-base entities

use crate::common::integration::{
  has_catalog_entity,
  pull_from_central_to_target,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
};

#[tokio::test]
async fn catalog_broadcast_reaches_all_peripherals() {
  let client = reqwest::Client::new();
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
