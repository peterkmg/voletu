use std::sync::Arc;

use axum::{
  extract::{Path, State},
  Json,
};
use axum_valid::Valid;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::{CreateRailWagonWeightRequest, RailWagonWeightResponse, UpdateRailWagonWeightRequest},
  endpoints::paths,
};

#[utoipa::path(
  get,
  tag = "Document - Transport",
  operation_id = "transport_rail_weight_list",
  summary = "List rail weights",
  description = "Returns rail wagon weight rows.",
  path = paths::transport::rail::WEIGHTS,
  responses((status = 200, body = ApiResponse<Vec<RailWagonWeightResponse>>))
)]
#[axum::debug_handler]
async fn rail_weight_list(
  State(state): State<Arc<ApiState>>,
) -> ApiResult<Vec<RailWagonWeightResponse>> {
  Ok(ApiResponse::success(
    state.svc.document.rail_weight_list(None).await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Document - Transport",
  operation_id = "transport_rail_weight_create",
  summary = "Create rail weight",
  description = "Creates a wagon weight row under an existing wagon manifest.",
  path = paths::transport::rail::WEIGHTS,
  request_body = CreateRailWagonWeightRequest,
  responses((status = 200, body = ApiResponse<RailWagonWeightResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn rail_weight_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateRailWagonWeightRequest>>,
) -> ApiResult<RailWagonWeightResponse> {
  Ok(ApiResponse::success(
    state.svc.document.rail_weight_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Document - Transport",
  operation_id = "transport_rail_weight_get",
  summary = "Get rail weight",
  path = paths::transport::rail::WEIGHTS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<RailWagonWeightResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn rail_weight_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<RailWagonWeightResponse> {
  Ok(ApiResponse::success(
    state.svc.document.rail_weight_get(id).await?,
  ))
}

#[utoipa::path(
  put,
  tag = "Document - Transport",
  operation_id = "transport_rail_weight_update",
  summary = "Update rail weight",
  path = paths::transport::rail::WEIGHTS_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateRailWagonWeightRequest,
  responses((status = 200, body = ApiResponse<RailWagonWeightResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn rail_weight_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateRailWagonWeightRequest>>,
) -> ApiResult<RailWagonWeightResponse> {
  Ok(ApiResponse::success(
    state.svc.document.rail_weight_update(id, &req).await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Document - Transport",
  operation_id = "transport_rail_weight_soft_delete",
  summary = "Soft delete rail weight",
  path = paths::transport::rail::WEIGHTS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn rail_weight_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.rail_weight_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Document - Transport",
  operation_id = "transport_rail_weight_hard_delete",
  summary = "Hard delete rail weight",
  path = paths::transport::rail::WEIGHTS_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn rail_weight_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.rail_weight_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn weight_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(rail_weight_list, rail_weight_create))
    .routes(routes!(rail_weight_get, rail_weight_update))
    .routes(routes!(rail_weight_soft_delete))
    .routes(routes!(rail_weight_hard_delete))
    .with_state(state)
}
