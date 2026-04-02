use std::sync::Arc;

use axum::extract::{Query, State};
use serde::Deserialize;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::response::flow::TruckReceiptFlowRow,
  endpoints::{paths, query::PaginationParams},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TruckReceiptFlowQueryParams {
  pipeline_status: Option<String>,
  contractor_id: Option<Uuid>,
  #[serde(flatten)]
  pagination: PaginationParams,
}

#[utoipa::path(
  get,
  tag = "Flows",
  operation_id = "flow_truck_receipt_query",
  summary = "Query truck receipt flow",
  description = "Returns truck waybills LEFT JOINed with their linked acceptance documents and a computed pipeline_status.",
  path = paths::flows::TRUCK_RECEIPT_QUERY,
  params(
    ("pipelineStatus" = Option<String>, Query, description = "Filter by pipeline status: pending, draft, executed"),
    ("contractorId" = Option<Uuid>, Query, description = "Filter by contractor (sender) UUID"),
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
  ),
  responses((status = 200, body = ApiResponse<Vec<TruckReceiptFlowRow>>))
)]
#[axum::debug_handler]
async fn truck_receipt_query(
  State(state): State<Arc<ApiState>>,
  Query(params): Query<TruckReceiptFlowQueryParams>,
) -> ApiResult<Vec<TruckReceiptFlowRow>> {
  let rows = state
    .svc
    .flow
    .truck_receipt_query(
      params.pipeline_status.as_deref(),
      params.contractor_id,
      params.pagination.page,
      params.pagination.per_page,
    )
    .await?;
  Ok(ApiResponse::success(rows))
}

pub fn truck_receipt_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(truck_receipt_query))
    .with_state(state)
}
