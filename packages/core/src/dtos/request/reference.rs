use uuid::Uuid;
use voletu_core_macros::request_dto;

#[request_dto]
pub struct CreateCompanyRequest {
  #[validate(length(min = 2))]
  pub common_name: String,
  pub legal_name: Option<String>,
  pub is_contractor: bool,
  pub is_exporter: bool,
  pub is_manufacturer: bool,
  pub is_sender: bool,
}

#[request_dto]
pub struct CreateProductTypeRequest {
  #[validate(length(min = 2))]
  pub common_name: String,
  pub long_name: Option<String>,
}

#[request_dto]
pub struct CreateProductGroupRequest {
  pub product_type_id: Uuid,
  #[validate(length(min = 2))]
  pub common_name: String,
  pub long_name: Option<String>,
}

#[request_dto]
pub struct CreateProductRequest {
  pub product_group_id: Uuid,
  pub manufacturer_id: Option<Uuid>,
  #[validate(length(min = 2))]
  pub common_name: String,
  pub long_name: Option<String>,
  pub add_identification: Option<String>,
  pub is_component: Option<bool>,
}

#[request_dto]
pub struct CreateBaseRequest {
  #[validate(length(min = 2))]
  pub common_name: String,
  pub long_name: Option<String>,
}

#[request_dto]
pub struct CreateWarehouseRequest {
  pub base_id: Uuid,
  #[validate(length(min = 2))]
  pub common_name: String,
  pub long_name: Option<String>,
}

#[request_dto]
pub struct CreateStorageRequest {
  pub warehouse_id: Uuid,
  #[validate(length(min = 2))]
  pub common_name: String,
  pub long_name: Option<String>,
  pub capacity: Option<f64>,
  pub is_type_specific: Option<bool>,
  pub product_type_id: Option<Uuid>,
}

#[request_dto]
pub struct CreatePortRequest {
  #[validate(length(min = 2))]
  pub common_name: String,
  pub country: Option<String>,
}
