use std::sync::Arc;

use axum::{
  extract::{Query, State},
  Json,
};
use axum_valid::Valid;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::{
    CreateRailWagonManifestRequest, CreateRailWagonMeasurementRequest,
    CreateRailWagonWeightRequest, CreateRailWaybillRequest, RailWagonManifestResponse,
    RailWagonMeasurementResponse, RailWagonWeightResponse, RailWaybillCompositeRequest,
    RailWaybillCompositeResponse, RailWaybillResponse,
  },
  endpoints::{
    paths,
    query::{apply_entity_query, EntityQueryParams},
  },
};

#[utoipa::path(
  get,
  path = paths::transport::rail::WAYBILLS,
  responses((status = 200, body = ApiResponse<Vec<RailWaybillResponse>>))
)]
#[axum::debug_handler]
async fn rail_waybill_list(
  State(state): State<Arc<ApiState>>,
  Query(query): Query<EntityQueryParams>,
) -> ApiResult<Vec<RailWaybillResponse>> {
  let rows = state.svc.document.rail_waybill_list(None).await?;
  Ok(ApiResponse::success(apply_entity_query(rows, &query)?))
}

#[utoipa::path(
  post,
  path = paths::transport::rail::WAYBILLS,
  request_body = CreateRailWaybillRequest,
  responses((status = 200, body = ApiResponse<RailWaybillResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn rail_waybill_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateRailWaybillRequest>>,
) -> ApiResult<RailWaybillResponse> {
  Ok(ApiResponse::success(
    state.svc.document.rail_waybill_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  path = paths::transport::rail::MANIFESTS,
  responses((status = 200, body = ApiResponse<Vec<RailWagonManifestResponse>>))
)]
#[axum::debug_handler]
async fn rail_manifest_list(
  State(state): State<Arc<ApiState>>,
  Query(query): Query<EntityQueryParams>,
) -> ApiResult<Vec<RailWagonManifestResponse>> {
  let rows = state.svc.document.rail_manifest_list(None).await?;
  Ok(ApiResponse::success(apply_entity_query(rows, &query)?))
}

#[utoipa::path(
  post,
  path = paths::transport::rail::MANIFESTS,
  request_body = CreateRailWagonManifestRequest,
  responses((status = 200, body = ApiResponse<RailWagonManifestResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn rail_manifest_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateRailWagonManifestRequest>>,
) -> ApiResult<RailWagonManifestResponse> {
  Ok(ApiResponse::success(
    state.svc.document.rail_manifest_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  path = paths::transport::rail::MEASUREMENTS,
  responses((status = 200, body = ApiResponse<Vec<RailWagonMeasurementResponse>>))
)]
#[axum::debug_handler]
async fn rail_measurement_list(
  State(state): State<Arc<ApiState>>,
  Query(query): Query<EntityQueryParams>,
) -> ApiResult<Vec<RailWagonMeasurementResponse>> {
  let rows = state.svc.document.rail_measurement_list(None).await?;
  Ok(ApiResponse::success(apply_entity_query(rows, &query)?))
}

#[utoipa::path(
  post,
  path = paths::transport::rail::MEASUREMENTS,
  request_body = CreateRailWagonMeasurementRequest,
  responses((status = 200, body = ApiResponse<RailWagonMeasurementResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn rail_measurement_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateRailWagonMeasurementRequest>>,
) -> ApiResult<RailWagonMeasurementResponse> {
  Ok(ApiResponse::success(
    state.svc.document.rail_measurement_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  path = paths::transport::rail::WEIGHTS,
  responses((status = 200, body = ApiResponse<Vec<RailWagonWeightResponse>>))
)]
#[axum::debug_handler]
async fn rail_weight_list(
  State(state): State<Arc<ApiState>>,
  Query(query): Query<EntityQueryParams>,
) -> ApiResult<Vec<RailWagonWeightResponse>> {
  let rows = state.svc.document.rail_weight_list(None).await?;
  Ok(ApiResponse::success(apply_entity_query(rows, &query)?))
}

#[utoipa::path(
  post,
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
  post,
  path = paths::transport::rail::COMPOSITE_CREATE,
  request_body = RailWaybillCompositeRequest,
  responses((status = 200, body = ApiResponse<RailWaybillCompositeResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn rail_waybill_composite_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<RailWaybillCompositeRequest>>,
) -> ApiResult<RailWaybillCompositeResponse> {
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .rail_waybill_composite_create(&req)
      .await?,
  ))
}

pub fn rail_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(rail_waybill_list, rail_waybill_create))
    .routes(routes!(rail_manifest_list, rail_manifest_create))
    .routes(routes!(rail_measurement_list, rail_measurement_create))
    .routes(routes!(rail_weight_list, rail_weight_create))
    .routes(routes!(rail_waybill_composite_create))
    .with_state(state)
}
