use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use voletu_core_macros::request_dto;

#[request_dto]
pub struct CreateCompanyRequest {
  #[validate(length(min = 2))]
  pub common_name: String,
  #[validate(length(min = 2))]
  pub legal_name: Option<String>,
  pub is_contractor: bool,
  pub is_exporter: bool,
  pub is_manufacturer: bool,
  pub is_sender: bool,
}

#[request_dto]
pub struct UpdateCompanyRequest {
  #[validate(length(min = 2))]
  pub common_name: Option<String>,
  #[validate(length(min = 2))]
  pub legal_name: Option<String>,
  pub is_contractor: Option<bool>,
  pub is_exporter: Option<bool>,
  pub is_manufacturer: Option<bool>,
  pub is_sender: Option<bool>,
}

#[request_dto]
pub struct CreateProductTypeRequest {
  #[validate(length(min = 2))]
  pub common_name: String,
  #[validate(length(min = 2))]
  pub long_name: Option<String>,
}

#[request_dto]
pub struct UpdateProductTypeRequest {
  #[validate(length(min = 2))]
  pub common_name: Option<String>,
  #[validate(length(min = 2))]
  pub long_name: Option<String>,
}

#[request_dto]
pub struct CreateProductGroupRequest {
  pub product_type_id: Uuid,
  #[validate(length(min = 2))]
  pub common_name: String,
  #[validate(length(min = 2))]
  pub long_name: Option<String>,
}

#[request_dto]
pub struct UpdateProductGroupRequest {
  pub product_type_id: Option<Uuid>,
  #[validate(length(min = 2))]
  pub common_name: Option<String>,
  #[validate(length(min = 2))]
  pub long_name: Option<String>,
}

#[request_dto]
pub struct CreateProductRequest {
  pub product_group_id: Uuid,
  pub manufacturer_id: Option<Uuid>,
  #[validate(length(min = 2))]
  pub common_name: String,
  #[validate(length(min = 2))]
  pub long_name: Option<String>,
  #[validate(length(min = 2))]
  pub add_identification: Option<String>,
  pub is_component: Option<bool>,
}

#[request_dto]
pub struct UpdateProductRequest {
  pub product_group_id: Option<Uuid>,
  pub manufacturer_id: Option<Uuid>,
  #[validate(length(min = 2))]
  pub common_name: Option<String>,
  #[validate(length(min = 2))]
  pub long_name: Option<String>,
  #[validate(length(min = 2))]
  pub add_identification: Option<String>,
  pub is_component: Option<bool>,
}

#[request_dto]
pub struct CreateBaseRequest {
  #[validate(length(min = 2))]
  pub common_name: String,
  #[validate(length(min = 2))]
  pub long_name: Option<String>,
}

#[request_dto]
pub struct UpdateBaseRequest {
  #[validate(length(min = 2))]
  pub common_name: Option<String>,
  #[validate(length(min = 2))]
  pub long_name: Option<String>,
}

#[request_dto]
pub struct CreateWarehouseRequest {
  pub base_id: Uuid,
  #[validate(length(min = 2))]
  pub common_name: String,
  #[validate(length(min = 2))]
  pub long_name: Option<String>,
}

#[request_dto]
pub struct UpdateWarehouseRequest {
  pub base_id: Option<Uuid>,
  #[validate(length(min = 2))]
  pub common_name: Option<String>,
  #[validate(length(min = 2))]
  pub long_name: Option<String>,
}

#[request_dto]
pub struct CreateStorageRequest {
  pub warehouse_id: Uuid,
  #[validate(length(min = 2))]
  pub common_name: String,
  #[validate(length(min = 2))]
  pub long_name: Option<String>,
  pub capacity: Option<Decimal>,
  pub is_type_specific: Option<bool>,
  pub product_type_id: Option<Uuid>,
}

#[request_dto]
pub struct UpdateStorageRequest {
  pub warehouse_id: Option<Uuid>,
  #[validate(length(min = 2))]
  pub common_name: Option<String>,
  #[validate(length(min = 2))]
  pub long_name: Option<String>,
  pub capacity: Option<Decimal>,
  pub is_type_specific: Option<bool>,
  pub product_type_id: Option<Uuid>,
}

#[request_dto]
pub struct CreatePortRequest {
  #[validate(length(min = 2))]
  pub common_name: String,
  #[validate(length(min = 2))]
  pub country: Option<String>,
}

#[request_dto]
pub struct UpdatePortRequest {
  #[validate(length(min = 2))]
  pub common_name: Option<String>,
  #[validate(length(min = 2))]
  pub country: Option<String>,
}
