use axum::http::StatusCode;
use sea_orm::{prelude::Decimal, ColumnTrait, EntityLoaderTrait, QueryFilter};
use uuid::Uuid;
use voletu_core::{endpoints::paths as api_paths, entities::inventory_ledger_entry};

use super::seed_inventory_context;
use crate::common::{
  catalog_seed::seed_ledger_balance,
  http::{
    assert_api_success,
    get,
    post_empty,
    post_json,
    response_json,
    setup_seeded_app_with_admin_token,
    with_auth_token,
  },
  payloads::{
    dispatch_composite_save,
    dispatch_composite_save_and_execute,
    dispatch_composite_save_and_execute_with_measurement,
    dispatch_storage_measurement,
  },
};

const DISPATCH_DOC_NUMBER: &str = "DISP-EP-1";
const DISPATCH_COMPOSITE_DOC_NUMBER: &str = "DISP-COMP-1";
const DISPATCH_COMPOSITE_EXEC_DOC_NUMBER: &str = "DISP-COMP-2";
const DISPATCH_NAMES_DOC_NUMBER: &str = "DISP-NAMES-1";

#[tokio::test]
async fn dispatch_endpoints_create_measure_and_execute_successfully() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let ctx = seed_inventory_context(&db).await;

  with_auth_token(token, async {
    // Seed ledger balance first (dispatch validation checks balance)
    seed_ledger_balance(
      &db,
      ctx.storage_a,
      ctx.product_id,
      ctx.contractor_id,
      Decimal::from(10),
    )
    .await;

    // Create dispatch document + item via composite endpoint
    let create_composite = post_json(
      &app,
      api_paths::dispatch::COMPOSITE_SAVE,
      dispatch_composite_save(
        DISPATCH_DOC_NUMBER,
        "2026-01-01T00:00:00Z",
        ctx.contractor_id,
        ctx.product_id,
        ctx.storage_a,
        "3.0",
      ),
    )
    .await;
    let composite_json = assert_api_success(create_composite).await;
    assert_eq!(
      composite_json["data"]["documentNumber"],
      DISPATCH_DOC_NUMBER
    );
    assert_eq!(composite_json["data"]["items"].as_array().unwrap().len(), 1);

    let dispatch_doc_id = Uuid::parse_str(composite_json["data"]["id"].as_str().unwrap()).unwrap();

    // Add measurement (kept as standalone — linked to doc)
    let measure = post_json(
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
    let measure_json = assert_api_success(measure).await;
    assert_eq!(measure_json["data"]["afterMass"], "7");

    // Execute
    let execute = post_empty(
      &app,
      api_paths::dispatch::EXECUTE_BY_ID.replace("{id}", &dispatch_doc_id.to_string()),
    )
    .await;
    let execute_json = assert_api_success(execute).await;
    assert!(execute_json["data"].is_null());
  })
  .await;
}

#[tokio::test]
async fn dispatch_composite_endpoint_executes_by_default() {
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

    let omitted = post_json(
      &app,
      api_paths::dispatch::COMPOSITE_SAVE_AND_EXECUTE,
      dispatch_composite_save_and_execute(
        DISPATCH_COMPOSITE_DOC_NUMBER,
        "2026-01-06T00:00:00Z",
        ctx.contractor_id,
        ctx.product_id,
        ctx.storage_a,
        "1.0",
      ),
    )
    .await;
    let omitted_status = omitted.status();
    let omitted_body = response_json(omitted).await;
    assert_eq!(omitted_status, StatusCode::OK, "{}", omitted_body);
    let omitted_json = omitted_body;
    assert_eq!(
      omitted_json["data"]["documentNumber"],
      DISPATCH_COMPOSITE_DOC_NUMBER
    );
    assert_eq!(omitted_json["data"]["items"].as_array().unwrap().len(), 1);
    assert_eq!(
      omitted_json["data"]["storageMeasurements"]
        .as_array()
        .unwrap()
        .len(),
      0
    );
    assert_eq!(omitted_json["data"]["status"], "EXECUTED");

    let provided = post_json(
      &app,
      api_paths::dispatch::COMPOSITE_SAVE_AND_EXECUTE,
      dispatch_composite_save_and_execute_with_measurement(
        DISPATCH_COMPOSITE_EXEC_DOC_NUMBER,
        "2026-01-06T01:00:00Z",
        ctx.contractor_id,
        ctx.product_id,
        ctx.storage_a,
        "3.0",
        "10.0",
        "7.0",
      ),
    )
    .await;
    let provided_status = provided.status();
    let provided_body = response_json(provided).await;
    assert_eq!(provided_status, StatusCode::OK, "{}", provided_body);
    let provided_json = provided_body;
    assert_eq!(provided_json["data"]["status"], "EXECUTED");
    assert_eq!(provided_json["data"]["items"].as_array().unwrap().len(), 1);
    assert_eq!(
      provided_json["data"]["storageMeasurements"]
        .as_array()
        .unwrap()
        .len(),
      1
    );

    let ledger_entry = inventory_ledger_entry::Entity::load()
      .filter(inventory_ledger_entry::Column::StorageId.eq(ctx.storage_a))
      .filter(inventory_ledger_entry::Column::ProductId.eq(ctx.product_id))
      .filter(inventory_ledger_entry::Column::ContractorId.eq(ctx.contractor_id))
      .one(&*db)
      .await
      .unwrap()
      .unwrap();
    assert_eq!(ledger_entry.current_amount.to_string(), "6");
  })
  .await;
}

#[tokio::test]
async fn dispatch_endpoints_return_related_names_when_embed_names_is_requested() {
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
