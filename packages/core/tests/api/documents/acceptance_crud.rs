use sea_orm::{ColumnTrait, EntityLoaderTrait, QueryFilter};
use serde_json::json;
use uuid::Uuid;
use voletu_core::{endpoints::paths as api_paths, entities::inventory_ledger_entry};

use super::seed_inventory_context;
use crate::common::{
  http::{
    assert_api_success,
    get,
    post_empty,
    post_json,
    setup_seeded_app_with_admin_token,
    with_auth_token,
  },
  payloads::{
    acceptance_composite_save,
    acceptance_composite_save_and_execute,
    transport_truck_waybill,
  },
};

const ACCEPTANCE_DOC_NUMBER: &str = "ACC-EP-1";
const ACCEPTANCE_COMPOSITE_DOC_NUMBER: &str = "ACC-COMP-1";
const ACCEPTANCE_COMPOSITE_EXEC_DOC_NUMBER: &str = "ACC-COMP-2";
const ACCEPTANCE_NAMES_DOC_NUMBER: &str = "ACC-NAMES-1";
const ACCEPTANCE_NAMES_WAYBILL_NUMBER: &str = "TWB-ACC-NAMES-1";

#[tokio::test]
async fn acceptance_document_endpoints_create_item_execute_and_return_expected_payloads() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let ctx = seed_inventory_context(&db).await;

  with_auth_token(token, async {
    // Create acceptance document + item via composite endpoint
    let create_composite = post_json(
      &app,
      api_paths::acceptance::COMPOSITE_SAVE,
      acceptance_composite_save(
        ACCEPTANCE_DOC_NUMBER,
        "2026-01-01T00:00:00Z",
        ctx.contractor_id,
        ctx.product_id,
        ctx.storage_a,
        "5.0",
      ),
    )
    .await;
    let composite_json = assert_api_success(create_composite).await;
    assert_eq!(
      composite_json["data"]["documentNumber"],
      ACCEPTANCE_DOC_NUMBER
    );
    assert_eq!(composite_json["data"]["items"].as_array().unwrap().len(), 1);
    assert_eq!(composite_json["data"]["items"][0]["acceptedAmount"], "5");

    let doc_id = Uuid::parse_str(composite_json["data"]["id"].as_str().unwrap()).unwrap();

    // Execute
    let execute = post_empty(
      &app,
      api_paths::acceptance::EXECUTE_BY_ID.replace("{id}", &doc_id.to_string()),
    )
    .await;
    let execute_json = assert_api_success(execute).await;
    assert!(execute_json["data"].is_null());

    // Verify ledger entry
    let entry = inventory_ledger_entry::Entity::load()
      .filter(inventory_ledger_entry::Column::StorageId.eq(ctx.storage_a))
      .filter(inventory_ledger_entry::Column::ProductId.eq(ctx.product_id))
      .filter(inventory_ledger_entry::Column::ContractorId.eq(ctx.contractor_id))
      .one(&*db)
      .await
      .unwrap()
      .unwrap();
    assert_eq!(entry.current_amount.to_string(), "5");
  })
  .await;
}

#[tokio::test]
async fn acceptance_composite_endpoint_executes_by_default() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let ctx = seed_inventory_context(&db).await;

  with_auth_token(token, async {
    let omitted = post_json(
      &app,
      api_paths::acceptance::COMPOSITE_SAVE_AND_EXECUTE,
      acceptance_composite_save_and_execute(
        ACCEPTANCE_COMPOSITE_DOC_NUMBER,
        "2026-01-05T00:00:00Z",
        ctx.product_id,
        ctx.contractor_id,
        ctx.storage_a,
        "2.0",
      ),
    )
    .await;
    let omitted_json = assert_api_success(omitted).await;
    assert_eq!(
      omitted_json["data"]["documentNumber"],
      ACCEPTANCE_COMPOSITE_DOC_NUMBER
    );
    assert_eq!(omitted_json["data"]["items"].as_array().unwrap().len(), 1);
    assert_eq!(omitted_json["data"]["status"], "EXECUTED");

    let provided = post_json(
      &app,
      api_paths::acceptance::COMPOSITE_SAVE_AND_EXECUTE,
      acceptance_composite_save_and_execute(
        ACCEPTANCE_COMPOSITE_EXEC_DOC_NUMBER,
        "2026-01-05T01:00:00Z",
        ctx.product_id,
        ctx.contractor_id,
        ctx.storage_a,
        "5.0",
      ),
    )
    .await;
    let provided_json = assert_api_success(provided).await;
    assert_eq!(provided_json["data"]["status"], "EXECUTED");
    assert_eq!(provided_json["data"]["items"].as_array().unwrap().len(), 1);

    let ledger_entry = inventory_ledger_entry::Entity::load()
      .filter(inventory_ledger_entry::Column::StorageId.eq(ctx.storage_a))
      .filter(inventory_ledger_entry::Column::ProductId.eq(ctx.product_id))
      .filter(inventory_ledger_entry::Column::ContractorId.eq(ctx.contractor_id))
      .one(&*db)
      .await
      .unwrap()
      .unwrap();
    assert_eq!(ledger_entry.current_amount.to_string(), "7");
  })
  .await;
}

#[tokio::test]
async fn acceptance_endpoints_return_related_names_when_embed_names_is_requested() {
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
