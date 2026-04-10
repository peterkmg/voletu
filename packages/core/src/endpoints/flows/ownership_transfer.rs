use std::sync::Arc;

use axum::extract::{Query, State};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::{response::document::OwnershipTransferFlatRow, OwnershipTransferFlatQueryParams},
  endpoints::paths,
  enums::DocumentStatus,
};

#[utoipa::path(
  get,
  tag = "Flows",
  operation_id = "flow_ownership_transfer_flat_query",
  summary = "Query ownership transfers with items (flat)",
  description = "Returns one row per ownership transfer item with document fields repeated. Used for grouped-row list tables.",
  path = paths::flows::OWNERSHIP_TRANSFER_FLAT_QUERY,
  params(
    ("status" = Option<DocumentStatus>, Query, description = "Filter by document status"),
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
  ),
  responses((status = 200, body = ApiResponse<Vec<OwnershipTransferFlatRow>>))
)]
#[axum::debug_handler]
async fn ownership_transfer_flat_query(
  State(state): State<Arc<ApiState>>,
  Query(params): Query<OwnershipTransferFlatQueryParams>,
) -> ApiResult<Vec<OwnershipTransferFlatRow>> {
  let rows = state
    .svc
    .document
    .ownership_transfer_flat_query(params.into())
    .await?;
  Ok(ApiResponse::success(rows))
}

pub fn ownership_transfer_flat_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(ownership_transfer_flat_query))
    .with_state(state)
}
