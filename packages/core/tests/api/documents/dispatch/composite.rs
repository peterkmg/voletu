use axum::http::StatusCode;
use sea_orm::prelude::Decimal;
use voletu_core::{endpoints::paths as api_paths, services::ledger::LedgerService};

use crate::{
  common::{
    catalog_seed::seed_ledger_balance,
    http::{post_json, response_json, setup_seeded_app_with_admin_token, with_auth_token},
    payloads::{
      dispatch_composite_save_and_execute,
      dispatch_composite_save_and_execute_with_measurement,
    },
  },
  documents::seed_inventory_context,
};

const DISPATCH_COMPOSITE_DOC_NUMBER: &str = "DISP-COMP-1";
const DISPATCH_COMPOSITE_EXEC_DOC_NUMBER: &str = "DISP-COMP-2";

#[tokio::test]
async fn endpoint_auto_executes_and_applies_ledger_entries() {
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

    let ledger_entry = LedgerService::new(db.clone())
      .balance_by_dimensions(ctx.storage_a, ctx.product_id, ctx.contractor_id)
      .await
      .unwrap()
      .unwrap();
    assert_eq!(ledger_entry.current_amount.to_string(), "6");
  })
  .await;
}
