use std::sync::Arc;

use axum::{
  extract::{Path, State},
  Json,
};
use axum_valid::Valid;
use uuid::Uuid;

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::{AcceptanceItemResponse, CreateAcceptanceItemRequest},
  endpoints::paths,
};

#[utoipa::path(
  get,
  tag = "Document - Acceptance",
  operation_id = "acceptance_item_list",
  summary = "List acceptance items",
  path = paths::acceptance::ITEMS,
  responses((status = 200, body = ApiResponse<Vec<AcceptanceItemResponse>>))
)]
#[axum::debug_handler]
pub(super) async fn acceptance_item_list(
  State(state): State<Arc<ApiState>>,
) -> ApiResult<Vec<AcceptanceItemResponse>> {
  Ok(ApiResponse::success(
    state.svc.document.acceptance_item_list(None).await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Document - Acceptance",
  operation_id = "acceptance_item_create",
  summary = "Create acceptance item",
  path = paths::acceptance::ITEMS,
  request_body = CreateAcceptanceItemRequest,
  responses((status = 200, body = ApiResponse<AcceptanceItemResponse>), (status = 400))
)]
#[axum::debug_handler]
pub(super) async fn acceptance_item_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateAcceptanceItemRequest>>,
) -> ApiResult<AcceptanceItemResponse> {
  Ok(ApiResponse::success(
    state.svc.document.acceptance_item_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Document - Acceptance",
  operation_id = "acceptance_item_get",
  summary = "Get acceptance item",
  path = paths::acceptance::ITEMS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<AcceptanceItemResponse>), (status = 404))
)]
#[axum::debug_handler]
pub(super) async fn acceptance_item_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<AcceptanceItemResponse> {
  Ok(ApiResponse::success(
    state.svc.document.acceptance_item_get(id).await?,
  ))
}

#[utoipa::path(
  put,
  tag = "Document - Acceptance",
  operation_id = "acceptance_item_update",
  summary = "Update acceptance item",
  path = paths::acceptance::ITEMS_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = crate::dtos::UpdateAcceptanceItemRequest,
  responses((status = 200, body = ApiResponse<AcceptanceItemResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
pub(super) async fn acceptance_item_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<crate::dtos::UpdateAcceptanceItemRequest>>,
) -> ApiResult<AcceptanceItemResponse> {
  Ok(ApiResponse::success(
    state.svc.document.acceptance_item_update(id, &req).await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Document - Acceptance",
  operation_id = "acceptance_item_soft_delete",
  summary = "Soft delete acceptance item",
  path = paths::acceptance::ITEMS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
pub(super) async fn acceptance_item_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.acceptance_item_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Document - Acceptance",
  operation_id = "acceptance_item_hard_delete",
  summary = "Hard delete acceptance item",
  path = paths::acceptance::ITEMS_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
pub(super) async fn acceptance_item_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.acceptance_item_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}
