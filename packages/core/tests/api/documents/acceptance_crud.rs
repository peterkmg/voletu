use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;
use voletu_core::{endpoints::paths as api_paths, entities::inventory_ledger_entry};

use crate::common::http::{
  assert_api_success, post_empty, post_json, setup_seeded_app_with_admin_token, with_auth_token,
};
use crate::common::payloads::{
  acceptance_composite_save, acceptance_composite_save_and_execute,
};

use super::seed_inventory_context;

const ACCEPTANCE_DOC_NUMBER: &str = "ACC-EP-1";
const ACCEPTANCE_COMPOSITE_DOC_NUMBER: &str = "ACC-COMP-1";
const ACCEPTANCE_COMPOSITE_EXEC_DOC_NUMBER: &str = "ACC-COMP-2";

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
