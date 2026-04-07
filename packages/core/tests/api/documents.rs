use std::sync::Arc;

use axum::http::StatusCode;
use sea_orm::{prelude::Decimal, ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;
use voletu_core::{
  endpoints::paths as api_paths,
  entities::{
    acceptance_document,
    blending_document,
    dispatch_document,
    inventory_ledger_entry,
    inventory_reconciliation,
    ownership_transfer,
  },
};

use crate::common::{
  fixtures::{seed_inventory_fixture, seed_ledger_balance},
  http::{
    assert_api_error,
    assert_api_success,
    get,
    post_empty,
    post_json,
    response_json,
    setup_seeded_app_with_admin_token,
    with_auth_token,
  },
  payloads::{
    acceptance_composite_save,
    acceptance_composite_save_and_execute,
    acceptance_save_truck,
    blending_composite_save,
    blending_composite_save_and_execute,
    blending_save,
    dispatch_composite_save,
    dispatch_composite_save_and_execute,
    dispatch_composite_save_and_execute_with_measurement,
    dispatch_save_external_truck,
    dispatch_storage_measurement,
    operations_ownership_transfer,
    operations_physical_transfer,
    operations_reconciliation_adjustment,
    operations_reconciliation_save,
  },
};

const ACCEPTANCE_DOC_NUMBER: &str = "ACC-EP-1";
const DISPATCH_DOC_NUMBER: &str = "DISP-EP-1";
const PHYSICAL_TRANSFER_DOC_NUMBER: &str = "PT-EP-1";
const RECONCILIATION_DOC_NUMBER: &str = "REC-EP-1";
const BLENDING_DOC_NUMBER: &str = "BLD-EP-1";
const ACCEPTANCE_COMPOSITE_DOC_NUMBER: &str = "ACC-COMP-1";
const ACCEPTANCE_COMPOSITE_EXEC_DOC_NUMBER: &str = "ACC-COMP-2";
const DISPATCH_COMPOSITE_DOC_NUMBER: &str = "DISP-COMP-1";
const DISPATCH_COMPOSITE_EXEC_DOC_NUMBER: &str = "DISP-COMP-2";
const BLENDING_COMPOSITE_DOC_NUMBER: &str = "BLD-COMP-1";
const BLENDING_COMPOSITE_EXEC_DOC_NUMBER: &str = "BLD-COMP-2";
const ACCEPTANCE_QUERY_DOC_NUMBER_1: &str = "ACC-Q-1";
const ACCEPTANCE_QUERY_DOC_NUMBER_2: &str = "ACC-Q-2";
const PHYSICAL_QUERY_DOC_NUMBER_1: &str = "PT-Q-1";
const PHYSICAL_QUERY_DOC_NUMBER_2: &str = "PT-Q-2";
const DISPATCH_QUERY_DOC_NUMBER_1: &str = "DISP-Q-1";
const DISPATCH_QUERY_DOC_NUMBER_2: &str = "DISP-Q-2";
const BLENDING_QUERY_DOC_NUMBER_1: &str = "BLD-Q-1";
const BLENDING_QUERY_DOC_NUMBER_2: &str = "BLD-Q-2";
const BLENDING_STATUS_QUERY_DOC_NUMBER_1: &str = "BLD-SQ-1";
const BLENDING_STATUS_QUERY_DOC_NUMBER_2: &str = "BLD-SQ-2";
const RECONCILIATION_QUERY_DOC_NUMBER_1: &str = "REC-Q-1";
const RECONCILIATION_QUERY_DOC_NUMBER_2: &str = "REC-Q-2";
const ACCEPTANCE_PAGINATION_DOC_NUMBER_1: &str = "ACC-PAG-1";
const ACCEPTANCE_PAGINATION_DOC_NUMBER_2: &str = "ACC-PAG-2";
const ACCEPTANCE_PAGINATION_DOC_NUMBER_3: &str = "ACC-PAG-3";
const DISPATCH_PAGINATION_DOC_NUMBER_1: &str = "DISP-PAG-1";
const DISPATCH_PAGINATION_DOC_NUMBER_2: &str = "DISP-PAG-2";
const DISPATCH_PAGINATION_DOC_NUMBER_3: &str = "DISP-PAG-3";
const RECONCILIATION_PAGINATION_DOC_NUMBER_1: &str = "REC-PAG-1";
const RECONCILIATION_PAGINATION_DOC_NUMBER_2: &str = "REC-PAG-2";
const RECONCILIATION_PAGINATION_DOC_NUMBER_3: &str = "REC-PAG-3";
const OWNERSHIP_PAGINATION_DATE_1: &str = "2026-01-13T09:00:00Z";
const OWNERSHIP_PAGINATION_DATE_2: &str = "2026-01-13T10:00:00Z";
const OWNERSHIP_PAGINATION_DATE_3: &str = "2026-01-13T11:00:00Z";
const PHYSICAL_PAGINATION_DOC_NUMBER_1: &str = "PT-PAG-1";
const PHYSICAL_PAGINATION_DOC_NUMBER_2: &str = "PT-PAG-2";
const PHYSICAL_PAGINATION_DOC_NUMBER_3: &str = "PT-PAG-3";
const BLENDING_PAGINATION_DOC_NUMBER_1: &str = "BLD-PAG-1";
const BLENDING_PAGINATION_DOC_NUMBER_2: &str = "BLD-PAG-2";
const BLENDING_PAGINATION_DOC_NUMBER_3: &str = "BLD-PAG-3";

struct InventoryContext {
  contractor_id: Uuid,
  product_id: Uuid,
  second_product_id: Uuid,
  storage_a: Uuid,
  storage_b: Uuid,
  warehouse_id: Uuid,
}

async fn seed_inventory_context(db: &Arc<sea_orm::DatabaseConnection>) -> InventoryContext {
  let fixture = seed_inventory_fixture(db).await;
  InventoryContext {
    contractor_id: fixture.contractor_a_id,
    product_id: fixture.product_a_id,
    second_product_id: fixture.product_b_id,
    storage_a: fixture.storage_a_id,
    storage_b: fixture.storage_b_id,
    warehouse_id: fixture.warehouse_id,
  }
}

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
    let entry = inventory_ledger_entry::Entity::find()
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

    let reconciliation_id = inventory_reconciliation::Entity::find()
      .filter(inventory_reconciliation::Column::DocumentNumber.eq(RECONCILIATION_DOC_NUMBER))
      .one(&*db)
      .await
      .unwrap()
      .unwrap()
      .id;

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

    let ledger_entry = inventory_ledger_entry::Entity::find()
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

    let ledger_entry = inventory_ledger_entry::Entity::find()
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

    let source_entry = inventory_ledger_entry::Entity::find()
      .filter(inventory_ledger_entry::Column::StorageId.eq(ctx.storage_a))
      .filter(inventory_ledger_entry::Column::ProductId.eq(ctx.product_id))
      .filter(inventory_ledger_entry::Column::ContractorId.eq(ctx.contractor_id))
      .one(&*db)
      .await
      .unwrap()
      .unwrap();
    assert_eq!(source_entry.current_amount.to_string(), "5");

    let result_entry = inventory_ledger_entry::Entity::find()
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
async fn query_endpoints_filter_by_document_number_and_status() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let ctx = seed_inventory_context(&db).await;

  with_auth_token(token, async {
    let acceptance_1 = post_json(
      &app,
      api_paths::acceptance::SAVE,
      acceptance_save_truck(
        ACCEPTANCE_QUERY_DOC_NUMBER_1,
        "2026-01-08T00:00:00Z",
        ctx.contractor_id,
      ),
    )
    .await;
    let _ = assert_api_success(acceptance_1).await;

    let acceptance_2 = post_json(
      &app,
      api_paths::acceptance::COMPOSITE_SAVE,
      acceptance_composite_save(
        ACCEPTANCE_QUERY_DOC_NUMBER_2,
        "2026-01-08T01:00:00Z",
        ctx.contractor_id,
        ctx.product_id,
        ctx.storage_a,
        "3.0",
      ),
    )
    .await;
    let acceptance_2_json = assert_api_success(acceptance_2).await;
    let acceptance_2_id =
      Uuid::parse_str(acceptance_2_json["data"]["id"].as_str().unwrap()).unwrap();

    let execute_acceptance_2 = post_empty(
      &app,
      api_paths::acceptance::EXECUTE_BY_ID.replace("{id}", &acceptance_2_id.to_string()),
    )
    .await;
    let _ = assert_api_success(execute_acceptance_2).await;

    let acceptance_query_doc = get(
      &app,
      format!(
        "{}?documentNumber={}",
        api_paths::acceptance::QUERY,
        ACCEPTANCE_QUERY_DOC_NUMBER_1
      ),
    )
    .await;
    let acceptance_query_doc_json = assert_api_success(acceptance_query_doc).await;
    let acceptance_doc_rows = acceptance_query_doc_json["data"]
      .as_array()
      .expect("acceptance query data must be array");
    assert_eq!(acceptance_doc_rows.len(), 1);
    assert_eq!(
      acceptance_doc_rows[0]["documentNumber"],
      ACCEPTANCE_QUERY_DOC_NUMBER_1
    );

    let acceptance_query_status = get(
      &app,
      format!("{}?status=EXECUTED", api_paths::acceptance::QUERY),
    )
    .await;
    let acceptance_query_status_json = assert_api_success(acceptance_query_status).await;
    let acceptance_status_rows = acceptance_query_status_json["data"]
      .as_array()
      .expect("acceptance status query data must be array");
    assert_eq!(acceptance_status_rows.len(), 1);
    assert_eq!(
      acceptance_status_rows[0]["documentNumber"],
      ACCEPTANCE_QUERY_DOC_NUMBER_2
    );

    let physical_1 = post_json(
      &app,
      api_paths::operations::PHYSICAL_TRANSFERS_SAVE,
      operations_physical_transfer(
        PHYSICAL_QUERY_DOC_NUMBER_1,
        "2026-01-09T00:00:00Z",
        "2026-01-09T01:00:00Z",
        "2026-01-09T02:00:00Z",
        ctx.contractor_id,
        ctx.product_id,
        ctx.storage_a,
        ctx.storage_b,
        "1.0",
      ),
    )
    .await;
    let _ = assert_api_success(physical_1).await;

    let physical_2 = post_json(
      &app,
      api_paths::operations::PHYSICAL_TRANSFERS_SAVE,
      operations_physical_transfer(
        PHYSICAL_QUERY_DOC_NUMBER_2,
        "2026-01-09T03:00:00Z",
        "2026-01-09T04:00:00Z",
        "2026-01-09T05:00:00Z",
        ctx.contractor_id,
        ctx.product_id,
        ctx.storage_a,
        ctx.storage_b,
        "2.0",
      ),
    )
    .await;
    let _ = assert_api_success(physical_2).await;

    let physical_query_doc = get(
      &app,
      format!(
        "{}?documentNumber={}",
        api_paths::operations::PHYSICAL_TRANSFERS_QUERY,
        PHYSICAL_QUERY_DOC_NUMBER_1
      ),
    )
    .await;
    let physical_query_doc_json = assert_api_success(physical_query_doc).await;
    let physical_doc_rows = physical_query_doc_json["data"]
      .as_array()
      .expect("physical query data must be array");
    assert_eq!(physical_doc_rows.len(), 1);
    assert_eq!(
      physical_doc_rows[0]["documentNumber"],
      PHYSICAL_QUERY_DOC_NUMBER_1
    );

    let physical_query_status = get(
      &app,
      format!(
        "{}?status=DRAFT",
        api_paths::operations::PHYSICAL_TRANSFERS_QUERY
      ),
    )
    .await;
    let physical_query_status_json = assert_api_success(physical_query_status).await;
    let physical_status_rows = physical_query_status_json["data"]
      .as_array()
      .expect("physical status query data must be array");
    assert_eq!(physical_status_rows.len(), 2);

    let dispatch_1 = post_json(
      &app,
      api_paths::dispatch::SAVE,
      dispatch_save_external_truck(
        DISPATCH_QUERY_DOC_NUMBER_1,
        "2026-01-09T06:00:00Z",
        ctx.contractor_id,
      ),
    )
    .await;
    let _ = assert_api_success(dispatch_1).await;

    let dispatch_2 = post_json(
      &app,
      api_paths::dispatch::SAVE,
      dispatch_save_external_truck(
        DISPATCH_QUERY_DOC_NUMBER_2,
        "2026-01-09T07:00:00Z",
        ctx.contractor_id,
      ),
    )
    .await;
    let _ = assert_api_success(dispatch_2).await;

    let dispatch_query_doc = get(
      &app,
      format!(
        "{}?documentNumber={}",
        api_paths::dispatch::QUERY,
        DISPATCH_QUERY_DOC_NUMBER_1
      ),
    )
    .await;
    let dispatch_query_doc_json = assert_api_success(dispatch_query_doc).await;
    let dispatch_doc_rows = dispatch_query_doc_json["data"]
      .as_array()
      .expect("dispatch query data must be array");
    assert_eq!(dispatch_doc_rows.len(), 1);
    assert_eq!(
      dispatch_doc_rows[0]["documentNumber"],
      DISPATCH_QUERY_DOC_NUMBER_1
    );

    let dispatch_query_no_match = get(
      &app,
      format!(
        "{}?documentNumber=DISP-Q-NOMATCH",
        api_paths::dispatch::QUERY
      ),
    )
    .await;
    let dispatch_query_no_match_json = assert_api_success(dispatch_query_no_match).await;
    assert_eq!(
      dispatch_query_no_match_json["data"]
        .as_array()
        .expect("dispatch no-match query data must be array")
        .len(),
      0
    );

    let blending_draft = post_json(
      &app,
      api_paths::blending::SAVE,
      blending_save(
        BLENDING_QUERY_DOC_NUMBER_1,
        "2026-01-10T00:00:00Z",
        ctx.contractor_id,
        ctx.second_product_id,
      ),
    )
    .await;
    let _ = assert_api_success(blending_draft).await;

    let blending_posted = post_json(
      &app,
      api_paths::blending::SAVE,
      blending_save(
        BLENDING_QUERY_DOC_NUMBER_2,
        "2026-01-10T01:00:00Z",
        ctx.contractor_id,
        ctx.second_product_id,
      ),
    )
    .await;
    let _ = assert_api_success(blending_posted).await;

    let blending_query_doc = get(
      &app,
      format!(
        "{}?documentNumber={}",
        api_paths::blending::QUERY,
        BLENDING_QUERY_DOC_NUMBER_1
      ),
    )
    .await;
    let blending_query_doc_json = assert_api_success(blending_query_doc).await;
    let blending_doc_rows = blending_query_doc_json["data"]
      .as_array()
      .expect("blending document query data must be array");
    assert_eq!(blending_doc_rows.len(), 1);
    assert_eq!(
      blending_doc_rows[0]["documentNumber"],
      BLENDING_QUERY_DOC_NUMBER_1
    );

    let blending_query_no_match = get(
      &app,
      format!(
        "{}?documentNumber=BLD-Q-NOMATCH",
        api_paths::blending::QUERY
      ),
    )
    .await;
    let blending_query_no_match_json = assert_api_success(blending_query_no_match).await;
    assert_eq!(
      blending_query_no_match_json["data"]
        .as_array()
        .expect("blending no-match query data must be array")
        .len(),
      0
    );

    seed_ledger_balance(
      &db,
      ctx.storage_a,
      ctx.product_id,
      ctx.contractor_id,
      Decimal::from(100),
    )
    .await;

    let ownership_draft = post_json(
      &app,
      api_paths::operations::OWNERSHIP_TRANSFERS_SAVE,
      operations_ownership_transfer(
        "2026-01-10T00:00:00Z",
        ctx.storage_a,
        ctx.product_id,
        ctx.contractor_id,
        ctx.contractor_id,
        "1.0",
      ),
    )
    .await;
    let ownership_draft_json = assert_api_success(ownership_draft).await;
    let ownership_draft_id = ownership_draft_json["data"]["id"]
      .as_str()
      .expect("ownership draft id must exist")
      .to_string();

    let ownership_draft_2 = post_json(
      &app,
      api_paths::operations::OWNERSHIP_TRANSFERS_SAVE,
      operations_ownership_transfer(
        "2026-01-10T01:00:00Z",
        ctx.storage_a,
        ctx.product_id,
        ctx.contractor_id,
        ctx.contractor_id,
        "2.0",
      ),
    )
    .await;
    let _ = assert_api_success(ownership_draft_2).await;

    let ownership_query_draft = get(
      &app,
      format!(
        "{}?status=DRAFT",
        api_paths::operations::OWNERSHIP_TRANSFERS_QUERY
      ),
    )
    .await;
    let ownership_query_draft_json = assert_api_success(ownership_query_draft).await;
    let ownership_draft_rows = ownership_query_draft_json["data"]
      .as_array()
      .expect("ownership draft query data must be array");
    assert_eq!(ownership_draft_rows.len(), 2);
    assert!(
      ownership_draft_rows
        .iter()
        .any(|row| row["id"] == ownership_draft_id),
      "draft query should include the first ownership transfer"
    );

    let reconciliation_draft = post_json(
      &app,
      api_paths::operations::RECONCILIATIONS_SAVE,
      operations_reconciliation_save(
        RECONCILIATION_QUERY_DOC_NUMBER_1,
        "2026-01-11T00:00:00Z",
        ctx.contractor_id,
        ctx.warehouse_id,
      ),
    )
    .await;
    let _ = assert_api_success(reconciliation_draft).await;

    let reconciliation_posted = post_json(
      &app,
      api_paths::operations::RECONCILIATIONS_SAVE_AND_EXECUTE,
      operations_reconciliation_save(
        RECONCILIATION_QUERY_DOC_NUMBER_2,
        "2026-01-11T01:00:00Z",
        ctx.contractor_id,
        ctx.warehouse_id,
      ),
    )
    .await;
    let reconciliation_posted_json = assert_api_success(reconciliation_posted).await;
    assert_eq!(reconciliation_posted_json["data"]["status"], "EXECUTED");

    let reconciliation_query_doc = get(
      &app,
      format!(
        "{}?documentNumber={}",
        api_paths::operations::RECONCILIATIONS_QUERY,
        RECONCILIATION_QUERY_DOC_NUMBER_1
      ),
    )
    .await;
    let reconciliation_query_doc_json = assert_api_success(reconciliation_query_doc).await;
    let reconciliation_doc_rows = reconciliation_query_doc_json["data"]
      .as_array()
      .expect("reconciliation document query data must be array");
    assert_eq!(reconciliation_doc_rows.len(), 1);
    assert_eq!(
      reconciliation_doc_rows[0]["documentNumber"],
      RECONCILIATION_QUERY_DOC_NUMBER_1
    );

    let reconciliation_query_status = get(
      &app,
      format!(
        "{}?status=EXECUTED",
        api_paths::operations::RECONCILIATIONS_QUERY
      ),
    )
    .await;
    let reconciliation_query_status_json = assert_api_success(reconciliation_query_status).await;
    let reconciliation_status_rows = reconciliation_query_status_json["data"]
      .as_array()
      .expect("reconciliation status query data must be array");
    assert_eq!(reconciliation_status_rows.len(), 1);
    assert_eq!(
      reconciliation_status_rows[0]["documentNumber"],
      RECONCILIATION_QUERY_DOC_NUMBER_2
    );

    let persisted_ownership = ownership_transfer::Entity::find()
      .filter(ownership_transfer::Column::Id.eq(Uuid::parse_str(&ownership_draft_id).unwrap()))
      .one(&*db)
      .await
      .unwrap();
    assert!(persisted_ownership.is_some());
  })
  .await;
}

