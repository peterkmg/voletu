use axum::http::StatusCode;
use sea_orm::{prelude::Decimal, ColumnTrait, EntityLoaderTrait, EntityTrait, QueryFilter};
use uuid::Uuid;
use voletu_core::{endpoints::paths as api_paths, entities::ownership_transfer};

use super::seed_inventory_context;
use crate::common::{
  catalog_seed::seed_ledger_balance,
  http::{
    assert_api_error, assert_api_success, get, post_empty, post_json,
    setup_seeded_app_with_admin_token, with_auth_token,
  },
  payloads::{
    acceptance_composite_save, acceptance_save_truck, blending_composite_save, blending_save,
    dispatch_save_external_truck, operations_ownership_transfer, operations_physical_transfer,
    operations_reconciliation_save,
  },
};

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

    let persisted_ownership = ownership_transfer::Entity::load()
      .filter_by_id(Uuid::parse_str(&ownership_draft_id).unwrap())
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
