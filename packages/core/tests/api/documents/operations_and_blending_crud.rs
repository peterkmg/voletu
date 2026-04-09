use axum::http::StatusCode;
use sea_orm::{prelude::Decimal, ColumnTrait, EntityLoaderTrait, QueryFilter};
use uuid::Uuid;
use voletu_core::{
  endpoints::paths as api_paths,
  entities::{inventory_ledger_entry, inventory_reconciliation},
};

use super::seed_inventory_context;
use crate::common::{
  catalog_seed::seed_ledger_balance,
  http::{
    assert_api_error,
    assert_api_success,
    get,
    post_empty,
    post_json,
    setup_seeded_app_with_admin_token,
    with_auth_token,
  },
  payloads::{
    blending_composite_save,
    blending_composite_save_and_execute,
    operations_ownership_transfer,
    operations_physical_transfer,
    operations_reconciliation_adjustment,
    operations_reconciliation_save,
  },
};

const PHYSICAL_TRANSFER_DOC_NUMBER: &str = "PT-EP-1";
const PHYSICAL_TRANSFER_NAMES_DOC_NUMBER: &str = "PT-NAMES-1";
const OWNERSHIP_TRANSFER_NAMES_DATE: &str = "2026-01-08T03:00:00Z";
const RECONCILIATION_DOC_NUMBER: &str = "REC-EP-1";
const RECONCILIATION_NAMES_DOC_NUMBER: &str = "REC-NAMES-1";
const BLENDING_DOC_NUMBER: &str = "BLD-EP-1";
const BLENDING_COMPOSITE_DOC_NUMBER: &str = "BLD-COMP-1";
const BLENDING_COMPOSITE_EXEC_DOC_NUMBER: &str = "BLD-COMP-2";
const BLENDING_NAMES_DOC_NUMBER: &str = "BLD-NAMES-1";

#[tokio::test]
async fn operations_endpoints_execute_core_workflows_and_report_invalid_blending_execution_as_bad_request_error(
) {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let ctx = seed_inventory_context(&db).await;

  with_auth_token(token, async {
    seed_ledger_balance(
      &db,
      ctx.storage_a,
      ctx.product_id,
      ctx.contractor_id,
      Decimal::from(20),
    )
    .await;

    let physical = post_json(
      &app,
      api_paths::operations::PHYSICAL_TRANSFERS_SAVE,
      operations_physical_transfer(
        PHYSICAL_TRANSFER_DOC_NUMBER,
        "2026-01-01T00:00:00Z",
        "2026-01-01T01:00:00Z",
        "2026-01-01T02:00:00Z",
        ctx.contractor_id,
        ctx.product_id,
        ctx.storage_a,
        ctx.storage_b,
        "5.0",
      ),
    )
    .await;
    let physical_json = assert_api_success(physical).await;
    assert_eq!(
      physical_json["data"]["documentNumber"],
      PHYSICAL_TRANSFER_DOC_NUMBER
    );
    assert_eq!(physical_json["data"]["items"][0]["amount"], "5");

    let ownership = post_json(
      &app,
      api_paths::operations::OWNERSHIP_TRANSFERS_SAVE,
      operations_ownership_transfer(
        "2026-01-01T03:00:00Z",
        ctx.storage_b,
        ctx.product_id,
        ctx.contractor_id,
        ctx.contractor_id,
        "1.0",
      ),
    )
    .await;
    let ownership_json = assert_api_success(ownership).await;
    assert_eq!(
      ownership_json["data"]["items"][0]["storageId"],
      ctx.storage_b.to_string()
    );
    assert_eq!(ownership_json["data"]["items"][0]["amount"], "1");

    let reconciliation = post_json(
      &app,
      api_paths::operations::RECONCILIATIONS_SAVE,
      operations_reconciliation_save(
        RECONCILIATION_DOC_NUMBER,
        "2026-01-02T00:00:00Z",
        ctx.contractor_id,
        ctx.warehouse_id,
      ),
    )
    .await;
    let reconciliation_json = assert_api_success(reconciliation).await;
    assert_eq!(
      reconciliation_json["data"]["documentNumber"],
      RECONCILIATION_DOC_NUMBER
    );

    let reconciliation_id =
      Uuid::parse_str(reconciliation_json["data"]["id"].as_str().unwrap()).unwrap();

    let adjustment = post_json(
      &app,
      api_paths::operations::RECONCILIATION_ADJUSTMENTS_SAVE,
      operations_reconciliation_adjustment(
        reconciliation_id,
        ctx.storage_b,
        ctx.product_id,
        "SURPLUS",
        "2.0",
        "count",
      ),
    )
    .await;
    let adjustment_json = assert_api_success(adjustment).await;
    assert_eq!(
      adjustment_json["data"]["reconciliationId"],
      reconciliation_id.to_string()
    );
    assert_eq!(adjustment_json["data"]["adjustmentType"], "SURPLUS");

    // Create blending document with unbalanced component/result via composite
    let blend_composite = post_json(
      &app,
      api_paths::blending::COMPOSITE_SAVE,
      blending_composite_save(
        BLENDING_DOC_NUMBER,
        "2026-01-03T00:00:00Z",
        ctx.contractor_id,
        ctx.second_product_id,
        ctx.storage_a,
        ctx.product_id,
        "4.0",
        ctx.storage_b,
        "1.0",
      ),
    )
    .await;
    let blend_json = assert_api_success(blend_composite).await;
    assert_eq!(
      blend_json["data"]["document"]["documentNumber"],
      BLENDING_DOC_NUMBER
    );

    let blend_doc_id =
      Uuid::parse_str(blend_json["data"]["document"]["id"].as_str().unwrap()).unwrap();

    let execute_unbalanced = post_empty(
      &app,
      api_paths::blending::EXECUTE_BY_ID.replace("{id}", &blend_doc_id.to_string()),
    )
    .await;
    let execute_unbalanced_json = assert_api_error(
      execute_unbalanced,
      StatusCode::BAD_REQUEST,
      "VALIDATION_ERROR",
      Some("components and results do not match"),
    )
    .await;
    assert_eq!(execute_unbalanced_json["error"]["code"], "VALIDATION_ERROR");
  })
  .await;
}