#[tokio::test]
async fn blending_query_status_filter_returns_posted_only_and_rejects_invalid_status() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let ctx = seed_inventory_context(&db).await;

  with_auth_token(token, async {
    let draft_doc = post_json(
      &app,
      api_paths::blending::SAVE,
      blending_save(
        BLENDING_STATUS_QUERY_DOC_NUMBER_1,
        "2026-01-12T00:00:00Z",
        ctx.contractor_id,
        ctx.second_product_id,
      ),
    )
    .await;
    let _ = assert_api_success(draft_doc).await;

    seed_ledger_balance(
      &db,
      ctx.storage_a,
      ctx.product_id,
      ctx.contractor_id,
      Decimal::from(20),
    )
    .await;

    // Create blending doc_2 via composite (with balanced components/results for execution)
    let posted_doc = post_json(
      &app,
      api_paths::blending::COMPOSITE_SAVE,
      blending_composite_save(
        BLENDING_STATUS_QUERY_DOC_NUMBER_2,
        "2026-01-12T01:00:00Z",
        ctx.contractor_id,
        ctx.second_product_id,
        ctx.storage_a,
        ctx.product_id,
        "5.0",
        ctx.storage_b,
        "5.0",
      ),
    )
    .await;
    let posted_doc_json = assert_api_success(posted_doc).await;
    let posted_doc_id =
      Uuid::parse_str(posted_doc_json["data"]["document"]["id"].as_str().unwrap()).unwrap();

    let execute_posted = post_empty(
      &app,
      api_paths::blending::EXECUTE_BY_ID.replace("{id}", &posted_doc_id.to_string()),
    )
    .await;
    let _ = assert_api_success(execute_posted).await;

    let posted_query = get(
      &app,
      format!("{}?status=EXECUTED", api_paths::blending::QUERY),
    )
    .await;
    let posted_query_json = assert_api_success(posted_query).await;
    let posted_rows = posted_query_json["data"]
      .as_array()
      .expect("blending posted query data must be array");
    assert_eq!(posted_rows.len(), 1);
    assert_eq!(
      posted_rows[0]["documentNumber"],
      BLENDING_STATUS_QUERY_DOC_NUMBER_2
    );

    let draft_query = get(&app, format!("{}?status=DRAFT", api_paths::blending::QUERY)).await;
    let draft_query_json = assert_api_success(draft_query).await;
    let draft_rows = draft_query_json["data"]
      .as_array()
      .expect("blending draft query data must be array");
    assert_eq!(draft_rows.len(), 1);
    assert_eq!(
      draft_rows[0]["documentNumber"],
      BLENDING_STATUS_QUERY_DOC_NUMBER_1
    );

    let invalid_status_query = get(
      &app,
      format!("{}?status=INVALID", api_paths::blending::QUERY),
    )
    .await;
    let _ = assert_api_error(
      invalid_status_query,
      StatusCode::BAD_REQUEST,
      "VALIDATION_ERROR",
      Some("status"),
    )
    .await;
  })
  .await;
}

