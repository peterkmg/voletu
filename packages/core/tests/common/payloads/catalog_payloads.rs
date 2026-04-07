use serde_json::json;
use uuid::Uuid;

pub fn catalog_company(
  common_name: &str,
  legal_name: Option<&str>,
  is_contractor: bool,
  is_exporter: bool,
  is_manufacturer: bool,
  is_sender: bool,
) -> String {
  json!({
    "commonName": common_name,
    "legalName": legal_name,
    "isContractor": is_contractor,
    "isExporter": is_exporter,
    "isManufacturer": is_manufacturer,
    "isSender": is_sender,
  })
  .to_string()
}

pub fn catalog_product_type(common_name: &str, long_name: Option<&str>) -> String {
  json!({
    "commonName": common_name,
    "longName": long_name,
  })
  .to_string()
}

pub fn catalog_product_group(
  product_type_id: Uuid,
  common_name: &str,
  long_name: Option<&str>,
) -> String {
  json!({
    "productTypeId": product_type_id,
    "commonName": common_name,
    "longName": long_name,
  })
  .to_string()
}

pub fn catalog_product(
  product_group_id: Uuid,
  manufacturer_id: Option<Uuid>,
  common_name: &str,
  long_name: Option<&str>,
  add_identification: Option<&str>,
  is_component: bool,
) -> String {
  json!({
    "productGroupId": product_group_id,
    "manufacturerId": manufacturer_id,
    "commonName": common_name,
    "longName": long_name,
    "addIdentification": add_identification,
    "isComponent": is_component,
  })
  .to_string()
}

pub fn catalog_base(common_name: &str, long_name: Option<&str>) -> String {
  json!({
    "commonName": common_name,
    "longName": long_name,
  })
  .to_string()
}

pub fn catalog_warehouse(base_id: Uuid, common_name: &str, long_name: Option<&str>) -> String {
  json!({
    "baseId": base_id,
    "commonName": common_name,
    "longName": long_name,
  })
  .to_string()
}

pub fn catalog_storage(
  warehouse_id: Uuid,
  common_name: &str,
  long_name: Option<&str>,
  capacity: Option<&str>,
  is_type_specific: bool,
  product_type_id: Option<Uuid>,
) -> String {
  json!({
    "warehouseId": warehouse_id,
    "commonName": common_name,
    "longName": long_name,
    "capacity": capacity,
    "isTypeSpecific": is_type_specific,
    "productTypeId": product_type_id,
  })
  .to_string()
}

pub fn catalog_port(common_name: &str, country: &str) -> String {
  json!({
    "commonName": common_name,
    "country": country,
  })
  .to_string()
}
