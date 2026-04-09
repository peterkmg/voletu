use std::sync::Arc;

use axum::extract::{Query, State};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::response::pipeline::AcceptanceFlatRow,
  endpoints::{paths, query::AcceptanceFlatQueryParams},
  enums::DocumentStatus,
};

#[utoipa::path(
  get,
  tag = "Flows",
  operation_id = "flow_acceptance_flat_query",
  summary = "Query acceptance documents with items (flat)",
  description = "Returns one row per acceptance item with document fields repeated. Used for grouped-row list tables.",
  path = paths::flows::ACCEPTANCE_FLAT_QUERY,
  params(
    ("status" = Option<DocumentStatus>, Query, description = "Filter by document status"),
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
  ),
  responses((status = 200, body = ApiResponse<Vec<AcceptanceFlatRow>>))
)]
#[axum::debug_handler]
async fn acceptance_flat_query(
  State(state): State<Arc<ApiState>>,
  Query(params): Query<AcceptanceFlatQueryParams>,
) -> ApiResult<Vec<AcceptanceFlatRow>> {
  let rows = state
    .svc
    .document
    .acceptance_flat_query(params.into())
    .await?;
  Ok(ApiResponse::success(rows))
}

pub fn acceptance_flat_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(acceptance_flat_query))
    .with_state(state)
}
