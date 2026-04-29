use voletu_core::{endpoints::paths as api_paths, services::ledger::LedgerService};

use super::super::seed_inventory_context;
use crate::common::{
  http::{assert_api_success, post_json, setup_seeded_app_with_admin_token, with_auth_token},
  payloads::acceptance_composite_save_and_execute,
};

const ACCEPTANCE_COMPOSITE_DOC_NUMBER: &str = "ACC-COMP-1";
const ACCEPTANCE_COMPOSITE_EXEC_DOC_NUMBER: &str = "ACC-COMP-2";

#[tokio::test]
async fn endpoint_auto_executes_and_applies_ledger_entries() {
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

    let ledger_entry = LedgerService::new(db.clone())
      .balance_by_dimensions(ctx.storage_a, ctx.product_id, ctx.contractor_id)
      .await
      .unwrap()
      .unwrap();
    assert_eq!(ledger_entry.current_amount.to_string(), "7");
  })
  .await;
}
