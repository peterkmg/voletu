use axum::http::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use voletu_core::{
  endpoints::paths as api_paths,
  entities::{inventory_ledger_entry, rail_wagon_manifest, rail_waybill, truck_waybill},
};

use crate::common::{
  fixtures::seed_inventory_fixture,
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
    transport_truck_intake_save,
    transport_truck_item,
    transport_truck_waybill,
    transport_truck_weight_doc,
  },
};

const TRUCK_DOC_NUMBER: &str = "TW-100";
const RAIL_DOC_NUMBER: &str = "RW-100";

#[tokio::test]
async fn truck_transport_endpoints_create_waybill_item_and_weight_document_with_expected_response_data(
) {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let fixture = seed_inventory_fixture(&db).await;

  with_auth_token(token, async {
    let waybill_res = post_json(
      &app,
      api_paths::transport::truck::WAYBILLS,
      transport_truck_waybill(
        TRUCK_DOC_NUMBER,
        "2026-01-01",
        fixture.sender_id,
        fixture.base_id,
      ),
    )
    .await;
    let waybill_json = assert_api_success(waybill_res).await;
    assert_eq!(waybill_json["data"]["documentNumber"], TRUCK_DOC_NUMBER);
    assert_eq!(
      waybill_json["data"]["senderId"],
      fixture.sender_id.to_string()
    );

    let truck_waybill_id = truck_waybill::Entity::find()
      .filter(truck_waybill::Column::DocumentNumber.eq(TRUCK_DOC_NUMBER))
      .one(&*db)
      .await
      .unwrap()
      .unwrap()
      .id;

    let item_res = post_json(
      &app,
      api_paths::transport::truck::ITEMS,
      transport_truck_item(truck_waybill_id, fixture.product_a_id, "12.5"),
    )
    .await;
    let item_json = assert_api_success(item_res).await;
    assert_eq!(
      item_json["data"]["truckWaybillId"],
      truck_waybill_id.to_string()
    );
    assert_eq!(
      item_json["data"]["productId"],
      fixture.product_a_id.to_string()
    );

    let weight_res = post_json(
      &app,
      api_paths::transport::truck::WEIGHT_DOCS,
      transport_truck_weight_doc(truck_waybill_id, "13.0"),
    )
    .await;
    let weight_json = assert_api_success(weight_res).await;
    assert_eq!(
      weight_json["data"]["truckWaybillId"],
      truck_waybill_id.to_string()
    );
    assert_eq!(weight_json["data"]["totalWeight"], "13");
  })
  .await;
}

