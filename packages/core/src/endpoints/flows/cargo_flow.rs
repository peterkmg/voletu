use std::sync::Arc;

use axum::extract::{Query, State};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::response::pipeline::CargoFlowFlatRow,
  endpoints::{paths, query::PaginationParams},
};

#[utoipa::path(
  get,
  tag = "Flows",
  operation_id = "flow_cargo_flow_flat_query",
  summary = "Query all cargo flow documents with items (flat, aggregated)",
  description = "Returns a unified view of ALL document types (acceptance, dispatch, transfers, blending, reconciliation) normalized into a single flat schema. One row per item, sorted by date descending.",
  path = paths::flows::CARGO_FLOW_FLAT_QUERY,
  params(
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
  ),
  responses((status = 200, body = ApiResponse<Vec<CargoFlowFlatRow>>))
)]
#[axum::debug_handler]
async fn cargo_flow_flat_query(
  State(state): State<Arc<ApiState>>,
  Query(params): Query<PaginationParams>,
) -> ApiResult<Vec<CargoFlowFlatRow>> {
  let rows = state
    .svc
    .document
    .cargo_flow_flat_query(params.page, params.per_page)
    .await?;
  Ok(ApiResponse::success(rows))
}

pub fn cargo_flow_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(cargo_flow_flat_query))
    .with_state(state)
}
