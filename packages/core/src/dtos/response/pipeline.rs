use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use voletu_core_macros::response_dto;

use crate::enums::PipelineStatus;

#[response_dto]
pub struct TruckReceiptPipelineResponse {
  pub id: Uuid,
  pub basis_document_number: String,
  pub basis_date: String,
  pub contractor_id: Uuid,
  pub contractor_name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub product_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub expected_quantity: Option<Decimal>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub action_id: Option<Uuid>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub action_document_number: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub actual_quantity: Option<Decimal>,
  pub pipeline_status: PipelineStatus,
}

#[response_dto]
pub struct RailReceiptPipelineResponse {
  pub id: Uuid,
  pub basis_document_number: String,
  pub basis_date: String,
  pub contractor_id: Uuid,
  pub contractor_name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub product_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub expected_quantity: Option<Decimal>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub action_id: Option<Uuid>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub action_document_number: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub actual_quantity: Option<Decimal>,
  pub pipeline_status: PipelineStatus,
}

#[response_dto]
pub struct TruckDispatchPipelineResponse {
  pub id: Uuid,
  pub document_number: String,
  pub date: String,
  pub contractor_id: Uuid,
  pub contractor_name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub product_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub dispatched_quantity: Option<Decimal>,
  pub pipeline_status: PipelineStatus,
}
