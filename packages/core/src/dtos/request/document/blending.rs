use chrono::{DateTime, Utc};
use sea_orm::{entity::prelude::Decimal, ActiveValue::Set};
use uuid::Uuid;
use validator::Validate;
use voletu_core_macros::request_dto;

use crate::{
  entities::{blending_component, blending_document, blending_result},
  enums::DocumentStatus,
};

#[request_dto]
pub struct CreateBlendingRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date: DateTime<Utc>,
  pub contractor_id: Uuid,
  pub target_product_id: Uuid,
}

#[request_dto]
pub struct UpdateBlendingRequest {
  #[validate(length(min = 1))]
  pub document_number: Option<String>,
  pub date: Option<DateTime<Utc>>,
  pub contractor_id: Option<Uuid>,
  pub target_product_id: Option<Uuid>,
}

#[request_dto]
pub struct CreateBlendingComponentRequest {
  pub blending_doc_id: Uuid,
  #[validate(nested)]
  #[serde(flatten)]
  pub component: BlendingComponentCompositeRequest,
}

impl CreateBlendingComponentRequest {
  pub fn from_composite(
    blending_doc_id: Uuid,
    component: &BlendingComponentCompositeRequest,
  ) -> Self {
    Self {
      blending_doc_id,
      component: component.clone(),
    }
  }
}

#[request_dto]
pub struct UpdateBlendingComponentRequest {
  pub storage_id: Option<Uuid>,
  pub source_product_id: Option<Uuid>,
  pub amount_used: Option<Decimal>,
}

#[request_dto]
pub struct CreateBlendingResultRequest {
  pub blending_doc_id: Uuid,
  #[validate(nested)]
  #[serde(flatten)]
  pub result: BlendingResultCompositeRequest,
}

impl CreateBlendingResultRequest {
  pub fn from_composite(blending_doc_id: Uuid, result: &BlendingResultCompositeRequest) -> Self {
    Self {
      blending_doc_id,
      result: result.clone(),
    }
  }
}

#[request_dto]
pub struct UpdateBlendingResultRequest {
  pub storage_id: Option<Uuid>,
  pub produced_amount: Option<Decimal>,
}

#[request_dto]
pub struct BlendingComponentCompositeRequest {
  pub storage_id: Uuid,
  pub source_product_id: Uuid,
  pub amount_used: Decimal,
}

#[request_dto]
pub struct BlendingResultCompositeRequest {
  pub storage_id: Uuid,
  pub produced_amount: Decimal,
}

#[request_dto]
pub struct CreateBlendingCompositeRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date: DateTime<Utc>,
  pub contractor_id: Uuid,
  pub target_product_id: Uuid,
  #[validate(length(min = 1), nested)]
  pub components: Vec<BlendingComponentCompositeRequest>,
  #[validate(length(min = 1), nested)]
  pub results: Vec<BlendingResultCompositeRequest>,
}

impl CreateBlendingRequest {
  pub fn from_composite(req: &CreateBlendingCompositeRequest) -> Self {
    Self {
      document_number: req.document_number.clone(),
      date: req.date,
      contractor_id: req.contractor_id,
      target_product_id: req.target_product_id,
    }
  }
}

impl From<&BlendingComponentCompositeRequest> for blending_component::ActiveModelEx {
  fn from(component: &BlendingComponentCompositeRequest) -> Self {
    Self {
      storage_id: Set(component.storage_id),
      source_product_id: Set(component.source_product_id),
      amount_used: Set(component.amount_used),
      ..Default::default()
    }
  }
}

impl From<&BlendingResultCompositeRequest> for blending_result::ActiveModelEx {
  fn from(result: &BlendingResultCompositeRequest) -> Self {
    Self {
      storage_id: Set(result.storage_id),
      produced_amount: Set(result.produced_amount),
      ..Default::default()
    }
  }
}

/// Update payload for a single blending component inside a composite update.
///
/// Diff conventions match the dispatch / rail / truck composite update DTOs:
/// rows with `id: Some(_)` are updates against the existing row, rows with
/// `id: None` are inserts, and rows present in the database but missing from
/// the request are hard-deleted.
#[request_dto]
pub struct UpdateBlendingComponentCompositeRequest {
  /// Present for existing rows (UPDATE), absent for newly inserted rows (INSERT).
  pub id: Option<Uuid>,
  pub storage_id: Uuid,
  pub source_product_id: Uuid,
  pub amount_used: Decimal,
}

/// Update payload for a single blending result inside a composite update.
#[request_dto]
pub struct UpdateBlendingResultCompositeRequest {
  /// Present for existing rows (UPDATE), absent for newly inserted rows (INSERT).
  pub id: Option<Uuid>,
  pub storage_id: Uuid,
  pub produced_amount: Decimal,
}

/// Composite update payload for a blending document.
///
/// Header fields are applied as a partial update (mirrors `UpdateBlendingRequest`).
/// Both `components` and `results` are required and are the full new state of
/// each child collection. They are diff-applied against existing rows using
/// insert / update / delete semantics keyed on the row id, matching the
/// dispatch composite update flow.
#[request_dto]
pub struct UpdateBlendingCompositeRequest {
  /// Header fields applied as a partial update (mirrors per-row UpdateBlendingRequest).
  #[validate(nested)]
  #[serde(flatten)]
  pub blending: UpdateBlendingRequest,
  /// Full new components list, diff-applied against existing rows.
  #[validate(length(min = 1), nested)]
  pub components: Vec<UpdateBlendingComponentCompositeRequest>,
  /// Full new results list, diff-applied against existing rows.
  #[validate(length(min = 1), nested)]
  pub results: Vec<UpdateBlendingResultCompositeRequest>,
}

impl From<&CreateBlendingCompositeRequest> for blending_document::ActiveModelEx {
  fn from(req: &CreateBlendingCompositeRequest) -> Self {
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
      target_product_id: Set(req.target_product_id),
      components: req
        .components
        .iter()
        .map(blending_component::ActiveModelEx::from)
        .collect::<Vec<_>>()
        .into(),
      results: req
        .results
        .iter()
        .map(blending_result::ActiveModelEx::from)
        .collect::<Vec<_>>()
        .into(),
      ..Default::default()
    }
  }
}
