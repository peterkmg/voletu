use sea_orm::prelude::Decimal;
use uuid::Uuid;
use voletu_core::endpoints::paths as api_paths;

use crate::{
  common::{
    catalog_seed::seed_ledger_balance,
    http::{
      assert_api_success,
      get,
      post_json,
      setup_seeded_app_with_admin_token,
      with_auth_token,
    },
    payloads::{dispatch_composite_save, dispatch_storage_measurement},
  },
  documents::seed_inventory_context,
};

const DISPATCH_NAMES_DOC_NUMBER: &str = "DISP-NAMES-1";

#[tokio::test]
async fn endpoints_return_related_names_with_embed_param() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let ctx = seed_inventory_context(&db).await;

  with_auth_token(token, async {
    seed_ledger_balance(
      &db,
      ctx.storage_a,
      ctx.product_id,
      ctx.contractor_id,
      Decimal::from(10),
    )
    .await;

    let create_composite = post_json(
      &app,
      api_paths::dispatch::COMPOSITE_SAVE,
      dispatch_composite_save(
        DISPATCH_NAMES_DOC_NUMBER,
        "2026-01-07T00:00:00Z",
        ctx.contractor_id,
        ctx.product_id,
        ctx.storage_a,
        "3.0",
      ),
    )
    .await;
    let composite_json = assert_api_success(create_composite).await;
    let dispatch_doc_id = Uuid::parse_str(composite_json["data"]["id"].as_str().unwrap()).unwrap();

    let measurement = post_json(
      &app,
      api_paths::dispatch::STORAGE_MEASUREMENTS,
      dispatch_storage_measurement(
        dispatch_doc_id,
        ctx.storage_a,
        "10.0",
        "10.0",
        "1.0",
        "10.0",
        "7.0",
        "7.0",
        "1.0",
        "7.0",
      ),
    )
    .await;
    let _ = assert_api_success(measurement).await;

    let list = get(&app, format!("{}?embed=names", api_paths::dispatch::ROOT)).await;
    let list_json = assert_api_success(list).await;
    let list_row = list_json["data"]
      .as_array()
      .unwrap()
      .iter()
      .find(|row| row["id"] == dispatch_doc_id.to_string())
      .unwrap();
    assert_eq!(list_row["contractorIdName"], "Contractor A");

    let query = get(
      &app,
      format!(
        "{}?documentNumber={}&embed=names",
        api_paths::dispatch::QUERY,
        DISPATCH_NAMES_DOC_NUMBER
      ),
    )
    .await;
    let query_json = assert_api_success(query).await;
    let query_row = &query_json["data"].as_array().unwrap()[0];
    assert_eq!(query_row["contractorIdName"], "Contractor A");

    let get_document = get(
      &app,
      format!(
        "{}?embed=names",
        api_paths::dispatch::BY_ID.replace("{id}", &dispatch_doc_id.to_string())
      ),
    )
    .await;
    let get_document_json = assert_api_success(get_document).await;
    assert_eq!(
      get_document_json["data"]["contractorIdName"],
      "Contractor A"
    );

    let get_composite = get(
      &app,
      format!(
        "{}?embed=names",
        api_paths::dispatch::COMPOSITE_BY_ID.replace("{id}", &dispatch_doc_id.to_string())
      ),
    )
    .await;
    let get_composite_json = assert_api_success(get_composite).await;
    assert_eq!(
      get_composite_json["data"]["contractorIdName"],
      "Contractor A"
    );
    assert_eq!(
      get_composite_json["data"]["items"][0]["productIdName"],
      "Product A"
    );
    assert_eq!(
      get_composite_json["data"]["items"][0]["storageIdName"],
      "Tank A"
    );
    assert_eq!(
      get_composite_json["data"]["storageMeasurements"][0]["storageIdName"],
      "Tank A"
    );
  })
  .await;
}
