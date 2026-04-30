use serde_json::json;
use uuid::Uuid;
use voletu_core::endpoints::paths as api_paths;

use crate::{
  common::{
    http::{
      assert_api_success,
      get,
      post_json,
      setup_seeded_app_with_admin_token,
      with_auth_token,
    },
    payloads::transport_truck_waybill,
  },
  documents::seed_inventory_context,
};

const ACCEPTANCE_NAMES_DOC_NUMBER: &str = "ACC-NAMES-1";
const ACCEPTANCE_NAMES_WAYBILL_NUMBER: &str = "TWB-ACC-NAMES-1";

#[tokio::test]
async fn endpoints_return_related_names_with_embed_param() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let ctx = seed_inventory_context(&db).await;

  with_auth_token(token, async {
    let truck_waybill = post_json(
      &app,
      api_paths::transport::truck::SAVE,
      transport_truck_waybill(
        ACCEPTANCE_NAMES_WAYBILL_NUMBER,
        "2026-01-06",
        ctx.contractor_id,
        ctx.base_id,
      ),
    )
    .await;
    let truck_waybill_json = assert_api_success(truck_waybill).await;
    let truck_waybill_id = Uuid::parse_str(
      truck_waybill_json["data"]["waybill"]["id"]
        .as_str()
        .unwrap(),
    )
    .unwrap();

    let acceptance = post_json(
      &app,
      api_paths::acceptance::COMPOSITE_SAVE,
      json!({
        "documentNumber": ACCEPTANCE_NAMES_DOC_NUMBER,
        "dateAccepted": "2026-01-06T01:00:00Z",
        "arrivalType": "TRUCK",
        "contractorId": ctx.contractor_id,
        "truckWaybillId": truck_waybill_id,
        "items": [
          {
            "productId": ctx.product_id,
            "storageId": ctx.storage_a,
            "acceptedAmount": "3.0"
          }
        ]
      })
      .to_string(),
    )
    .await;
    let acceptance_json = assert_api_success(acceptance).await;
    let acceptance_id = Uuid::parse_str(acceptance_json["data"]["id"].as_str().unwrap()).unwrap();

    let list = get(&app, format!("{}?embed=names", api_paths::acceptance::ROOT)).await;
    let list_json = assert_api_success(list).await;
    let list_row = list_json["data"]
      .as_array()
      .unwrap()
      .iter()
      .find(|row| row["id"] == acceptance_id.to_string())
      .unwrap();
    assert_eq!(list_row["contractorIdName"], "Contractor A");
    assert_eq!(
      list_row["truckWaybillIdName"],
      ACCEPTANCE_NAMES_WAYBILL_NUMBER
    );

    let query = get(
      &app,
      format!(
        "{}?documentNumber={}&embed=names",
        api_paths::acceptance::QUERY,
        ACCEPTANCE_NAMES_DOC_NUMBER
      ),
    )
    .await;
    let query_json = assert_api_success(query).await;
    let query_row = &query_json["data"].as_array().unwrap()[0];
    assert_eq!(query_row["contractorIdName"], "Contractor A");
    assert_eq!(
      query_row["truckWaybillIdName"],
      ACCEPTANCE_NAMES_WAYBILL_NUMBER
    );

    let get_document = get(
      &app,
      format!(
        "{}?embed=names",
        api_paths::acceptance::BY_ID.replace("{id}", &acceptance_id.to_string())
      ),
    )
    .await;
    let get_document_json = assert_api_success(get_document).await;
    assert_eq!(
      get_document_json["data"]["contractorIdName"],
      "Contractor A"
    );
    assert_eq!(
      get_document_json["data"]["truckWaybillIdName"],
      ACCEPTANCE_NAMES_WAYBILL_NUMBER
    );

    let get_composite = get(
      &app,
      format!(
        "{}?embed=names",
        api_paths::acceptance::COMPOSITE_BY_ID.replace("{id}", &acceptance_id.to_string())
      ),
    )
    .await;
    let get_composite_json = assert_api_success(get_composite).await;
    assert_eq!(
      get_composite_json["data"]["contractorIdName"],
      "Contractor A"
    );
    assert_eq!(
      get_composite_json["data"]["truckWaybillIdName"],
      ACCEPTANCE_NAMES_WAYBILL_NUMBER
    );
    assert_eq!(
      get_composite_json["data"]["items"][0]["productIdName"],
      "Product A"
    );
    assert_eq!(
      get_composite_json["data"]["items"][0]["storageIdName"],
      "Tank A"
    );
  })
  .await;
}
