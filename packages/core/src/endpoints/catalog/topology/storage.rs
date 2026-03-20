use super::*;

#[utoipa::path(
  get,
  tag = "Catalog",
  operation_id = "catalog_storage_list",
  summary = "List storages",
  description = "Returns storages used by acceptance, dispatch, transfer, and blending operations.",
  path = paths::catalog::STORAGES,
  responses((status = 200, body = ApiResponse<Vec<StorageResponse>>))
)]
#[axum::debug_handler]
async fn storage_list(State(state): State<Arc<ApiState>>) -> ApiResult<Vec<StorageResponse>> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.storage_list().await?,
  ))
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
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<StorageResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn storage_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<StorageResponse> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.storage_get(id).await?,
  ))
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

pub(super) fn storage_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(storage_list, storage_create))
    .routes(routes!(storage_get, storage_update))
    .routes(routes!(storage_soft_delete))
    .routes(routes!(storage_hard_delete))
    .with_state(state)
}
