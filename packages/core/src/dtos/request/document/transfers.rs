use chrono::{DateTime, Utc};
use sea_orm::{entity::prelude::Decimal, ActiveValue::Set};
use uuid::Uuid;
use validator::Validate;
use voletu_core_macros::request_dto;

use crate::{
  entities::{
    ownership_transfer,
    ownership_transfer_item,
    physical_storage_transfer,
    physical_transfer_item,
  },
  enums::DocumentStatus,
};

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

/// Update payload for one item in a physical-transfer composite update.
///
/// Each item is a full replacement of its current state, not a partial patch:
/// `product_id`, `from_storage_id`, `to_storage_id`, and `amount` are all
/// required and overwrite whatever the existing row held. Items present here
/// that don't exist on the document are inserted; existing items not present
/// here are deleted.
#[request_dto]
pub struct UpdatePhysicalTransferItemCompositeRequest {
  /// Present for existing items (an UPDATE), absent for newly inserted items (an INSERT).
  pub id: Option<Uuid>,
  pub product_id: Uuid,
  pub from_storage_id: Uuid,
  pub to_storage_id: Uuid,
  pub amount: Decimal,
}

#[request_dto]
pub struct UpdatePhysicalTransferCompositeRequest {
  /// Header fields applied as a partial update (mirrors per-row UpdatePhysicalTransferRequest).
  #[validate(nested)]
  #[serde(flatten)]
  pub physical_transfer: UpdatePhysicalTransferRequest,
  /// Full new items list, diff-applied against existing rows.
  /// Items with `id: Some(uuid)` matching an existing row are updated.
  /// Items with `id: None` are inserted.
  /// Existing items not present in this list are hard-deleted.
  #[validate(length(min = 1), nested)]
  pub items: Vec<UpdatePhysicalTransferItemCompositeRequest>,
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

/// Update payload for one item in an ownership-transfer composite update.
///
/// Each item is a full replacement of its current state, not a partial patch:
/// `storage_id`, `product_id`, `from_contractor_id`, `to_contractor_id`, and
/// `amount` are all required and overwrite whatever the existing row held.
/// Items present here that don't exist on the document are inserted; existing
/// items not present here are deleted.
#[request_dto]
pub struct UpdateOwnershipTransferItemCompositeRequest {
  /// Present for existing items (an UPDATE), absent for newly inserted items (an INSERT).
  pub id: Option<Uuid>,
  pub storage_id: Uuid,
  pub product_id: Uuid,
  pub from_contractor_id: Uuid,
  pub to_contractor_id: Uuid,
  pub amount: Decimal,
}

#[request_dto]
pub struct UpdateOwnershipTransferCompositeRequest {
  /// Header fields applied as a partial update (mirrors per-row UpdateOwnershipTransferRequest).
  #[validate(nested)]
  #[serde(flatten)]
  pub ownership_transfer: UpdateOwnershipTransferRequest,
  /// Full new items list, diff-applied against existing rows.
  /// Items with `id: Some(uuid)` matching an existing row are updated.
  /// Items with `id: None` are inserted.
  /// Existing items not present in this list are hard-deleted.
  #[validate(length(min = 1), nested)]
  pub items: Vec<UpdateOwnershipTransferItemCompositeRequest>,
}

impl From<&PhysicalTransferItemCompositeRequest> for physical_transfer_item::ActiveModelEx {
  fn from(item: &PhysicalTransferItemCompositeRequest) -> Self {
    Self {
      product_id: Set(item.product_id),
      from_storage_id: Set(item.from_storage_id),
      to_storage_id: Set(item.to_storage_id),
      amount: Set(item.amount),
      ..Default::default()
    }
  }
}

impl From<&CreatePhysicalTransferRequest> for physical_storage_transfer::ActiveModelEx {
  fn from(req: &CreatePhysicalTransferRequest) -> Self {
    Self {
      document_number: Set(req.document_number.clone()),
      date: Set(req.date),
      status: Set(DocumentStatus::Draft),
      version: Set(1),
      executed_at: Set(None),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      contractor_id: Set(req.contractor_id),
      start_cargo_ops: Set(req.start_cargo_ops),
      end_cargo_ops: Set(req.end_cargo_ops),
      items: req
        .items
        .iter()
        .map(physical_transfer_item::ActiveModelEx::from)
        .collect::<Vec<_>>()
        .into(),
      ..Default::default()
    }
  }
}

impl From<&OwnershipTransferItemCompositeRequest> for ownership_transfer_item::ActiveModelEx {
  fn from(item: &OwnershipTransferItemCompositeRequest) -> Self {
    Self {
      storage_id: Set(item.storage_id),
      product_id: Set(item.product_id),
      from_contractor_id: Set(item.from_contractor_id),
      to_contractor_id: Set(item.to_contractor_id),
      amount: Set(item.amount),
      ..Default::default()
    }
  }
}

impl From<&CreateOwnershipTransferRequest> for ownership_transfer::ActiveModelEx {
  fn from(req: &CreateOwnershipTransferRequest) -> Self {
    Self {
      date: Set(req.date),
      status: Set(DocumentStatus::Draft),
      version: Set(1),
      executed_at: Set(None),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      items: req
        .items
        .iter()
        .map(ownership_transfer_item::ActiveModelEx::from)
        .collect::<Vec<_>>()
        .into(),
      ..Default::default()
    }
  }
}
