use uuid::Uuid;
use voletu_core_macros::response_dto;

#[response_dto]
pub struct CompanyResponse {
  pub id: Uuid,
  pub common_name: String,
  pub legal_name: Option<String>,
  pub is_contractor: bool,
  pub is_exporter: bool,
  pub is_manufacturer: bool,
  pub is_sender: bool,
}

#[response_dto]
pub struct ProductTypeResponse {
  pub id: Uuid,
  pub common_name: String,
  pub long_name: Option<String>,
}

#[response_dto]
pub struct ProductGroupResponse {
  pub id: Uuid,
  pub product_type_id: Uuid,
  pub common_name: String,
  pub long_name: Option<String>,
}

#[response_dto]
pub struct ProductResponse {
  pub id: Uuid,
  pub product_group_id: Uuid,
  pub manufacturer_id: Option<Uuid>,
  pub common_name: String,
  pub long_name: Option<String>,
  pub add_identification: Option<String>,
  pub is_component: bool,
}

#[response_dto]
pub struct BaseResponse {
  pub id: Uuid,
  pub common_name: String,
  pub long_name: Option<String>,
}

#[response_dto]
pub struct WarehouseResponse {
  pub id: Uuid,
  pub base_id: Uuid,
  pub common_name: String,
  pub long_name: Option<String>,
}

#[response_dto]
pub struct StorageResponse {
  pub id: Uuid,
  pub warehouse_id: Uuid,
  pub common_name: String,
  pub long_name: Option<String>,
  pub capacity: Option<f64>,
  pub is_type_specific: bool,
  pub product_type_id: Option<Uuid>,
}

#[response_dto]
pub struct PortResponse {
  pub id: Uuid,
  pub common_name: String,
  pub country: Option<String>,
}
