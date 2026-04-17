use chrono::{DateTime, Utc};
use sea_orm::{entity::prelude::Decimal, ActiveValue::Set};
use uuid::Uuid;
use validator::Validate;
use voletu_core_macros::request_dto;

use crate::{
  entities::{inventory_adjustment, inventory_reconciliation},
  enums::{AdjustmentType, DocumentStatus},
};

#[request_dto]
pub struct CreateInventoryReconciliationRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date: DateTime<Utc>,
  pub contractor_id: Uuid,
  pub warehouse_id: Uuid,
}

#[request_dto]
pub struct UpdateInventoryReconciliationRequest {
  #[validate(length(min = 1))]
  pub document_number: Option<String>,
  pub date: Option<DateTime<Utc>>,
  pub contractor_id: Option<Uuid>,
  pub warehouse_id: Option<Uuid>,
}

#[request_dto]
pub struct CreateInventoryAdjustmentRequest {
  pub reconciliation_id: Uuid,
  pub storage_id: Uuid,
  pub product_id: Uuid,
  pub adjustment_type: AdjustmentType,
  pub amount: Decimal,
  #[validate(length(min = 1))]
  pub reason: Option<String>,
}

#[request_dto]
pub struct UpdateInventoryAdjustmentRequest {
  pub storage_id: Option<Uuid>,
  pub product_id: Option<Uuid>,
  pub adjustment_type: Option<AdjustmentType>,
  pub amount: Option<Decimal>,
  #[validate(length(min = 1))]
  pub reason: Option<String>,
}

#[request_dto]
pub struct InventoryAdjustmentCompositeRequest {
  pub storage_id: Uuid,
  pub product_id: Uuid,
  pub adjustment_type: AdjustmentType,
  pub amount: Decimal,
  #[validate(length(min = 1))]
  pub reason: Option<String>,
}

#[request_dto]
pub struct CreateInventoryReconciliationCompositeRequest {
  #[validate(nested)]
  #[serde(flatten)]
  pub reconciliation: CreateInventoryReconciliationRequest,
  #[validate(length(min = 1), nested)]
  pub adjustments: Vec<InventoryAdjustmentCompositeRequest>,
}

/// Update payload for one adjustment in a reconciliation composite update.
///
/// Each adjustment is a full replacement of its current state, not a partial
/// patch: `storage_id`, `product_id`, `adjustment_type`, and `amount` are all
/// required and overwrite whatever the existing row held. Adjustments present
/// here that don't exist on the document are inserted; existing adjustments
/// not present here are deleted.
#[request_dto]
pub struct UpdateInventoryAdjustmentCompositeRequest {
  /// Present for existing adjustments (an UPDATE), absent for newly inserted
  /// adjustments (an INSERT).
  pub id: Option<Uuid>,
  pub storage_id: Uuid,
  pub product_id: Uuid,
  pub adjustment_type: AdjustmentType,
  pub amount: Decimal,
  #[validate(length(min = 1))]
  pub reason: Option<String>,
}

#[request_dto]
pub struct UpdateInventoryReconciliationCompositeRequest {
  /// Header fields applied as a partial update
  /// (mirrors per-row UpdateInventoryReconciliationRequest).
  #[validate(nested)]
  #[serde(flatten)]
  pub reconciliation: UpdateInventoryReconciliationRequest,
  /// Full new adjustments list, diff-applied against existing rows.
  /// Adjustments with `id: Some(uuid)` matching an existing row are updated.
  /// Adjustments with `id: None` are inserted.
  /// Existing adjustments not present in this list are hard-deleted.
  #[validate(length(min = 1), nested)]
  pub adjustments: Vec<UpdateInventoryAdjustmentCompositeRequest>,
}

impl From<&InventoryAdjustmentCompositeRequest> for inventory_adjustment::ActiveModelEx {
  fn from(adj: &InventoryAdjustmentCompositeRequest) -> Self {
    Self {
      storage_id: Set(adj.storage_id),
      product_id: Set(adj.product_id),
      adjustment_type: Set(adj.adjustment_type),
      amount: Set(adj.amount),
      reason: Set(adj.reason.clone()),
      ..Default::default()
    }
  }
}

impl From<&CreateInventoryReconciliationCompositeRequest>
  for inventory_reconciliation::ActiveModelEx
{
  fn from(req: &CreateInventoryReconciliationCompositeRequest) -> Self {
    Self {
      document_number: Set(req.reconciliation.document_number.clone()),
      date: Set(req.reconciliation.date),
      status: Set(DocumentStatus::Draft),
      version: Set(1),
      executed_at: Set(None),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      contractor_id: Set(req.reconciliation.contractor_id),
      warehouse_id: Set(req.reconciliation.warehouse_id),
      adjustments: req
        .adjustments
        .iter()
        .map(inventory_adjustment::ActiveModelEx::from)
        .collect::<Vec<_>>()
        .into(),
      ..Default::default()
    }
  }
}
