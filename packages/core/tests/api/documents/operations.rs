use axum::http::StatusCode;
use sea_orm::prelude::Decimal;
use uuid::Uuid;
use voletu_core::endpoints::paths as api_paths;

use super::seed_inventory_context;
use crate::common::{
  catalog_seed::seed_ledger_balance,
  http::{
    assert_api_error,
    assert_api_success,
    post_empty,
    post_json,
    setup_seeded_app_with_admin_token,
    with_auth_token,
  },
  payloads::{
    blending_composite_save,
    operations_ownership_transfer,
    operations_physical_transfer,
    operations_reconciliation_adjustment,
    operations_reconciliation_save,
  },
};

const PHYSICAL_TRANSFER_DOC_NUMBER: &str = "PT-EP-1";
const RECONCILIATION_DOC_NUMBER: &str = "REC-EP-1";
const BLENDING_DOC_NUMBER: &str = "BLD-EP-1";

#[tokio::test]
async fn endpoints_execute_workflows_and_reject_invalid_blending_as_bad_request() {
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
