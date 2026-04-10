use std::sync::Arc;

use axum::extract::{Query, State};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::{response::document::DispatchFlatRow, DispatchFlatQueryParams},
  endpoints::paths,
  enums::{DispatchMethod, DispatchPurpose, DocumentStatus},
};

#[utoipa::path(
  get,
  tag = "Flows",
  operation_id = "flow_dispatch_flat_query",
  summary = "Query dispatch documents with items (flat)",
  description = "Returns one row per dispatch item with document fields repeated. Used for grouped-row list tables.",
  path = paths::flows::DISPATCH_FLAT_QUERY,
  params(
    ("status" = Option<DocumentStatus>, Query, description = "Filter by document status"),
    ("dispatchMethod" = Option<DispatchMethod>, Query, description = "Filter by dispatch method"),
    ("dispatchPurpose" = Option<DispatchPurpose>, Query, description = "Filter by dispatch purpose"),
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
  ),
  responses((status = 200, body = ApiResponse<Vec<DispatchFlatRow>>))
)]
#[axum::debug_handler]
async fn dispatch_flat_query(
  State(state): State<Arc<ApiState>>,
  Query(params): Query<DispatchFlatQueryParams>,
) -> ApiResult<Vec<DispatchFlatRow>> {
  let rows = state
    .svc
    .document
    .dispatch_flat_query(params.into())
    .await?;
  Ok(ApiResponse::success(rows))
}

pub fn dispatch_flat_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(dispatch_flat_query))
    .with_state(state)
}
