use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use voletu_core_macros::response_dto;

use crate::entities::{
  base,
  company,
  port,
  product,
  product_group,
  product_type,
  storage,
  warehouse,
};

/// Response DTO for the `company` entity.
#[response_dto(service_fields(common))]
pub struct CompanyResponse {
  pub id: Uuid,
  pub common_name: String,
  pub legal_name: Option<String>,
  pub is_contractor: bool,
  pub is_exporter: bool,
  pub is_manufacturer: bool,
  pub is_sender: bool,
}

/// Response DTO for the `product_type` entity.
#[response_dto(service_fields(common))]
pub struct ProductTypeResponse {
  pub id: Uuid,
  pub common_name: String,
  pub long_name: Option<String>,
}

/// Response DTO for the `product_group` entity.
#[response_dto(service_fields(common))]
pub struct ProductGroupResponse {
  pub id: Uuid,
  pub product_type_id: Uuid,
  pub common_name: String,
  pub long_name: Option<String>,
}

/// Response DTO for the `product` entity.
#[response_dto(service_fields(common))]
pub struct ProductResponse {
  pub id: Uuid,
  pub product_group_id: Uuid,
  pub manufacturer_id: Option<Uuid>,
  pub common_name: String,
  pub long_name: Option<String>,
  pub add_identification: Option<String>,
  pub is_component: bool,
}

/// Response DTO for the `base` entity.
#[response_dto(service_fields(common))]
pub struct BaseResponse {
  pub id: Uuid,
  pub common_name: String,
  pub long_name: Option<String>,
}

/// Response DTO for the `warehouse` entity.
#[response_dto(service_fields(common))]
pub struct WarehouseResponse {
  pub id: Uuid,
  pub base_id: Uuid,
  pub common_name: String,
  pub long_name: Option<String>,
}

/// Response DTO for the `storage` entity.
#[response_dto(service_fields(common))]
pub struct StorageResponse {
  pub id: Uuid,
  pub warehouse_id: Uuid,
  pub common_name: String,
  pub long_name: Option<String>,
  pub capacity: Option<Decimal>,
  pub is_type_specific: bool,
  pub product_type_id: Option<Uuid>,
}

/// Response DTO for the `port` entity.
#[response_dto(service_fields(common))]
pub struct PortResponse {
  pub id: Uuid,
  pub common_name: String,
  pub country: Option<String>,
}

impl From<company::Model> for CompanyResponse {
  fn from(row: company::Model) -> Self {
    Self {
      id: row.id,
      common_name: row.common_name,
      legal_name: row.legal_name,
      is_contractor: row.is_contractor,
      is_exporter: row.is_exporter,
      is_manufacturer: row.is_manufacturer,
      is_sender: row.is_sender,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
    }
  }
}

impl From<product_type::Model> for ProductTypeResponse {
  fn from(row: product_type::Model) -> Self {
    Self {
      id: row.id,
      common_name: row.common_name,
      long_name: row.long_name,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
    }
  }
}

impl From<product_group::Model> for ProductGroupResponse {
  fn from(row: product_group::Model) -> Self {
    Self {
      id: row.id,
      product_type_id: row.product_type_id,
      common_name: row.common_name,
      long_name: row.long_name,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
    }
  }
}

impl From<product::Model> for ProductResponse {
  fn from(row: product::Model) -> Self {
    Self {
      id: row.id,
      product_group_id: row.product_group_id,
      manufacturer_id: row.manufacturer_id,
      common_name: row.common_name,
      long_name: row.long_name,
      add_identification: row.add_identification,
      is_component: row.is_component,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
    }
  }
}

impl From<base::Model> for BaseResponse {
  fn from(row: base::Model) -> Self {
    Self {
      id: row.id,
      common_name: row.common_name,
      long_name: row.long_name,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
    }
  }
}

impl From<warehouse::Model> for WarehouseResponse {
  fn from(row: warehouse::Model) -> Self {
    Self {
      id: row.id,
      base_id: row.base_id,
      common_name: row.common_name,
      long_name: row.long_name,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
    }
  }
}

impl From<storage::Model> for StorageResponse {
  fn from(row: storage::Model) -> Self {
    Self {
      id: row.id,
      warehouse_id: row.warehouse_id,
      common_name: row.common_name,
      long_name: row.long_name,
      capacity: row.capacity,
      is_type_specific: row.is_type_specific,
      product_type_id: row.product_type_id,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
    }
  }
}

impl From<port::Model> for PortResponse {
  fn from(row: port::Model) -> Self {
    Self {
      id: row.id,
      common_name: row.common_name,
      country: row.country,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
    }
  }
}
