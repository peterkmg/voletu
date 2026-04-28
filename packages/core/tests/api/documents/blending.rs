use sea_orm::{prelude::Decimal, ColumnTrait, QueryFilter};
use uuid::Uuid;
use voletu_core::{endpoints::paths as api_paths, entities::inventory_ledger_entry};

use super::seed_inventory_context;
use crate::common::{
  catalog_seed::seed_ledger_balance,
  http::{assert_api_success, get, post_json, setup_seeded_app_with_admin_token, with_auth_token},
  payloads::{blending_composite_save, blending_composite_save_and_execute},
};

const BLENDING_COMPOSITE_DOC_NUMBER: &str = "BLD-COMP-1";
const BLENDING_COMPOSITE_EXEC_DOC_NUMBER: &str = "BLD-COMP-2";
const BLENDING_NAMES_DOC_NUMBER: &str = "BLD-NAMES-1";

#[tokio::test]
async fn composite_endpoint_auto_executes_and_applies_ledger_entries() {
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
async fn endpoints_return_related_names_when_embed_names_is_requested() {
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
