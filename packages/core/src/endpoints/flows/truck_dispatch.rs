use std::sync::Arc;

use axum::extract::{Query, State};
use serde::Deserialize;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::response::flow::TruckDispatchFlowRow,
  endpoints::{paths, query::PaginationParams},
  enums::PipelineStatus,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TruckDispatchFlowQueryParams {
  pipeline_status: Option<PipelineStatus>,
  contractor_id: Option<Uuid>,
  #[serde(flatten)]
  pagination: PaginationParams,
}

#[utoipa::path(
  get,
  tag = "Flows",
  operation_id = "flow_truck_dispatch_query",
  summary = "Query truck dispatch flow",
  description = "Returns truck-method dispatch documents with a computed pipeline_status based on document status.",
  path = paths::flows::TRUCK_DISPATCH_QUERY,
  params(
    ("pipelineStatus" = Option<PipelineStatus>, Query, description = "Filter by pipeline status: DRAFT, EXECUTED"),
    ("contractorId" = Option<Uuid>, Query, description = "Filter by contractor UUID"),
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
  ),
  responses((status = 200, body = ApiResponse<Vec<TruckDispatchFlowRow>>))
)]
#[axum::debug_handler]
async fn truck_dispatch_query(
  State(state): State<Arc<ApiState>>,
  Query(params): Query<TruckDispatchFlowQueryParams>,
) -> ApiResult<Vec<TruckDispatchFlowRow>> {
  let rows = state
    .svc
    .flow
    .truck_dispatch_query(
      params.pipeline_status,
      params.contractor_id,
      params.pagination.page,
      params.pagination.per_page,
    )
    .await?;
  Ok(ApiResponse::success(rows))
}

pub fn truck_dispatch_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(truck_dispatch_query))
    .with_state(state)
}
