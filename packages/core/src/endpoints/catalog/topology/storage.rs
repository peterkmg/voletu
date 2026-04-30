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
  dtos::{
    CreateStorageRequest,
    EmbedParams,
    PaginationParams,
    StorageResponse,
    UpdateStorageRequest,
  },
  endpoints::paths,
  services::common::normalize_pagination,
};

#[utoipa::path(
  get,
  tag = "Catalog",
  operation_id = "catalog_storage_list",
  summary = "List storages",
  description = "Returns storages used by acceptance, dispatch, transfer, and blending operations.",
  path = paths::catalog::STORAGES,
  params(
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names"),
    ("page" = Option<u64>, Query, description = "Page number (1-based)"),
    ("per_page" = Option<u64>, Query, description = "Items per page"),
  ),
  responses((status = 200, body = ApiResponse<Vec<StorageResponse>>))
)]
#[axum::debug_handler]
async fn storage_list(
  State(state): State<Arc<ApiState>>,
  Query(embed): Query<EmbedParams>,
  Query(pagination): Query<PaginationParams>,
) -> ApiResult<Vec<StorageResponse>> {
  let pg = if pagination.page.is_some() || pagination.per_page.is_some() {
    Some(normalize_pagination(pagination.page, pagination.per_page)?)
  } else {
    None
  };
  let items = if embed.wants_names() {
    state
      .svc
      .catalog_service
      .storage_list_with_names(pg)
      .await?
  } else {
    state.svc.catalog_service.storage_list(pg).await?
  };
  Ok(ApiResponse::success(items))
}

#[utoipa::path(
  post,
  tag = "Catalog",
  operation_id = "catalog_storage_create",
  summary = "Create storage",
  description = "Creates a storage row with optional capacity or type constraints.",
  path = paths::catalog::STORAGES,
  request_body = CreateStorageRequest,
  responses((status = 200, body = ApiResponse<StorageResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn storage_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateStorageRequest>>,
) -> ApiResult<StorageResponse> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.storage_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Catalog",
  operation_id = "catalog_storage_get",
  summary = "Get storage",
  path = paths::catalog::STORAGES_BY_ID,
  params(("id" = Uuid, Path), ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")),
  responses((status = 200, body = ApiResponse<StorageResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn storage_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<StorageResponse> {
  let item = if embed.wants_names() {
    state.svc.catalog_service.storage_get_with_names(id).await?
  } else {
    state.svc.catalog_service.storage_get(id).await?
  };
  Ok(ApiResponse::success(item))
}

#[utoipa::path(
  put,
  tag = "Catalog",
  operation_id = "catalog_storage_update",
  summary = "Update storage",
  path = paths::catalog::STORAGES_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateStorageRequest,
  responses((status = 200, body = ApiResponse<StorageResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn storage_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateStorageRequest>>,
) -> ApiResult<StorageResponse> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.storage_update(id, &req).await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Catalog",
  operation_id = "catalog_storage_soft_delete",
  summary = "Soft delete storage",
  path = paths::catalog::STORAGES_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn storage_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.catalog_service.storage_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Catalog",
  operation_id = "catalog_storage_hard_delete",
  summary = "Hard delete storage",
  path = paths::catalog::STORAGES_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404), (status = 409))
)]
#[axum::debug_handler]
async fn storage_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.catalog_service.storage_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  post,
  tag = "Catalog",
  operation_id = "catalog_storage_restore",
  summary = "Restore soft-deleted storage",
  path = paths::catalog::STORAGES_RESTORE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn storage_restore(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state
    .svc
    .catalog_service
    .storage_soft_delete_undo(id)
    .await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn storage_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(storage_list, storage_create))
    .routes(routes!(storage_get, storage_update))
    .routes(routes!(storage_soft_delete))
    .routes(routes!(storage_hard_delete))
    .routes(routes!(storage_restore))
    .with_state(state)
}