#[tokio::test]
async fn dispatch_and_reconciliation_query_reject_invalid_status_values() {
  let (_db, app, token) = setup_seeded_app_with_admin_token().await;

  with_auth_token(token, async {
    let dispatch_invalid_status = get(
      &app,
      format!("{}?status=INVALID", api_paths::dispatch::QUERY),
    )
    .await;
    let _ = assert_api_error(
      dispatch_invalid_status,
      StatusCode::BAD_REQUEST,
      "VALIDATION_ERROR",
      Some("status"),
    )
    .await;

    let reconciliation_invalid_status = get(
      &app,
      format!(
        "{}?status=INVALID",
        api_paths::operations::RECONCILIATIONS_QUERY
      ),
    )
    .await;
    let _ = assert_api_error(
      reconciliation_invalid_status,
      StatusCode::BAD_REQUEST,
      "VALIDATION_ERROR",
      Some("status"),
    )
    .await;
  })
  .await;
}

#[tokio::test]
async fn acceptance_and_ownership_query_reject_invalid_status_values() {
  let (_db, app, token) = setup_seeded_app_with_admin_token().await;

  with_auth_token(token, async {
    let acceptance_invalid_status = get(
      &app,
      format!("{}?status=INVALID", api_paths::acceptance::QUERY),
    )
    .await;
    let _ = assert_api_error(
      acceptance_invalid_status,
      StatusCode::BAD_REQUEST,
      "VALIDATION_ERROR",
      Some("status"),
    )
    .await;

    let ownership_invalid_status = get(
      &app,
      format!(
        "{}?status=INVALID",
        api_paths::operations::OWNERSHIP_TRANSFERS_QUERY
      ),
    )
    .await;
    let _ = assert_api_error(
      ownership_invalid_status,
      StatusCode::BAD_REQUEST,
      "VALIDATION_ERROR",
      Some("status"),
    )
    .await;
  })
  .await;
}

