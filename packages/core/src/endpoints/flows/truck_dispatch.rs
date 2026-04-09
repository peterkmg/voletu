use std::sync::Arc;

use axum::extract::{Query, State};
use serde::Deserialize;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::response::pipeline::TruckDispatchPipelineResponse,
  endpoints::{paths, query::PaginationParams},
  enums::PipelineStatus,
  services::document::query::TruckDispatchPipelineQuerySpec,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TruckDispatchPipelineQueryParams {
  pipeline_status: Option<PipelineStatus>,
  contractor_id: Option<Uuid>,
  #[serde(flatten)]
  pagination: PaginationParams,
}

impl From<TruckDispatchPipelineQueryParams> for TruckDispatchPipelineQuerySpec {
  fn from(params: TruckDispatchPipelineQueryParams) -> Self {
    Self {
      pipeline_status: params.pipeline_status,
      contractor_id: params.contractor_id,
      page: params.pagination.page,
      per_page: params.pagination.per_page,
    }
  }
}

#[utoipa::path(
  get,
  tag = "Flows",
  operation_id = "truck_dispatch_pipeline_query",
  summary = "Query truck dispatch pipeline",
  description = "Returns truck-method dispatch documents with a computed pipeline_status based on document status.",
  path = paths::flows::TRUCK_DISPATCH_QUERY,
  params(
    ("pipelineStatus" = Option<PipelineStatus>, Query, description = "Filter by pipeline status: DRAFT, EXECUTED"),
    ("contractorId" = Option<Uuid>, Query, description = "Filter by contractor UUID"),
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
  ),
  responses((status = 200, body = ApiResponse<Vec<TruckDispatchPipelineResponse>>))
)]
#[axum::debug_handler]
async fn truck_dispatch_pipeline_query(
  State(state): State<Arc<ApiState>>,
  Query(params): Query<TruckDispatchPipelineQueryParams>,
) -> ApiResult<Vec<TruckDispatchPipelineResponse>> {
  let rows = state
    .svc
    .document
    .truck_dispatch_pipeline_query(params.into())
    .await?;
  Ok(ApiResponse::success(rows))
}

pub fn truck_dispatch_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(truck_dispatch_pipeline_query))
    .with_state(state)
}
