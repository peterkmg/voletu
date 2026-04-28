use reqwest::Client;
use serde_json::{json, Value};
use uuid::Uuid;

use super::api_post;

/// IDs returned after seeding a two-base catalog via HTTP API.
#[derive(Clone, Copy, Debug)]
pub struct RoutingCatalog {
  pub base_alpha: Uuid,
  pub base_beta: Uuid,
  pub warehouse_alpha: Uuid,
  pub warehouse_beta: Uuid,
  pub storage_alpha: Uuid,
  pub storage_beta: Uuid,
  pub product_type: Uuid,
  pub product_group: Uuid,
  pub product: Uuid,
  pub product_b: Uuid,
  pub contractor: Uuid,
  pub contractor_b: Uuid,
}

/// Seed catalog entities on a running node via HTTP API.
/// Creates two bases (alpha/beta), each with a warehouse + storage, plus products and companies.
pub async fn seed_catalog_via_api(client: &Client, base_url: &str, token: &str) -> RoutingCatalog {
  let base_alpha = api_post(
    client,
    &format!("{base_url}/catalog/bases"),
    token,
    json!({
      "commonName": "Base Alpha",
      "longName": null,
    }),
  )
  .await;
  let base_alpha_id = parse_uuid(&base_alpha, "id");

  let base_beta = api_post(
    client,
    &format!("{base_url}/catalog/bases"),
    token,
    json!({
      "commonName": "Base Beta",
      "longName": null,
    }),
  )
  .await;
  let base_beta_id = parse_uuid(&base_beta, "id");

  let wh_alpha = api_post(
    client,
    &format!("{base_url}/catalog/warehouses"),
    token,
    json!({
      "baseId": base_alpha_id,
      "commonName": "Warehouse Alpha",
      "longName": null,
    }),
  )
  .await;
  let warehouse_alpha_id = parse_uuid(&wh_alpha, "id");

  let wh_beta = api_post(
    client,
    &format!("{base_url}/catalog/warehouses"),
    token,
    json!({
      "baseId": base_beta_id,
      "commonName": "Warehouse Beta",
      "longName": null,
    }),
  )
  .await;
  let warehouse_beta_id = parse_uuid(&wh_beta, "id");

  let st_alpha = api_post(
    client,
    &format!("{base_url}/catalog/storages"),
    token,
    json!({
      "warehouseId": warehouse_alpha_id,
      "commonName": "Tank Alpha",
      "longName": null,
      "capacity": null,
      "isTypeSpecific": false,
      "productTypeId": null,
    }),
  )
  .await;
  let storage_alpha_id = parse_uuid(&st_alpha, "id");

  let st_beta = api_post(
    client,
    &format!("{base_url}/catalog/storages"),
    token,
    json!({
      "warehouseId": warehouse_beta_id,
      "commonName": "Tank Beta",
      "longName": null,
      "capacity": null,
      "isTypeSpecific": false,
      "productTypeId": null,
    }),
  )
  .await;
  let storage_beta_id = parse_uuid(&st_beta, "id");

  let pt = api_post(
    client,
    &format!("{base_url}/catalog/product-types"),
    token,
    json!({
      "commonName": "Test Fuel",
      "longName": null,
    }),
  )
  .await;
  let product_type_id = parse_uuid(&pt, "id");

  let pg = api_post(
    client,
    &format!("{base_url}/catalog/product-groups"),
    token,
    json!({
      "productTypeId": product_type_id,
      "commonName": "Test Diesel",
      "longName": null,
    }),
  )
  .await;
  let product_group_id = parse_uuid(&pg, "id");

  let prod = api_post(
    client,
    &format!("{base_url}/catalog/products"),
    token,
    json!({
      "productGroupId": product_group_id,
      "manufacturerId": null,
      "commonName": "Product A",
      "longName": null,
      "addIdentification": null,
      "isComponent": true,
    }),
  )
  .await;
  let product_id = parse_uuid(&prod, "id");

  let prod_b = api_post(
    client,
    &format!("{base_url}/catalog/products"),
    token,
    json!({
      "productGroupId": product_group_id,
      "manufacturerId": null,
      "commonName": "Product B",
      "longName": null,
      "addIdentification": null,
      "isComponent": false,
    }),
  )
  .await;
  let product_b_id = parse_uuid(&prod_b, "id");

  let company = api_post(
    client,
    &format!("{base_url}/catalog/companies"),
    token,
    json!({
      "commonName": "Test Contractor",
      "legalName": null,
      "isContractor": true,
      "isExporter": false,
      "isManufacturer": false,
      "isSender": false,
    }),
  )
  .await;
  let contractor_id = parse_uuid(&company, "id");

  let company_b = api_post(
    client,
    &format!("{base_url}/catalog/companies"),
    token,
    json!({
      "commonName": "Test Contractor B",
      "legalName": null,
      "isContractor": true,
      "isExporter": false,
      "isManufacturer": false,
      "isSender": false,
    }),
  )
  .await;
  let contractor_b_id = parse_uuid(&company_b, "id");

  RoutingCatalog {
    base_alpha: base_alpha_id,
    base_beta: base_beta_id,
    warehouse_alpha: warehouse_alpha_id,
    warehouse_beta: warehouse_beta_id,
    storage_alpha: storage_alpha_id,
    storage_beta: storage_beta_id,
    product_type: product_type_id,
    product_group: product_group_id,
    product: product_id,
    product_b: product_b_id,
    contractor: contractor_id,
    contractor_b: contractor_b_id,
  }
}

/// Parse a UUID from a JSON value field.
fn parse_uuid(json: &Value, field: &str) -> Uuid {
  Uuid::parse_str(
    json[field]
      .as_str()
      .unwrap_or_else(|| panic!("missing {field} in response: {json}")),
  )
  .unwrap()
}