#[tokio::test]
async fn acceptance_query_supports_pagination_params_and_rejects_malformed_values() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let ctx = seed_inventory_context(&db).await;

  with_auth_token(token, async {
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::acceptance::SAVE,
        acceptance_save_truck(
          ACCEPTANCE_PAGINATION_DOC_NUMBER_1,
          "2026-01-13T00:00:00Z",
          ctx.contractor_id,
        ),
      )
      .await,
    )
    .await;
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::acceptance::SAVE,
        acceptance_save_truck(
          ACCEPTANCE_PAGINATION_DOC_NUMBER_2,
          "2026-01-13T01:00:00Z",
          ctx.contractor_id,
        ),
      )
      .await,
    )
    .await;
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::acceptance::SAVE,
        acceptance_save_truck(
          ACCEPTANCE_PAGINATION_DOC_NUMBER_3,
          "2026-01-13T02:00:00Z",
          ctx.contractor_id,
        ),
      )
      .await,
    )
    .await;

    let first_page = get(
      &app,
      format!("{}?page=1&per_page=2", api_paths::acceptance::QUERY),
    )
    .await;
    let first_page_json = assert_api_success(first_page).await;
    let first_page_rows = first_page_json["data"]
      .as_array()
      .expect("acceptance pagination query data must be array");
    assert_eq!(first_page_rows.len(), 2);

    let second_page = get(
      &app,
      format!("{}?page=2&per_page=2", api_paths::acceptance::QUERY),
    )
    .await;
    let second_page_json = assert_api_success(second_page).await;
    let second_page_rows = second_page_json["data"]
      .as_array()
      .expect("acceptance pagination query data must be array");
    assert_eq!(second_page_rows.len(), 1);

    let malformed_page = get(&app, format!("{}?page=abc", api_paths::acceptance::QUERY)).await;
    let _ = assert_api_error(
      malformed_page,
      StatusCode::BAD_REQUEST,
      "VALIDATION_ERROR",
      None,
    )
    .await;
  })
  .await;
}

