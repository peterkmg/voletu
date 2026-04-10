use std::sync::Arc;

use axum::extract::{Query, State};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::{response::document::PhysicalTransferFlatRow, PhysicalTransferFlatQueryParams},
  endpoints::paths,
  enums::DocumentStatus,
};

#[utoipa::path(
  get,
  tag = "Flows",
  operation_id = "flow_physical_transfer_flat_query",
  summary = "Query physical transfer documents with items (flat)",
  description = "Returns one row per physical transfer item with document fields repeated. Used for grouped-row list tables.",
  path = paths::flows::PHYSICAL_TRANSFER_FLAT_QUERY,
  params(
    ("status" = Option<DocumentStatus>, Query, description = "Filter by document status"),
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
  ),
  responses((status = 200, body = ApiResponse<Vec<PhysicalTransferFlatRow>>))
)]
#[axum::debug_handler]
async fn physical_transfer_flat_query(
  State(state): State<Arc<ApiState>>,
  Query(params): Query<PhysicalTransferFlatQueryParams>,
) -> ApiResult<Vec<PhysicalTransferFlatRow>> {
  let rows = state
    .svc
    .document
    .physical_transfer_flat_query(params.into())
    .await?;
  Ok(ApiResponse::success(rows))
}

pub fn physical_transfer_flat_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(physical_transfer_flat_query))
    .with_state(state)
}
