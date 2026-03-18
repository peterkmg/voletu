use axum::http::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;
use voletu_core::{
  endpoints::paths as api_paths,
  entities::{base, company, product_group, product_type, warehouse},
};

use crate::common::{
  http::{
    assert_api_error,
    assert_api_success,
    post_json,
    setup_seeded_app_with_admin_token,
    with_auth_token,
  },
  payloads::{
    catalog_base,
    catalog_company,
    catalog_port,
    catalog_product,
    catalog_product_group,
    catalog_product_type,
    catalog_storage,
    catalog_warehouse,
  },
};

const COMPANY_COMMON_NAME: &str = "Sender Inc";
const PRODUCT_TYPE_COMMON_NAME: &str = "Fuel";
const PRODUCT_GROUP_COMMON_NAME: &str = "Diesel";
const PRODUCT_COMMON_NAME: &str = "Diesel X";
const BASE_COMMON_NAME: &str = "Base A";

#[tokio::test]
async fn reference_create_endpoints_accept_valid_payloads_and_return_expected_dto_data() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;

  with_auth_token(token, async {
    let create_company = post_json(
      &app,
      api_paths::catalog::COMPANIES,
      catalog_company(
        "Sender Inc",
        Some("Sender Incorporated"),
        true,
        false,
        true,
        true,
      ),
    )
    .await;
    let company_json = assert_api_success(create_company).await;
    assert_eq!(company_json["data"]["commonName"], COMPANY_COMMON_NAME);
    assert_eq!(company_json["data"]["isSender"], true);

    let create_product_type = post_json(
      &app,
      api_paths::catalog::PRODUCT_TYPES,
      catalog_product_type("Fuel", Some("Fuel products")),
    )
    .await;
    let product_type_json = assert_api_success(create_product_type).await;
    assert_eq!(
      product_type_json["data"]["commonName"],
      PRODUCT_TYPE_COMMON_NAME
    );

    let company_id = company::Entity::find().one(&*db).await.unwrap().unwrap().id;
    let product_type_id = product_type::Entity::find()
      .one(&*db)
      .await
      .unwrap()
      .unwrap()
      .id;

    let create_product_group = post_json(
      &app,
      api_paths::catalog::PRODUCT_GROUPS,
      catalog_product_group(product_type_id, PRODUCT_GROUP_COMMON_NAME, None),
    )
    .await;
    let group_json = assert_api_success(create_product_group).await;
    assert_eq!(group_json["data"]["commonName"], PRODUCT_GROUP_COMMON_NAME);
    assert_eq!(
      group_json["data"]["productTypeId"],
      product_type_id.to_string()
    );

    let product_group_id = product_group::Entity::find()
      .filter(product_group::Column::ProductTypeId.eq(product_type_id))
      .one(&*db)
      .await
      .unwrap()
      .unwrap()
      .id;

    let create_product = post_json(
      &app,
      api_paths::catalog::PRODUCTS,
      catalog_product(
        product_group_id,
        Some(company_id),
        PRODUCT_COMMON_NAME,
        None,
        Some("DX"),
        true,
      ),
    )
    .await;
    let product_json = assert_api_success(create_product).await;
    assert_eq!(product_json["data"]["commonName"], PRODUCT_COMMON_NAME);
    assert_eq!(product_json["data"]["isComponent"], true);

    let create_base = post_json(
      &app,
      api_paths::catalog::BASES,
      catalog_base(BASE_COMMON_NAME, Some("Primary Base")),
    )
    .await;
    let base_json = assert_api_success(create_base).await;
    assert_eq!(base_json["data"]["commonName"], BASE_COMMON_NAME);

    let base_id = base::Entity::find().one(&*db).await.unwrap().unwrap().id;

    let create_warehouse = post_json(
      &app,
      api_paths::catalog::WAREHOUSES,
      catalog_warehouse(base_id, "WH-1", Some("Warehouse One")),
    )
    .await;
    let warehouse_json = assert_api_success(create_warehouse).await;
    assert_eq!(warehouse_json["data"]["baseId"], base_id.to_string());
    assert_eq!(warehouse_json["data"]["commonName"], "WH-1");

    let warehouse_id = warehouse::Entity::find()
      .filter(warehouse::Column::BaseId.eq(base_id))
      .one(&*db)
      .await
      .unwrap()
      .unwrap()
      .id;

    let create_storage = post_json(
      &app,
      api_paths::catalog::STORAGES,
      catalog_storage(
        warehouse_id,
        "Tank-1",
        Some("Tank 1"),
        Some("1000.0"),
        true,
        Some(product_type_id),
      ),
    )
    .await;
    let storage_json = assert_api_success(create_storage).await;
    assert_eq!(
      storage_json["data"]["warehouseId"],
      warehouse_id.to_string()
    );
    assert_eq!(
      storage_json["data"]["productTypeId"],
      product_type_id.to_string()
    );

    let create_port = post_json(
      &app,
      api_paths::catalog::PORTS,
      catalog_port("Port A", "EE"),
    )
    .await;
    let port_json = assert_api_success(create_port).await;
    assert_eq!(port_json["data"]["commonName"], "Port A");
    assert_eq!(port_json["data"]["country"], "EE");
  })
  .await;
}

#[tokio::test]
async fn reference_create_endpoints_surface_structured_errors_for_invalid_foreign_keys() {
  let (_db, app, token) = setup_seeded_app_with_admin_token().await;

  with_auth_token(token, async {
    let invalid_group_fk = post_json(
      &app,
      api_paths::catalog::PRODUCT_GROUPS,
      catalog_product_group(Uuid::now_v7(), "Diesel", None),
    )
    .await;
    let invalid_group_fk_json = assert_api_error(
      invalid_group_fk,
      StatusCode::INTERNAL_SERVER_ERROR,
      "DATABASE_ERROR",
      Some("Database error"),
    )
    .await;
    assert_eq!(invalid_group_fk_json["error"]["code"], "DATABASE_ERROR");

    let invalid_warehouse_fk = post_json(
      &app,
      api_paths::catalog::STORAGES,
      catalog_storage(
        Uuid::now_v7(),
        "Tank-1",
        Some("Tank 1"),
        Some("1000.0"),
        false,
        None,
      ),
    )
    .await;
    let invalid_warehouse_fk_json = assert_api_error(
      invalid_warehouse_fk,
      StatusCode::INTERNAL_SERVER_ERROR,
      "DATABASE_ERROR",
      Some("Database error"),
    )
    .await;
    assert_eq!(invalid_warehouse_fk_json["error"]["code"], "DATABASE_ERROR");
  })
  .await;
}