#[tokio::test]
async fn dispatch_query_supports_pagination_params_and_rejects_malformed_values() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let ctx = seed_inventory_context(&db).await;

  with_auth_token(token, async {
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::dispatch::SAVE,
        dispatch_save_external_truck(
          DISPATCH_PAGINATION_DOC_NUMBER_1,
          "2026-01-13T03:00:00Z",
          ctx.contractor_id,
        ),
      )
      .await,
    )
    .await;
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::dispatch::SAVE,
        dispatch_save_external_truck(
          DISPATCH_PAGINATION_DOC_NUMBER_2,
          "2026-01-13T04:00:00Z",
          ctx.contractor_id,
        ),
      )
      .await,
    )
    .await;
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::dispatch::SAVE,
        dispatch_save_external_truck(
          DISPATCH_PAGINATION_DOC_NUMBER_3,
          "2026-01-13T05:00:00Z",
          ctx.contractor_id,
        ),
      )
      .await,
    )
    .await;

    let first_page = get(
      &app,
      format!("{}?page=1&per_page=2", api_paths::dispatch::QUERY),
    )
    .await;
    let first_page_json = assert_api_success(first_page).await;
    let first_page_rows = first_page_json["data"]
      .as_array()
      .expect("dispatch pagination query data must be array");
    assert_eq!(first_page_rows.len(), 2);

    let second_page = get(
      &app,
      format!("{}?page=2&per_page=2", api_paths::dispatch::QUERY),
    )
    .await;
    let second_page_json = assert_api_success(second_page).await;
    let second_page_rows = second_page_json["data"]
      .as_array()
      .expect("dispatch pagination query data must be array");
    assert_eq!(second_page_rows.len(), 1);

    let malformed_per_page = get(
      &app,
      format!("{}?per_page=oops", api_paths::dispatch::QUERY),
    )
    .await;
    let _ = assert_api_error(
      malformed_per_page,
      StatusCode::BAD_REQUEST,
      "VALIDATION_ERROR",
      None,
    )
    .await;
  })
  .await;
}

