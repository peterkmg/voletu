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
    CreateTruckWaybillItemRequest, CreateTruckWaybillRequest, CreateTruckWeightDocRequest,
    TruckWaybillCompositeRequest, TruckWaybillCompositeResponse, TruckWaybillItemResponse,
    TruckWaybillResponse, TruckWeightDocResponse,
  },
  endpoints::{
    paths,
    query::{apply_entity_query, EntityQueryParams},
  },
};

#[utoipa::path(
  get,
  path = paths::transport::truck::WAYBILLS,
  responses((status = 200, body = ApiResponse<Vec<TruckWaybillResponse>>))
)]
#[axum::debug_handler]
async fn truck_waybill_list(
  State(state): State<Arc<ApiState>>,
  Query(query): Query<EntityQueryParams>,
) -> ApiResult<Vec<TruckWaybillResponse>> {
  let rows = state.svc.document.truck_waybill_list().await?;
  Ok(ApiResponse::success(apply_entity_query(rows, &query)?))
}

#[utoipa::path(
  post,
  path = paths::transport::truck::WAYBILLS,
  request_body = CreateTruckWaybillRequest,
  responses((status = 200, body = ApiResponse<TruckWaybillResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn truck_waybill_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateTruckWaybillRequest>>,
) -> ApiResult<TruckWaybillResponse> {
  Ok(ApiResponse::success(
    state.svc.document.truck_waybill_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  path = paths::transport::truck::ITEMS,
  responses((status = 200, body = ApiResponse<Vec<TruckWaybillItemResponse>>))
)]
#[axum::debug_handler]
async fn truck_waybill_item_list(
  State(state): State<Arc<ApiState>>,
  Query(query): Query<EntityQueryParams>,
) -> ApiResult<Vec<TruckWaybillItemResponse>> {
  let rows = state.svc.document.truck_waybill_item_list().await?;
  Ok(ApiResponse::success(apply_entity_query(rows, &query)?))
}

#[utoipa::path(
  post,
  path = paths::transport::truck::ITEMS,
  request_body = CreateTruckWaybillItemRequest,
  responses((status = 200, body = ApiResponse<TruckWaybillItemResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn truck_waybill_item_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateTruckWaybillItemRequest>>,
) -> ApiResult<TruckWaybillItemResponse> {
  Ok(ApiResponse::success(
    state.svc.document.truck_waybill_item_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  path = paths::transport::truck::WEIGHT_DOCS,
  responses((status = 200, body = ApiResponse<Vec<TruckWeightDocResponse>>))
)]
#[axum::debug_handler]
async fn truck_weight_doc_list(
  State(state): State<Arc<ApiState>>,
  Query(query): Query<EntityQueryParams>,
) -> ApiResult<Vec<TruckWeightDocResponse>> {
  let rows = state.svc.document.truck_weight_doc_list().await?;
  Ok(ApiResponse::success(apply_entity_query(rows, &query)?))
}

#[utoipa::path(
  post,
  path = paths::transport::truck::WEIGHT_DOCS,
  request_body = CreateTruckWeightDocRequest,
  responses((status = 200, body = ApiResponse<TruckWeightDocResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn truck_weight_doc_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateTruckWeightDocRequest>>,
) -> ApiResult<TruckWeightDocResponse> {
  Ok(ApiResponse::success(
    state.svc.document.truck_weight_doc_create(&req).await?,
  ))
}

#[utoipa::path(
  post,
  path = paths::transport::truck::COMPOSITE_CREATE,
  request_body = TruckWaybillCompositeRequest,
  responses((status = 200, body = ApiResponse<TruckWaybillCompositeResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn truck_waybill_composite_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<TruckWaybillCompositeRequest>>,
) -> ApiResult<TruckWaybillCompositeResponse> {
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .truck_waybill_composite_create(&req)
      .await?,
  ))
}

pub fn truck_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(truck_waybill_list, truck_waybill_create))
    .routes(routes!(truck_waybill_item_list, truck_waybill_item_create))
    .routes(routes!(truck_weight_doc_list, truck_weight_doc_create))
    .routes(routes!(truck_waybill_composite_create))
    .with_state(state)
}