#[tokio::test]
async fn blending_composite_endpoint_executes_by_default() {
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
      api_paths::blending::COMPOSITE_SAVE_AND_EXECUTE,
      blending_composite_save_and_execute(
        BLENDING_COMPOSITE_DOC_NUMBER,
        "2026-01-07T00:00:00Z",
        ctx.contractor_id,
        ctx.second_product_id,
        ctx.storage_a,
        ctx.product_id,
        "1.0",
        ctx.storage_b,
        "1.0",
      ),
    )
    .await;
    let omitted_json = assert_api_success(omitted).await;
    assert_eq!(
      omitted_json["data"]["document"]["documentNumber"],
      BLENDING_COMPOSITE_DOC_NUMBER
    );
    assert_eq!(
      omitted_json["data"]["components"].as_array().unwrap().len(),
      1
    );
    assert_eq!(omitted_json["data"]["results"].as_array().unwrap().len(), 1);
    assert_eq!(omitted_json["data"]["document"]["status"], "EXECUTED");

    let provided = post_json(
      &app,
      api_paths::blending::COMPOSITE_SAVE_AND_EXECUTE,
      blending_composite_save_and_execute(
        BLENDING_COMPOSITE_EXEC_DOC_NUMBER,
        "2026-01-07T01:00:00Z",
        ctx.contractor_id,
        ctx.second_product_id,
        ctx.storage_a,
        ctx.product_id,
        "4.0",
        ctx.storage_b,
        "4.0",
      ),
    )
    .await;
    let provided_json = assert_api_success(provided).await;
    assert_eq!(provided_json["data"]["document"]["status"], "EXECUTED");
    assert_eq!(
      provided_json["data"]["components"]
        .as_array()
        .unwrap()
        .len(),
      1
    );
    assert_eq!(
      provided_json["data"]["results"].as_array().unwrap().len(),
      1
    );

    let source_entry = inventory_ledger_entry::Entity::load()
      .filter(inventory_ledger_entry::Column::StorageId.eq(ctx.storage_a))
      .filter(inventory_ledger_entry::Column::ProductId.eq(ctx.product_id))
      .filter(inventory_ledger_entry::Column::ContractorId.eq(ctx.contractor_id))
      .one(&*db)
      .await
      .unwrap()
      .unwrap();
    assert_eq!(source_entry.current_amount.to_string(), "5");

    let result_entry = inventory_ledger_entry::Entity::load()
      .filter(inventory_ledger_entry::Column::StorageId.eq(ctx.storage_b))
      .filter(inventory_ledger_entry::Column::ProductId.eq(ctx.second_product_id))
      .filter(inventory_ledger_entry::Column::ContractorId.eq(ctx.contractor_id))
      .one(&*db)
      .await
      .unwrap()
      .unwrap();
    assert_eq!(result_entry.current_amount.to_string(), "5");
  })
  .await;
}