#[tokio::test]
async fn reconciliation_query_supports_pagination_params_and_rejects_malformed_values() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let ctx = seed_inventory_context(&db).await;

  with_auth_token(token, async {
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::operations::RECONCILIATIONS_SAVE,
        operations_reconciliation_save(
          RECONCILIATION_PAGINATION_DOC_NUMBER_1,
          "2026-01-13T06:00:00Z",
          ctx.contractor_id,
          ctx.warehouse_id,
        ),
      )
      .await,
    )
    .await;
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::operations::RECONCILIATIONS_SAVE,
        operations_reconciliation_save(
          RECONCILIATION_PAGINATION_DOC_NUMBER_2,
          "2026-01-13T07:00:00Z",
          ctx.contractor_id,
          ctx.warehouse_id,
        ),
      )
      .await,
    )
    .await;
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::operations::RECONCILIATIONS_SAVE,
        operations_reconciliation_save(
          RECONCILIATION_PAGINATION_DOC_NUMBER_3,
          "2026-01-13T08:00:00Z",
          ctx.contractor_id,
          ctx.warehouse_id,
        ),
      )
      .await,
    )
    .await;

    let first_page = get(
      &app,
      format!(
        "{}?page=1&per_page=2",
        api_paths::operations::RECONCILIATIONS_QUERY
      ),
    )
    .await;
    let first_page_json = assert_api_success(first_page).await;
    let first_page_rows = first_page_json["data"]
      .as_array()
      .expect("reconciliation pagination query data must be array");
    assert_eq!(first_page_rows.len(), 2);

    let second_page = get(
      &app,
      format!(
        "{}?page=2&per_page=2",
        api_paths::operations::RECONCILIATIONS_QUERY
      ),
    )
    .await;
    let second_page_json = assert_api_success(second_page).await;
    let second_page_rows = second_page_json["data"]
      .as_array()
      .expect("reconciliation pagination query data must be array");
    assert_eq!(second_page_rows.len(), 1);

    let malformed_page = get(
      &app,
      format!("{}?page=NaN", api_paths::operations::RECONCILIATIONS_QUERY),
    )
    .await;
    let _ = assert_api_error(
      malformed_page,
      StatusCode::BAD_REQUEST,
      "VALIDATION_ERROR",
      None,
    )
    .await;
  })
  .await;
}

