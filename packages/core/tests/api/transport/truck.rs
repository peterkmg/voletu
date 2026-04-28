use axum::http::StatusCode;
use voletu_core::endpoints::paths as api_paths;

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
    transport_truck_intake_save,
    transport_truck_item,
    transport_truck_waybill,
    transport_truck_weight_doc,
  },
};

const TRUCK_DOC_NUMBER: &str = "TW-100";

#[tokio::test]
async fn endpoints_create_waybill_item_and_weight_document_with_expected_response_data() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let catalog = seed_inventory_catalog(&db).await;

  with_auth_token(token, async {
    let waybill_res = post_json(
      &app,
      api_paths::transport::truck::WAYBILLS,
      transport_truck_waybill(
        TRUCK_DOC_NUMBER,
        "2026-01-01",
        catalog.sender_id,
        catalog.base_id,
      ),
    )
    .await;
    let waybill_json = assert_api_success(waybill_res).await;
    assert_eq!(waybill_json["data"]["documentNumber"], TRUCK_DOC_NUMBER);
    assert_eq!(
      waybill_json["data"]["senderId"],
      catalog.sender_id.to_string()
    );

    let truck_waybill_id =
      uuid::Uuid::parse_str(waybill_json["data"]["id"].as_str().unwrap()).unwrap();

    let item_res = post_json(
      &app,
      api_paths::transport::truck::ITEMS,
      transport_truck_item(truck_waybill_id, catalog.product_a_id, "12.5"),
    )
    .await;
    let item_json = assert_api_success(item_res).await;
    assert_eq!(
      item_json["data"]["truckWaybillId"],
      truck_waybill_id.to_string()
    );
    assert_eq!(
      item_json["data"]["productId"],
      catalog.product_a_id.to_string()
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
async fn intake_complete_endpoint_supports_optional_nested_sections_being_omitted() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let catalog = seed_inventory_catalog(&db).await;

  with_auth_token(token, async {
    let response = post_json(
      &app,
      api_paths::transport::truck::SAVE,
      transport_truck_intake_save(
        "TW-COMP-1",
        "2026-01-02",
        catalog.sender_id,
        catalog.base_id,
        catalog.product_a_id,
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
async fn waybill_create_endpoint_rejects_invalid_date_format_with_validation_error() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let catalog = seed_inventory_catalog(&db).await;

  with_auth_token(token, async {
    let invalid = post_json(
      &app,
      api_paths::transport::truck::WAYBILLS,
      transport_truck_waybill("TW-BAD", "2026-99-99", catalog.sender_id, catalog.base_id),
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
