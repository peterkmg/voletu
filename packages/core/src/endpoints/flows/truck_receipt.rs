use std::sync::Arc;

use axum::extract::{Query, State};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::response::pipeline::TruckReceiptPipelineResponse,
  endpoints::{paths, query::TruckReceiptPipelineQueryParams},
  enums::PipelineStatus,
};

#[utoipa::path(
  get,
  tag = "Flows",
  operation_id = "flow_truck_receipt_query",
  summary = "Query truck receipt flow",
  description = "Returns truck waybills LEFT JOINed with their linked acceptance documents and a computed pipeline_status.",
  path = paths::flows::TRUCK_RECEIPT_QUERY,
  params(
    ("pipelineStatus" = Option<PipelineStatus>, Query, description = "Filter by pipeline status: PENDING, DRAFT, EXECUTED"),
    ("contractorId" = Option<Uuid>, Query, description = "Filter by contractor (sender) UUID"),
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
  ),
  responses((status = 200, body = ApiResponse<Vec<TruckReceiptPipelineResponse>>))
)]
#[axum::debug_handler]
async fn truck_receipt_query(
  State(state): State<Arc<ApiState>>,
  Query(params): Query<TruckReceiptPipelineQueryParams>,
) -> ApiResult<Vec<TruckReceiptPipelineResponse>> {
  let rows = state
    .svc
    .document
    .truck_receipt_pipeline_query(params.into())
    .await?;
  Ok(ApiResponse::success(rows))
}

pub fn truck_receipt_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(truck_receipt_query))
    .with_state(state)
}