#[tokio::test]
async fn ownership_transfer_query_supports_pagination_params_and_rejects_malformed_values() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let ctx = seed_inventory_context(&db).await;

  with_auth_token(token, async {
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::operations::OWNERSHIP_TRANSFERS_SAVE,
        operations_ownership_transfer(
          OWNERSHIP_PAGINATION_DATE_1,
          ctx.storage_a,
          ctx.product_id,
          ctx.contractor_id,
          ctx.contractor_id,
          "1.0",
        ),
      )
      .await,
    )
    .await;
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::operations::OWNERSHIP_TRANSFERS_SAVE,
        operations_ownership_transfer(
          OWNERSHIP_PAGINATION_DATE_2,
          ctx.storage_a,
          ctx.product_id,
          ctx.contractor_id,
          ctx.contractor_id,
          "2.0",
        ),
      )
      .await,
    )
    .await;
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::operations::OWNERSHIP_TRANSFERS_SAVE,
        operations_ownership_transfer(
          OWNERSHIP_PAGINATION_DATE_3,
          ctx.storage_a,
          ctx.product_id,
          ctx.contractor_id,
          ctx.contractor_id,
          "3.0",
        ),
      )
      .await,
    )
    .await;

    let first_page = get(
      &app,
      format!(
        "{}?page=1&per_page=2",
        api_paths::operations::OWNERSHIP_TRANSFERS_QUERY
      ),
    )
    .await;
    let first_page_json = assert_api_success(first_page).await;
    let first_page_rows = first_page_json["data"]
      .as_array()
      .expect("ownership pagination query data must be array");
    assert_eq!(first_page_rows.len(), 2);

    let second_page = get(
      &app,
      format!(
        "{}?page=2&per_page=2",
        api_paths::operations::OWNERSHIP_TRANSFERS_QUERY
      ),
    )
    .await;
    let second_page_json = assert_api_success(second_page).await;
    let second_page_rows = second_page_json["data"]
      .as_array()
      .expect("ownership pagination query data must be array");
    assert_eq!(second_page_rows.len(), 1);

    let malformed_page = get(
      &app,
      format!(
        "{}?page=oops",
        api_paths::operations::OWNERSHIP_TRANSFERS_QUERY
      ),
    )
    .await;
    let _ = assert_api_error(
      malformed_page,
      StatusCode::BAD_REQUEST,
      "VALIDATION_ERROR",
      None,
    )
    .await;
  })
  .await;
}

#[tokio::test]
async fn physical_transfer_query_supports_pagination_params_and_rejects_malformed_values() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let ctx = seed_inventory_context(&db).await;

  with_auth_token(token, async {
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::operations::PHYSICAL_TRANSFERS_SAVE,
        operations_physical_transfer(
          PHYSICAL_PAGINATION_DOC_NUMBER_1,
          "2026-01-13T12:00:00Z",
          "2026-01-13T12:10:00Z",
          "2026-01-13T12:20:00Z",
          ctx.contractor_id,
          ctx.product_id,
          ctx.storage_a,
          ctx.storage_b,
          "1.0",
        ),
      )
      .await,
    )
    .await;
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::operations::PHYSICAL_TRANSFERS_SAVE,
        operations_physical_transfer(
          PHYSICAL_PAGINATION_DOC_NUMBER_2,
          "2026-01-13T13:00:00Z",
          "2026-01-13T13:10:00Z",
          "2026-01-13T13:20:00Z",
          ctx.contractor_id,
          ctx.product_id,
          ctx.storage_a,
          ctx.storage_b,
          "2.0",
        ),
      )
      .await,
    )
    .await;
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::operations::PHYSICAL_TRANSFERS_SAVE,
        operations_physical_transfer(
          PHYSICAL_PAGINATION_DOC_NUMBER_3,
          "2026-01-13T14:00:00Z",
          "2026-01-13T14:10:00Z",
          "2026-01-13T14:20:00Z",
          ctx.contractor_id,
          ctx.product_id,
          ctx.storage_a,
          ctx.storage_b,
          "3.0",
        ),
      )
      .await,
    )
    .await;

    let first_page = get(
      &app,
      format!(
        "{}?page=1&per_page=2",
        api_paths::operations::PHYSICAL_TRANSFERS_QUERY
      ),
    )
    .await;
    let first_page_json = assert_api_success(first_page).await;
    let first_page_rows = first_page_json["data"]
      .as_array()
      .expect("physical transfer pagination query data must be array");
    assert_eq!(first_page_rows.len(), 2);

    let second_page = get(
      &app,
      format!(
        "{}?page=2&per_page=2",
        api_paths::operations::PHYSICAL_TRANSFERS_QUERY
      ),
    )
    .await;
    let second_page_json = assert_api_success(second_page).await;
    let second_page_rows = second_page_json["data"]
      .as_array()
      .expect("physical transfer pagination query data must be array");
    assert_eq!(second_page_rows.len(), 1);

    let malformed_per_page = get(
      &app,
      format!(
        "{}?per_page=invalid",
        api_paths::operations::PHYSICAL_TRANSFERS_QUERY
      ),
    )
    .await;
    let _ = assert_api_error(
      malformed_per_page,
      StatusCode::BAD_REQUEST,
      "VALIDATION_ERROR",
      None,
    )
    .await;
  })
  .await;
}

