use std::sync::Arc;

use axum::extract::{Query, State};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::{response::document::BlendingFlatRow, BlendingFlatQueryParams},
  endpoints::paths,
  enums::DocumentStatus,
};

#[utoipa::path(
  get,
  tag = "Flows",
  operation_id = "flow_blending_flat_query",
  summary = "Query blending documents with items (flat)",
  description = "Returns one row per blending component/result with document fields repeated. Used for grouped-row list tables.",
  path = paths::flows::BLENDING_FLAT_QUERY,
  params(
    ("status" = Option<DocumentStatus>, Query, description = "Filter by document status"),
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
  ),
  responses((status = 200, body = ApiResponse<Vec<BlendingFlatRow>>))
)]
#[axum::debug_handler]
async fn blending_flat_query(
  State(state): State<Arc<ApiState>>,
  Query(params): Query<BlendingFlatQueryParams>,
) -> ApiResult<Vec<BlendingFlatRow>> {
  let rows = state
    .svc
    .document
    .blending_flat_query(params.into())
    .await?;
  Ok(ApiResponse::success(rows))
}

pub fn blending_flat_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(blending_flat_query))
    .with_state(state)
}
