use axum::http::StatusCode;
use sea_orm::{ColumnTrait, QueryFilter};
use voletu_core::{endpoints::paths as api_paths, entities::inventory_ledger_entry};

use crate::common::{
  catalog_seed::seed_inventory_catalog,
  http::{
    assert_api_error,
    assert_api_success,
    post_json,
    setup_seeded_app_with_admin_token,
    with_auth_token,
  },
  payloads::{
    transport_rail_intake_with_acceptance,
    transport_rail_manifest,
    transport_rail_measurement,
    transport_rail_waybill,
    transport_rail_weight,
  },
};

const RAIL_DOC_NUMBER: &str = "RW-100";

#[tokio::test]
async fn endpoints_create_waybill_manifest_measurement_and_weight_with_expected_response() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let catalog = seed_inventory_catalog(&db).await;

  with_auth_token(token, async {
    let waybill_res = post_json(
      &app,
      api_paths::transport::rail::WAYBILLS,
      transport_rail_waybill(
        RAIL_DOC_NUMBER,
        "2026-01-01",
        catalog.sender_id,
        catalog.base_id,
      ),
    )
    .await;
    let waybill_json = assert_api_success(waybill_res).await;
    assert_eq!(waybill_json["data"]["documentNumber"], RAIL_DOC_NUMBER);

    let rail_waybill_id =
      uuid::Uuid::parse_str(waybill_json["data"]["id"].as_str().unwrap()).unwrap();

    let manifest_res = post_json(
      &app,
      api_paths::transport::rail::MANIFESTS,
      transport_rail_manifest(
        rail_waybill_id,
        "WG-1",
        catalog.product_a_id,
        "20.0",
        "0.8",
        "16.0",
      ),
    )
    .await;
    let manifest_json = assert_api_success(manifest_res).await;
    assert_eq!(
      manifest_json["data"]["railWaybillId"],
      rail_waybill_id.to_string()
    );
    assert_eq!(manifest_json["data"]["wagonNumber"], "WG-1");

    let manifest_id = uuid::Uuid::parse_str(manifest_json["data"]["id"].as_str().unwrap()).unwrap();

    let measurement_res = post_json(
      &app,
      api_paths::transport::rail::MEASUREMENTS,
      transport_rail_measurement(manifest_id, "2.0", "0.79", "15.8"),
    )
    .await;
    let measurement_json = assert_api_success(measurement_res).await;
    assert_eq!(
      measurement_json["data"]["wagonManifestId"],
      manifest_id.to_string()
    );
    assert_eq!(measurement_json["data"]["calculatedMass"], "15.8");

    let weight_res = post_json(
      &app,
      api_paths::transport::rail::WEIGHTS,
      transport_rail_weight(manifest_id, "40.0", "20.0", "20.0"),
    )
    .await;
    let weight_json = assert_api_success(weight_res).await;
    assert_eq!(
      weight_json["data"]["wagonManifestId"],
      manifest_id.to_string()
    );
    assert_eq!(weight_json["data"]["netProductWeight"], "20");
  })
  .await;
}

#[tokio::test]
async fn intake_save_endpoint_persists_nested_manifest_measurements_and_weights() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let catalog = seed_inventory_catalog(&db).await;

  with_auth_token(token, async {
    let response = post_json(
      &app,
      api_paths::transport::rail::SAVE,
      transport_rail_intake_with_acceptance(
        "RW-COMP-1",
        "2026-01-02",
        catalog.sender_id,
        catalog.base_id,
        "WAGON-C1",
        "AC-RAIL-1",
        catalog.product_a_id,
        catalog.contractor_a_id,
        catalog.storage_a_id,
        true,
      ),
    )
    .await;

    let body = assert_api_success(response).await;
    assert_eq!(body["data"]["waybill"]["documentNumber"], "RW-COMP-1");
    let manifests = body["data"]["wagonManifests"].as_array().unwrap();
    assert_eq!(manifests.len(), 1);
    assert_eq!(manifests[0]["measurements"].as_array().unwrap().len(), 1);
    assert_eq!(manifests[0]["weights"].as_array().unwrap().len(), 1);
    assert!(body["data"]["acceptance"].is_null());

    let ledger_row = inventory_ledger_entry::Entity::load()
      .filter(inventory_ledger_entry::Column::StorageId.eq(catalog.storage_a_id))
      .filter(inventory_ledger_entry::Column::ProductId.eq(catalog.product_a_id))
      .filter(inventory_ledger_entry::Column::ContractorId.eq(catalog.contractor_a_id))
      .one(&*db)
      .await
      .unwrap();
    assert!(ledger_row.is_none());
  })
  .await;
}

#[tokio::test]
async fn intake_save_endpoint_ignores_nested_acceptance_payload() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let catalog = seed_inventory_catalog(&db).await;

  with_auth_token(token, async {
    let response = post_json(
      &app,
      api_paths::transport::rail::SAVE,
      transport_rail_intake_with_acceptance(
        "RW-COMP-DRAFT-1",
        "2026-01-02",
        catalog.sender_id,
        catalog.base_id,
        "WAGON-D1",
        "AC-RAIL-DRAFT-1",
        catalog.product_a_id,
        catalog.contractor_a_id,
        catalog.storage_a_id,
        false,
      ),
    )
    .await;

    let body = assert_api_success(response).await;
    assert!(body["data"]["acceptance"].is_null());

    let ledger_row = inventory_ledger_entry::Entity::load()
      .filter(inventory_ledger_entry::Column::StorageId.eq(catalog.storage_a_id))
      .filter(inventory_ledger_entry::Column::ProductId.eq(catalog.product_a_id))
      .filter(inventory_ledger_entry::Column::ContractorId.eq(catalog.contractor_a_id))
      .one(&*db)
      .await
      .unwrap();
    assert!(ledger_row.is_none());
  })
  .await;
}

#[tokio::test]
async fn waybill_create_endpoint_rejects_invalid_date_format_with_validation_error() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let catalog = seed_inventory_catalog(&db).await;

  with_auth_token(token, async {
    let invalid = post_json(
      &app,
      api_paths::transport::rail::WAYBILLS,
      transport_rail_waybill("RW-BAD", "bad-date", catalog.sender_id, catalog.base_id),
    )
    .await;
    let err_json = assert_api_error(
      invalid,
      StatusCode::UNPROCESSABLE_ENTITY,
      "VALIDATION_ERROR",
      Some("date"),
    )
    .await;
    assert_eq!(err_json["error"]["code"], "VALIDATION_ERROR");
  })
  .await;
}
