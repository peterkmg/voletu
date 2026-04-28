use uuid::Uuid;
use voletu_core::endpoints::paths as api_paths;

use super::seed_inventory_context;
use crate::common::{
  http::{assert_api_success, get, post_json, setup_seeded_app_with_admin_token, with_auth_token},
  payloads::operations_reconciliation_save,
};

const RECONCILIATION_NAMES_DOC_NUMBER: &str = "REC-NAMES-1";

#[tokio::test]
async fn endpoints_return_related_names_when_embed_names_is_requested() {
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
