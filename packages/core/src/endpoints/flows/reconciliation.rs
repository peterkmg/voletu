use std::sync::Arc;

use axum::extract::{Query, State};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::{response::document::ReconciliationFlatRow, ReconciliationFlatQueryParams},
  endpoints::paths,
  enums::DocumentStatus,
};

#[utoipa::path(
  get,
  tag = "Flows",
  operation_id = "flow_reconciliation_flat_query",
  summary = "Query reconciliations with adjustments (flat)",
  description = "Returns one row per reconciliation adjustment with document fields repeated. Used for grouped-row list tables.",
  path = paths::flows::RECONCILIATION_FLAT_QUERY,
  params(
    ("status" = Option<DocumentStatus>, Query, description = "Filter by document status"),
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
  ),
  responses((status = 200, body = ApiResponse<Vec<ReconciliationFlatRow>>))
)]
#[axum::debug_handler]
async fn reconciliation_flat_query(
  State(state): State<Arc<ApiState>>,
  Query(params): Query<ReconciliationFlatQueryParams>,
) -> ApiResult<Vec<ReconciliationFlatRow>> {
  let rows = state
    .svc
    .document
    .reconciliation_flat_query(params.into())
    .await?;
  Ok(ApiResponse::success(rows))
}

pub fn reconciliation_flat_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(reconciliation_flat_query))
    .with_state(state)
}