#[tokio::test]
async fn rail_transport_endpoints_create_waybill_manifest_measurement_and_weight_with_expected_response_data(
) {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let fixture = seed_inventory_fixture(&db).await;

  with_auth_token(token, async {
    let waybill_res = post_json(
      &app,
      api_paths::transport::rail::WAYBILLS,
      transport_rail_waybill(
        RAIL_DOC_NUMBER,
        "2026-01-01",
        fixture.sender_id,
        fixture.base_id,
      ),
    )
    .await;
    let waybill_json = assert_api_success(waybill_res).await;
    assert_eq!(waybill_json["data"]["documentNumber"], RAIL_DOC_NUMBER);

    let rail_waybill_id = rail_waybill::Entity::find()
      .filter(rail_waybill::Column::DocumentNumber.eq(RAIL_DOC_NUMBER))
      .one(&*db)
      .await
      .unwrap()
      .unwrap()
      .id;

    let manifest_res = post_json(
      &app,
      api_paths::transport::rail::MANIFESTS,
      transport_rail_manifest(
        rail_waybill_id,
        "WG-1",
        fixture.product_a_id,
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

    let manifest_id = rail_wagon_manifest::Entity::find()
      .filter(rail_wagon_manifest::Column::RailWaybillId.eq(rail_waybill_id))
      .one(&*db)
      .await
      .unwrap()
      .unwrap()
      .id;

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
async fn truck_intake_complete_endpoint_supports_optional_nested_sections_being_omitted() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let fixture = seed_inventory_fixture(&db).await;

  with_auth_token(token, async {
    let response = post_json(
      &app,
      api_paths::transport::truck::SAVE,
      transport_truck_intake_save(
        "TW-COMP-1",
        "2026-01-02",
        fixture.sender_id,
        fixture.base_id,
        fixture.product_a_id,
        "12.5",
      ),
    )
    .await;

    let body = assert_api_success(response).await;
    assert_eq!(body["data"]["waybill"]["documentNumber"], "TW-COMP-1");
    assert_eq!(body["data"]["items"].as_array().unwrap().len(), 1);
    assert!(body["data"]["weightDoc"].is_null());
    assert!(body["data"]["acceptance"].is_null());
  })
  .await;
}

#[tokio::test]
async fn rail_intake_save_endpoint_persists_nested_manifest_measurements_and_weights() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let fixture = seed_inventory_fixture(&db).await;

  with_auth_token(token, async {
    let response = post_json(
      &app,
      api_paths::transport::rail::SAVE,
      transport_rail_intake_with_acceptance(
        "RW-COMP-1",
        "2026-01-02",
        fixture.sender_id,
        fixture.base_id,
        "WAGON-C1",
        "AC-RAIL-1",
        fixture.product_a_id,
        fixture.contractor_a_id,
        fixture.storage_a_id,
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

    let _rail_waybill_id = rail_waybill::Entity::find()
      .filter(rail_waybill::Column::DocumentNumber.eq("RW-COMP-1"))
      .one(&*db)
      .await
      .unwrap()
      .unwrap()
      .id;
    let ledger_row = inventory_ledger_entry::Entity::find()
      .filter(inventory_ledger_entry::Column::StorageId.eq(fixture.storage_a_id))
      .filter(inventory_ledger_entry::Column::ProductId.eq(fixture.product_a_id))
      .filter(inventory_ledger_entry::Column::ContractorId.eq(fixture.contractor_a_id))
      .one(&*db)
      .await
      .unwrap();
    assert!(ledger_row.is_none());
  })
  .await;
}

#[tokio::test]
async fn rail_intake_save_endpoint_ignores_nested_acceptance_payload() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let fixture = seed_inventory_fixture(&db).await;

  with_auth_token(token, async {
    let response = post_json(
      &app,
      api_paths::transport::rail::SAVE,
      transport_rail_intake_with_acceptance(
        "RW-COMP-DRAFT-1",
        "2026-01-02",
        fixture.sender_id,
        fixture.base_id,
        "WAGON-D1",
        "AC-RAIL-DRAFT-1",
        fixture.product_a_id,
        fixture.contractor_a_id,
        fixture.storage_a_id,
        false,
      ),
    )
    .await;

    let body = assert_api_success(response).await;
    assert!(body["data"]["acceptance"].is_null());

    let ledger_row = inventory_ledger_entry::Entity::find()
      .filter(inventory_ledger_entry::Column::StorageId.eq(fixture.storage_a_id))
      .filter(inventory_ledger_entry::Column::ProductId.eq(fixture.product_a_id))
      .filter(inventory_ledger_entry::Column::ContractorId.eq(fixture.contractor_a_id))
      .one(&*db)
      .await
      .unwrap();
    assert!(ledger_row.is_none());
  })
  .await;
}

#[tokio::test]
async fn truck_waybill_create_endpoint_rejects_invalid_date_format_with_validation_error_payload() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let fixture = seed_inventory_fixture(&db).await;

  with_auth_token(token, async {
    let invalid = post_json(
      &app,
      api_paths::transport::truck::WAYBILLS,
      transport_truck_waybill("TW-BAD", "2026-99-99", fixture.sender_id, fixture.base_id),
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

#[tokio::test]
async fn rail_waybill_create_endpoint_rejects_invalid_date_format_with_validation_error_payload() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let fixture = seed_inventory_fixture(&db).await;

  with_auth_token(token, async {
    let invalid = post_json(
      &app,
      api_paths::transport::rail::WAYBILLS,
      transport_rail_waybill("RW-BAD", "bad-date", fixture.sender_id, fixture.base_id),
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