#[tokio::test]
async fn blending_endpoints_return_related_names_when_embed_names_is_requested() {
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

    let create = post_json(
      &app,
      api_paths::blending::COMPOSITE_SAVE,
      blending_composite_save(
        BLENDING_NAMES_DOC_NUMBER,
        "2026-01-08T00:00:00Z",
        ctx.contractor_id,
        ctx.second_product_id,
        ctx.storage_a,
        ctx.product_id,
        "3.0",
        ctx.storage_b,
        "3.0",
      ),
    )
    .await;
    let create_json = assert_api_success(create).await;
    let blending_id =
      Uuid::parse_str(create_json["data"]["document"]["id"].as_str().unwrap()).unwrap();

    let list = get(&app, format!("{}?embed=names", api_paths::blending::ROOT)).await;
    let list_json = assert_api_success(list).await;
    let list_row = list_json["data"]
      .as_array()
      .unwrap()
      .iter()
      .find(|row| row["id"] == blending_id.to_string())
      .unwrap();
    assert_eq!(list_row["contractorIdName"], "Contractor A");
    assert_eq!(list_row["targetProductIdName"], "Product B");

    let query = get(
      &app,
      format!(
        "{}?documentNumber={}&embed=names",
        api_paths::blending::QUERY,
        BLENDING_NAMES_DOC_NUMBER
      ),
    )
    .await;
    let query_json = assert_api_success(query).await;
    let query_row = &query_json["data"].as_array().unwrap()[0];
    assert_eq!(query_row["contractorIdName"], "Contractor A");
    assert_eq!(query_row["targetProductIdName"], "Product B");

    let get_document = get(
      &app,
      format!(
        "{}?embed=names",
        api_paths::blending::BY_ID.replace("{id}", &blending_id.to_string())
      ),
    )
    .await;
    let get_document_json = assert_api_success(get_document).await;
    assert_eq!(
      get_document_json["data"]["contractorIdName"],
      "Contractor A"
    );
    assert_eq!(
      get_document_json["data"]["targetProductIdName"],
      "Product B"
    );

    let get_composite = get(
      &app,
      format!(
        "{}?embed=names",
        api_paths::blending::COMPOSITE_BY_ID.replace("{id}", &blending_id.to_string())
      ),
    )
    .await;
    let get_composite_json = assert_api_success(get_composite).await;
    assert_eq!(
      get_composite_json["data"]["document"]["contractorIdName"],
      "Contractor A"
    );
    assert_eq!(
      get_composite_json["data"]["document"]["targetProductIdName"],
      "Product B"
    );
    assert_eq!(
      get_composite_json["data"]["components"][0]["sourceProductIdName"],
      "Product A"
    );
    assert_eq!(
      get_composite_json["data"]["components"][0]["storageIdName"],
      "Tank A"
    );
    assert_eq!(
      get_composite_json["data"]["results"][0]["storageIdName"],
      "Tank B"
    );
  })
  .await;
}