#[tokio::test]
async fn blending_query_supports_pagination_params_and_rejects_malformed_values() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let ctx = seed_inventory_context(&db).await;

  with_auth_token(token, async {
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::blending::SAVE,
        blending_save(
          BLENDING_PAGINATION_DOC_NUMBER_1,
          "2026-01-13T15:00:00Z",
          ctx.contractor_id,
          ctx.second_product_id,
        ),
      )
      .await,
    )
    .await;
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::blending::SAVE,
        blending_save(
          BLENDING_PAGINATION_DOC_NUMBER_2,
          "2026-01-13T16:00:00Z",
          ctx.contractor_id,
          ctx.second_product_id,
        ),
      )
      .await,
    )
    .await;
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::blending::SAVE,
        blending_save(
          BLENDING_PAGINATION_DOC_NUMBER_3,
          "2026-01-13T17:00:00Z",
          ctx.contractor_id,
          ctx.second_product_id,
        ),
      )
      .await,
    )
    .await;

    let first_page = get(
      &app,
      format!("{}?page=1&per_page=2", api_paths::blending::QUERY),
    )
    .await;
    let first_page_json = assert_api_success(first_page).await;
    let first_page_rows = first_page_json["data"]
      .as_array()
      .expect("blending pagination query data must be array");
    assert_eq!(first_page_rows.len(), 2);

    let second_page = get(
      &app,
      format!("{}?page=2&per_page=2", api_paths::blending::QUERY),
    )
    .await;
    let second_page_json = assert_api_success(second_page).await;
    let second_page_rows = second_page_json["data"]
      .as_array()
      .expect("blending pagination query data must be array");
    assert_eq!(second_page_rows.len(), 1);

    let malformed_per_page =
      get(&app, format!("{}?per_page=bad", api_paths::blending::QUERY)).await;
    let _ = assert_api_error(
      malformed_per_page,
      StatusCode::BAD_REQUEST,
      "VALIDATION_ERROR",
      None,
    )
    .await;
  })
  .await;
}

#[tokio::test]
async fn query_endpoints_reject_zero_page_and_per_page_values() {
  let (_db, app, token) = setup_seeded_app_with_admin_token().await;

  with_auth_token(token, async {
    let query_paths = [
      api_paths::acceptance::QUERY,
      api_paths::dispatch::QUERY,
      api_paths::blending::QUERY,
      api_paths::operations::PHYSICAL_TRANSFERS_QUERY,
      api_paths::operations::OWNERSHIP_TRANSFERS_QUERY,
      api_paths::operations::RECONCILIATIONS_QUERY,
    ];

    for query_path in query_paths {
      let zero_page = get(&app, format!("{}?page=0", query_path)).await;
      let _ = assert_api_error(
        zero_page,
        StatusCode::BAD_REQUEST,
        "VALIDATION_ERROR",
        Some("page and per_page"),
      )
      .await;

      let zero_per_page = get(&app, format!("{}?per_page=0", query_path)).await;
      let _ = assert_api_error(
        zero_per_page,
        StatusCode::BAD_REQUEST,
        "VALIDATION_ERROR",
        Some("page and per_page"),
      )
      .await;
    }
  })
  .await;
}

#[tokio::test]
async fn execute_endpoints_apply_route_specific_missing_document_semantics_and_error_payload_structure(
) {
  let (_db, app, token) = setup_seeded_app_with_admin_token().await;
  let unknown_id = Uuid::now_v7();

  with_auth_token(token, async {
    let acceptance_execute = post_empty(
      &app,
      api_paths::acceptance::EXECUTE_BY_ID.replace("{id}", &unknown_id.to_string()),
    )
    .await;
    let acceptance_json = assert_api_error(
      acceptance_execute,
      StatusCode::NOT_FOUND,
      "NOT_FOUND",
      Some("Acceptance document"),
    )
    .await;
    assert_eq!(acceptance_json["error"]["code"], "NOT_FOUND");

    let dispatch_execute = post_empty(
      &app,
      api_paths::dispatch::EXECUTE_BY_ID.replace("{id}", &unknown_id.to_string()),
    )
    .await;
    let dispatch_json = assert_api_error(
      dispatch_execute,
      StatusCode::NOT_FOUND,
      "NOT_FOUND",
      Some("Dispatch document"),
    )
    .await;
    assert_eq!(dispatch_json["error"]["code"], "NOT_FOUND");

    let blending_execute = post_empty(
      &app,
      api_paths::blending::EXECUTE_BY_ID.replace("{id}", &unknown_id.to_string()),
    )
    .await;
    let blending_json = assert_api_error(
      blending_execute,
      StatusCode::NOT_FOUND,
      "NOT_FOUND",
      Some("Blending document"),
    )
    .await;
    assert_eq!(blending_json["error"]["code"], "NOT_FOUND");
  })
  .await;
}
