use uuid::Uuid;
use voletu_core::endpoints::paths as api_paths;

use super::seed_inventory_context;
use crate::common::{
  http::{assert_api_success, get, post_json, setup_seeded_app_with_admin_token, with_auth_token},
  payloads::{operations_ownership_transfer, operations_physical_transfer},
};

const PHYSICAL_TRANSFER_NAMES_DOC_NUMBER: &str = "PT-NAMES-1";
const OWNERSHIP_TRANSFER_NAMES_DATE: &str = "2026-01-08T03:00:00Z";

#[tokio::test]
async fn physical_and_ownership_endpoints_return_names_when_embed_names_is_requested() {
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
