use std::sync::Arc;

use axum::{
  extract::{Path, Query, State},
  Json,
};
use axum_valid::Valid;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::{BaseResponse, CreateBaseRequest, PaginationParams, UpdateBaseRequest},
  endpoints::paths,
  services::common::normalize_pagination,
};

#[utoipa::path(
  get,
  tag = "Catalog",
  operation_id = "catalog_base_list",
  summary = "List bases",
  description = "Returns operational bases used by topology and sync targeting logic.",
  path = paths::catalog::BASES,
  params(
    ("page" = Option<u64>, Query, description = "Page number (1-based)"),
    ("per_page" = Option<u64>, Query, description = "Items per page"),
  ),
  responses((status = 200, body = ApiResponse<Vec<BaseResponse>>))
)]
#[axum::debug_handler]
async fn base_list(
  State(state): State<Arc<ApiState>>,
  Query(pagination): Query<PaginationParams>,
) -> ApiResult<Vec<BaseResponse>> {
  let pg = if pagination.page.is_some() || pagination.per_page.is_some() {
    Some(normalize_pagination(pagination.page, pagination.per_page)?)
  } else {
    None
  };
  Ok(ApiResponse::success(
    state.svc.catalog_service.base_list(pg).await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Catalog",
  operation_id = "catalog_base_create",
  summary = "Create base",
  description = "Creates a base row for operational topology.",
  path = paths::catalog::BASES,
  request_body = CreateBaseRequest,
  responses((status = 200, body = ApiResponse<BaseResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn base_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateBaseRequest>>,
) -> ApiResult<BaseResponse> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.base_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Catalog",
  operation_id = "catalog_base_get",
  summary = "Get base",
  path = paths::catalog::BASES_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<BaseResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn base_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<BaseResponse> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.base_get(id).await?,
  ))
}

#[utoipa::path(
  put,
  tag = "Catalog",
  operation_id = "catalog_base_update",
  summary = "Update base",
  path = paths::catalog::BASES_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateBaseRequest,
  responses((status = 200, body = ApiResponse<BaseResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn base_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateBaseRequest>>,
) -> ApiResult<BaseResponse> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.base_update(id, &req).await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Catalog",
  operation_id = "catalog_base_soft_delete",
  summary = "Soft delete base",
  path = paths::catalog::BASES_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn base_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.catalog_service.base_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Catalog",
  operation_id = "catalog_base_hard_delete",
  summary = "Hard delete base",
  path = paths::catalog::BASES_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404), (status = 409))
)]
#[axum::debug_handler]
async fn base_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.catalog_service.base_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  post,
  tag = "Catalog",
  operation_id = "catalog_base_restore",
  summary = "Restore soft-deleted base",
  path = paths::catalog::BASES_RESTORE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn base_restore(State(state): State<Arc<ApiState>>, Path(id): Path<Uuid>) -> ApiResult<()> {
  state.svc.catalog_service.base_soft_delete_undo(id).await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn base_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(base_list, base_create))
    .routes(routes!(base_get, base_update))
    .routes(routes!(base_soft_delete))
    .routes(routes!(base_hard_delete))
    .routes(routes!(base_restore))
    .with_state(state)
}
