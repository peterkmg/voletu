use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use validator::Validate;
use voletu_core_macros::request_dto;

#[request_dto]
#[validate(schema(function = "crate::dtos::validators::validate_physical_transfer_request"))]
pub struct CreatePhysicalTransferRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date: DateTime<Utc>,
  pub contractor_id: Uuid,
  pub start_cargo_ops: DateTime<Utc>,
  pub end_cargo_ops: DateTime<Utc>,
  #[validate(length(min = 1), nested)]
  pub items: Vec<PhysicalTransferItemCompositeRequest>,
}

#[request_dto]
pub struct UpdatePhysicalTransferRequest {
  #[validate(length(min = 1))]
  pub document_number: Option<String>,
  pub date: Option<DateTime<Utc>>,
  pub contractor_id: Option<Uuid>,
  pub start_cargo_ops: Option<DateTime<Utc>>,
  pub end_cargo_ops: Option<DateTime<Utc>>,
}

#[request_dto]
pub struct CreateOwnershipTransferRequest {
  pub date: DateTime<Utc>,
  #[validate(length(min = 1), nested)]
  pub items: Vec<OwnershipTransferItemCompositeRequest>,
}

#[request_dto]
pub struct UpdateOwnershipTransferRequest {
  pub date: Option<DateTime<Utc>>,
}

#[request_dto]
pub struct PhysicalTransferItemCompositeRequest {
  pub product_id: Uuid,
  pub from_storage_id: Uuid,
  pub to_storage_id: Uuid,
  pub amount: Decimal,
}

#[request_dto]
pub struct CreatePhysicalTransferItemRequest {
  pub physical_transfer_id: Uuid,
  #[validate(nested)]
  #[serde(flatten)]
  pub item: PhysicalTransferItemCompositeRequest,
}

impl CreatePhysicalTransferItemRequest {
  pub fn from_composite(
    physical_transfer_id: Uuid,
    item: &PhysicalTransferItemCompositeRequest,
  ) -> Self {
    Self {
      physical_transfer_id,
      item: item.clone(),
    }
  }
}

#[request_dto]
pub struct UpdatePhysicalTransferItemRequest {
  pub product_id: Option<Uuid>,
  pub from_storage_id: Option<Uuid>,
  pub to_storage_id: Option<Uuid>,
  pub amount: Option<Decimal>,
}

#[request_dto]
pub struct OwnershipTransferItemCompositeRequest {
  pub storage_id: Uuid,
  pub product_id: Uuid,
  pub from_contractor_id: Uuid,
  pub to_contractor_id: Uuid,
  pub amount: Decimal,
}

#[request_dto]
pub struct CreateOwnershipTransferItemRequest {
  pub ownership_transfer_id: Uuid,
  #[validate(nested)]
  #[serde(flatten)]
  pub item: OwnershipTransferItemCompositeRequest,
}

impl CreateOwnershipTransferItemRequest {
  pub fn from_composite(
    ownership_transfer_id: Uuid,
    item: &OwnershipTransferItemCompositeRequest,
  ) -> Self {
    Self {
      ownership_transfer_id,
      item: item.clone(),
    }
  }
}

#[request_dto]
pub struct UpdateOwnershipTransferItemRequest {
  pub storage_id: Option<Uuid>,
  pub product_id: Option<Uuid>,
  pub from_contractor_id: Option<Uuid>,
  pub to_contractor_id: Option<Uuid>,
  pub amount: Option<Decimal>,
}
