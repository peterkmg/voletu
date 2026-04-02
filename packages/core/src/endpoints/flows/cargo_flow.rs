use std::sync::Arc;

use axum::extract::{Query, State};
use serde::Deserialize;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::response::flow::CargoFlowRow,
  endpoints::{paths, query::PaginationParams},
  enums::{FlowType, PipelineStatus},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CargoFlowQueryParams {
  pub flow_type: Option<FlowType>,
  pub operation: Option<String>,
  pub status: Option<PipelineStatus>,
  pub contractor_id: Option<Uuid>,
  #[serde(flatten)]
  pub pagination: PaginationParams,
}

#[utoipa::path(
  get,
  tag = "Flows",
  operation_id = "cargo_flow_query",
  summary = "Query all cargo flow documents (aggregate view)",
  path = paths::flows::CARGO_FLOW_QUERY,
  params(
    ("flowType" = Option<FlowType>, Query, description = "INCOMING, OUTGOING, or INTERNAL"),
    ("operation" = Option<String>, Query, description = "Truck Receipt, Bunkering, etc."),
    ("status" = Option<PipelineStatus>, Query, description = "PENDING, DRAFT, or EXECUTED"),
    ("contractorId" = Option<Uuid>, Query),
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
  ),
  responses(
    (status = 200, body = ApiResponse<Vec<CargoFlowRow>>),
    (status = 400),
  )
)]
#[axum::debug_handler]
pub(super) async fn cargo_flow_query(
  State(state): State<Arc<ApiState>>,
  Query(params): Query<CargoFlowQueryParams>,
) -> ApiResult<Vec<CargoFlowRow>> {
  let rows = state
    .svc
    .flow
    .cargo_flow_query(
      params.flow_type,
      params.operation.as_deref(),
      params.status,
      params.contractor_id,
      params.pagination.page,
      params.pagination.per_page,
    )
    .await?;
  Ok(ApiResponse::success(rows))
}

pub fn cargo_flow_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(cargo_flow_query))
    .with_state(state)
}