#[tokio::test]
async fn physical_and_ownership_endpoints_return_related_names_when_embed_names_is_requested() {
  let (_db, app, token) = setup_seeded_app_with_admin_token().await;
  let ctx = seed_inventory_context(&_db).await;

  with_auth_token(token, async {
    let physical = post_json(
      &app,
      api_paths::operations::PHYSICAL_TRANSFERS_SAVE,
      operations_physical_transfer(
        PHYSICAL_TRANSFER_NAMES_DOC_NUMBER,
        "2026-01-08T00:00:00Z",
        "2026-01-08T01:00:00Z",
        "2026-01-08T02:00:00Z",
        ctx.contractor_id,
        ctx.product_id,
        ctx.storage_a,
        ctx.storage_b,
        "7.0",
      ),
    )
    .await;
    let physical_json = assert_api_success(physical).await;
    let physical_id = Uuid::parse_str(physical_json["data"]["id"].as_str().unwrap()).unwrap();

    let ownership = post_json(
      &app,
      api_paths::operations::OWNERSHIP_TRANSFERS_SAVE,
      operations_ownership_transfer(
        OWNERSHIP_TRANSFER_NAMES_DATE,
        ctx.storage_b,
        ctx.product_id,
        ctx.contractor_id,
        ctx.contractor_id,
        "2.0",
      ),
    )
    .await;
    let ownership_json = assert_api_success(ownership).await;
    let ownership_id = Uuid::parse_str(ownership_json["data"]["id"].as_str().unwrap()).unwrap();

    let physical_list = get(
      &app,
      format!("{}?embed=names", api_paths::operations::PHYSICAL_TRANSFERS),
    )
    .await;
    let physical_list_json = assert_api_success(physical_list).await;
    let physical_list_row = physical_list_json["data"]
      .as_array()
      .unwrap()
      .iter()
      .find(|row| row["id"] == physical_id.to_string())
      .unwrap();
    assert_eq!(physical_list_row["contractorIdName"], "Contractor A");
    assert_eq!(physical_list_row["items"][0]["productIdName"], "Product A");
    assert_eq!(physical_list_row["items"][0]["fromStorageIdName"], "Tank A");
    assert_eq!(physical_list_row["items"][0]["toStorageIdName"], "Tank B");

    let physical_query = get(
      &app,
      format!(
        "{}?documentNumber={}&embed=names",
        api_paths::operations::PHYSICAL_TRANSFERS_QUERY,
        PHYSICAL_TRANSFER_NAMES_DOC_NUMBER
      ),
    )
    .await;
    let physical_query_json = assert_api_success(physical_query).await;
    let physical_query_row = &physical_query_json["data"].as_array().unwrap()[0];
    assert_eq!(physical_query_row["contractorIdName"], "Contractor A");
    assert_eq!(physical_query_row["items"][0]["productIdName"], "Product A");
    assert_eq!(
      physical_query_row["items"][0]["fromStorageIdName"],
      "Tank A"
    );
    assert_eq!(physical_query_row["items"][0]["toStorageIdName"], "Tank B");

    let physical_get = get(
      &app,
      format!(
        "{}?embed=names",
        api_paths::operations::PHYSICAL_TRANSFER_DOCUMENTS_BY_ID
          .replace("{id}", &physical_id.to_string())
      ),
    )
    .await;
    let physical_get_json = assert_api_success(physical_get).await;
    assert_eq!(
      physical_get_json["data"]["contractorIdName"],
      "Contractor A"
    );
    assert_eq!(
      physical_get_json["data"]["items"][0]["productIdName"],
      "Product A"
    );
    assert_eq!(
      physical_get_json["data"]["items"][0]["fromStorageIdName"],
      "Tank A"
    );
    assert_eq!(
      physical_get_json["data"]["items"][0]["toStorageIdName"],
      "Tank B"
    );

    let physical_composite = get(
      &app,
      format!(
        "{}?embed=names",
        api_paths::operations::PHYSICAL_TRANSFERS_COMPOSITE_BY_ID
          .replace("{id}", &physical_id.to_string())
      ),
    )
    .await;
    let physical_composite_json = assert_api_success(physical_composite).await;
    assert_eq!(
      physical_composite_json["data"]["contractorIdName"],
      "Contractor A"
    );
    assert_eq!(
      physical_composite_json["data"]["items"][0]["productIdName"],
      "Product A"
    );
    assert_eq!(
      physical_composite_json["data"]["items"][0]["fromStorageIdName"],
      "Tank A"
    );
    assert_eq!(
      physical_composite_json["data"]["items"][0]["toStorageIdName"],
      "Tank B"
    );

    let ownership_list = get(
      &app,
      format!("{}?embed=names", api_paths::operations::OWNERSHIP_TRANSFERS),
    )
    .await;
    let ownership_list_json = assert_api_success(ownership_list).await;
    let ownership_list_row = ownership_list_json["data"]
      .as_array()
      .unwrap()
      .iter()
      .find(|row| row["id"] == ownership_id.to_string())
      .unwrap();
    assert_eq!(ownership_list_row["items"][0]["storageIdName"], "Tank B");
    assert_eq!(ownership_list_row["items"][0]["productIdName"], "Product A");
    assert_eq!(
      ownership_list_row["items"][0]["fromContractorIdName"],
      "Contractor A"
    );
    assert_eq!(
      ownership_list_row["items"][0]["toContractorIdName"],
      "Contractor A"
    );

    let ownership_query = get(
      &app,
      format!(
        "{}?status=DRAFT&embed=names",
        api_paths::operations::OWNERSHIP_TRANSFERS_QUERY
      ),
    )
    .await;
    let ownership_query_json = assert_api_success(ownership_query).await;
    let ownership_query_row = ownership_query_json["data"]
      .as_array()
      .unwrap()
      .iter()
      .find(|row| row["id"] == ownership_id.to_string())
      .unwrap();
    assert_eq!(ownership_query_row["items"][0]["storageIdName"], "Tank B");
    assert_eq!(
      ownership_query_row["items"][0]["productIdName"],
      "Product A"
    );
    assert_eq!(
      ownership_query_row["items"][0]["fromContractorIdName"],
      "Contractor A"
    );
    assert_eq!(
      ownership_query_row["items"][0]["toContractorIdName"],
      "Contractor A"
    );

    let ownership_get = get(
      &app,
      format!(
        "{}?embed=names",
        api_paths::operations::OWNERSHIP_TRANSFER_DOCUMENTS_BY_ID
          .replace("{id}", &ownership_id.to_string())
      ),
    )
    .await;
    let ownership_get_json = assert_api_success(ownership_get).await;
    assert_eq!(
      ownership_get_json["data"]["items"][0]["storageIdName"],
      "Tank B"
    );
    assert_eq!(
      ownership_get_json["data"]["items"][0]["productIdName"],
      "Product A"
    );
    assert_eq!(
      ownership_get_json["data"]["items"][0]["fromContractorIdName"],
      "Contractor A"
    );
    assert_eq!(
      ownership_get_json["data"]["items"][0]["toContractorIdName"],
      "Contractor A"
    );

    let ownership_composite = get(
      &app,
      format!(
        "{}?embed=names",
        api_paths::operations::OWNERSHIP_TRANSFERS_COMPOSITE_BY_ID
          .replace("{id}", &ownership_id.to_string())
      ),
    )
    .await;
    let ownership_composite_json = assert_api_success(ownership_composite).await;
    assert_eq!(
      ownership_composite_json["data"]["items"][0]["storageIdName"],
      "Tank B"
    );
    assert_eq!(
      ownership_composite_json["data"]["items"][0]["productIdName"],
      "Product A"
    );
    assert_eq!(
      ownership_composite_json["data"]["items"][0]["fromContractorIdName"],
      "Contractor A"
    );
    assert_eq!(
      ownership_composite_json["data"]["items"][0]["toContractorIdName"],
      "Contractor A"
    );
  })
  .await;
}

