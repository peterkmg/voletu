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
  dtos::{
    CreateDispatchMeasurementRequest,
    DispatchMeasurementResponse,
    UpdateDispatchMeasurementRequest,
  },
  endpoints::paths,
};

#[utoipa::path(
  get,
  tag = "Document - Dispatch",
  operation_id = "dispatch_storage_measurement_list",
  summary = "List dispatch storage measurements",
  description = "Returns storage measurement rows associated with dispatch documents.",
  path = paths::dispatch::STORAGE_MEASUREMENTS,
  responses((status = 200, body = ApiResponse<Vec<DispatchMeasurementResponse>>))
)]
#[axum::debug_handler]
async fn dispatch_storage_measurement_list(
  State(state): State<Arc<ApiState>>,
) -> ApiResult<Vec<DispatchMeasurementResponse>> {
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .dispatch_storage_measurement_list(None)
      .await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Document - Dispatch",
  operation_id = "dispatch_storage_measurement_create",
  summary = "Create dispatch storage measurement",
  description = "Creates a storage measurement entry under an existing dispatch draft document.",
  path = paths::dispatch::STORAGE_MEASUREMENTS,
  request_body = CreateDispatchMeasurementRequest,
  responses((status = 200, body = ApiResponse<DispatchMeasurementResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn dispatch_storage_measurement_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateDispatchMeasurementRequest>>,
) -> ApiResult<DispatchMeasurementResponse> {
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .dispatch_storage_measurement_create(&req)
      .await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Document - Dispatch",
  operation_id = "dispatch_storage_measurement_get",
  summary = "Get dispatch storage measurement",
  path = paths::dispatch::STORAGE_MEASUREMENTS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<DispatchMeasurementResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn dispatch_storage_measurement_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<DispatchMeasurementResponse> {
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .dispatch_storage_measurement_get(id)
      .await?,
  ))
}

#[utoipa::path(
  put,
  tag = "Document - Dispatch",
  operation_id = "dispatch_storage_measurement_update",
  summary = "Update dispatch storage measurement",
  path = paths::dispatch::STORAGE_MEASUREMENTS_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateDispatchMeasurementRequest,
  responses((status = 200, body = ApiResponse<DispatchMeasurementResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn dispatch_storage_measurement_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateDispatchMeasurementRequest>>,
) -> ApiResult<DispatchMeasurementResponse> {
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .dispatch_storage_measurement_update(id, &req)
      .await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Document - Dispatch",
  operation_id = "dispatch_storage_measurement_soft_delete",
  summary = "Soft delete dispatch storage measurement",
  path = paths::dispatch::STORAGE_MEASUREMENTS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn dispatch_storage_measurement_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state
    .svc
    .document
    .dispatch_storage_measurement_soft_delete(id)
    .await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Document - Dispatch",
  operation_id = "dispatch_storage_measurement_hard_delete",
  summary = "Hard delete dispatch storage measurement",
  path = paths::dispatch::STORAGE_MEASUREMENTS_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn dispatch_storage_measurement_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state
    .svc
    .document
    .dispatch_storage_measurement_hard_delete(id)
    .await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn measurement_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(
      dispatch_storage_measurement_list,
      dispatch_storage_measurement_create
    ))
    .routes(routes!(
      dispatch_storage_measurement_get,
      dispatch_storage_measurement_update
    ))
    .routes(routes!(dispatch_storage_measurement_soft_delete))
    .routes(routes!(dispatch_storage_measurement_hard_delete))
    .with_state(state)
}
