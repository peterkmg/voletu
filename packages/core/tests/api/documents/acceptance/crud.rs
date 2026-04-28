use sea_orm::{ColumnTrait, QueryFilter};
use uuid::Uuid;
use voletu_core::{endpoints::paths as api_paths, entities::inventory_ledger_entry};

use super::super::seed_inventory_context;
use crate::common::{
  http::{
    assert_api_success,
    post_empty,
    post_json,
    setup_seeded_app_with_admin_token,
    with_auth_token,
  },
  payloads::acceptance_composite_save,
};

const ACCEPTANCE_DOC_NUMBER: &str = "ACC-EP-1";

#[tokio::test]
async fn endpoints_create_item_execute_and_return_expected_payloads() {
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