#[tokio::test]
async fn reconciliation_endpoints_return_related_names_when_embed_names_is_requested() {
  let (_db, app, token) = setup_seeded_app_with_admin_token().await;
  let ctx = seed_inventory_context(&_db).await;

  with_auth_token(token, async {
    let create = post_json(
      &app,
      api_paths::operations::RECONCILIATIONS_SAVE,
      operations_reconciliation_save(
        RECONCILIATION_NAMES_DOC_NUMBER,
        "2026-01-09T00:00:00Z",
        ctx.contractor_id,
        ctx.warehouse_id,
      ),
    )
    .await;
    let create_json = assert_api_success(create).await;
    let reconciliation_id = Uuid::parse_str(create_json["data"]["id"].as_str().unwrap()).unwrap();

    let list = get(
      &app,
      format!("{}?embed=names", api_paths::operations::RECONCILIATIONS),
    )
    .await;
    let list_json = assert_api_success(list).await;
    let list_row = list_json["data"]
      .as_array()
      .unwrap()
      .iter()
      .find(|row| row["id"] == reconciliation_id.to_string())
      .unwrap();
    assert_eq!(list_row["contractorIdName"], "Contractor A");
    assert_eq!(list_row["warehouseIdName"], "WH 1");

    let query = get(
      &app,
      format!(
        "{}?documentNumber={}&warehouseId={}&embed=names",
        api_paths::operations::RECONCILIATIONS_QUERY,
        RECONCILIATION_NAMES_DOC_NUMBER,
        ctx.warehouse_id
      ),
    )
    .await;
    let query_json = assert_api_success(query).await;
    let query_row = &query_json["data"].as_array().unwrap()[0];
    assert_eq!(query_row["contractorIdName"], "Contractor A");
    assert_eq!(query_row["warehouseIdName"], "WH 1");

    let get_by_id = get(
      &app,
      format!(
        "{}?embed=names",
        api_paths::operations::RECONCILIATIONS_BY_ID
          .replace("{id}", &reconciliation_id.to_string())
      ),
    )
    .await;
    let get_json = assert_api_success(get_by_id).await;
    assert_eq!(get_json["data"]["contractorIdName"], "Contractor A");
    assert_eq!(get_json["data"]["warehouseIdName"], "WH 1");
  })
  .await;
}
