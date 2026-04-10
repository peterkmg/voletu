use std::sync::Arc;

use axum::extract::{Query, State};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::{response::flow::RailReceiptPipelineResponse, RailReceiptPipelineQueryParams},
  endpoints::paths,
  enums::PipelineStatus,
};

#[utoipa::path(
  get,
  tag = "Flows",
  operation_id = "rail_receipt_pipeline_query",
  summary = "Query rail receipt pipeline",
  description = "Returns rail waybills with their linked acceptance documents and a computed pipeline_status.",
  path = paths::flows::RAIL_RECEIPT_QUERY,
  params(
    ("pipelineStatus" = Option<PipelineStatus>, Query, description = "Filter by pipeline status: PENDING, DRAFT, EXECUTED"),
    ("contractorId" = Option<Uuid>, Query, description = "Filter by contractor (sender) UUID"),
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
  ),
  responses((status = 200, body = ApiResponse<Vec<RailReceiptPipelineResponse>>))
)]
#[axum::debug_handler]
async fn rail_receipt_pipeline_query(
  State(state): State<Arc<ApiState>>,
  Query(params): Query<RailReceiptPipelineQueryParams>,
) -> ApiResult<Vec<RailReceiptPipelineResponse>> {
  let rows = state
    .svc
    .document
    .rail_receipt_pipeline_query(params.into())
    .await?;
  Ok(ApiResponse::success(rows))
}

pub fn rail_receipt_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(rail_receipt_pipeline_query))
    .with_state(state)
}
