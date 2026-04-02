use std::sync::Arc;

use axum::extract::{Query, State};
use serde::Deserialize;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::response::flow::RailReceiptFlowRow,
  endpoints::{paths, query::PaginationParams},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RailReceiptFlowQueryParams {
  pipeline_status: Option<String>,
  contractor_id: Option<Uuid>,
  #[serde(flatten)]
  pagination: PaginationParams,
}

#[utoipa::path(
  get,
  tag = "Flows",
  operation_id = "flow_rail_receipt_query",
  summary = "Query rail receipt flow",
  description = "Returns rail waybills LEFT JOINed with their linked acceptance documents and a computed pipeline_status.",
  path = paths::flows::RAIL_RECEIPT_QUERY,
  params(
    ("pipelineStatus" = Option<String>, Query, description = "Filter by pipeline status: pending, draft, executed"),
    ("contractorId" = Option<Uuid>, Query, description = "Filter by contractor (sender) UUID"),
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
  ),
  responses((status = 200, body = ApiResponse<Vec<RailReceiptFlowRow>>))
)]
#[axum::debug_handler]
async fn rail_receipt_query(
  State(state): State<Arc<ApiState>>,
  Query(params): Query<RailReceiptFlowQueryParams>,
) -> ApiResult<Vec<RailReceiptFlowRow>> {
  let rows = state
    .svc
    .flow
    .rail_receipt_query(
      params.pipeline_status.as_deref(),
      params.contractor_id,
      params.pagination.page,
      params.pagination.per_page,
    )
    .await?;
  Ok(ApiResponse::success(rows))
}

pub fn rail_receipt_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(rail_receipt_query))
    .with_state(state)
}
