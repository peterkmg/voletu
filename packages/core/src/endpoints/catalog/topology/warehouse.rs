use super::*;

#[utoipa::path(
  get,
  tag = "Catalog",
  operation_id = "catalog_warehouse_list",
  summary = "List warehouses",
  description = "Returns warehouses linked to bases.",
  path = paths::catalog::WAREHOUSES,
  params(
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names"),
    ("page" = Option<u64>, Query, description = "Page number (1-based)"),
    ("per_page" = Option<u64>, Query, description = "Items per page"),
  ),
  responses((status = 200, body = ApiResponse<Vec<WarehouseResponse>>))
)]
#[axum::debug_handler]
async fn warehouse_list(
  State(state): State<Arc<ApiState>>,
  Query(embed): Query<EmbedParams>,
  Query(pagination): Query<PaginationParams>,
) -> ApiResult<Vec<WarehouseResponse>> {
  let pg = if pagination.page.is_some() || pagination.per_page.is_some() {
    Some(crate::services::common::normalize_pagination(
      pagination.page,
      pagination.per_page,
    )?)
  } else {
    None
  };
  let items = if embed.wants_names() {
    state
      .svc
      .catalog_service
      .warehouse_list_with_names(pg)
      .await?
  } else {
    state.svc.catalog_service.warehouse_list(pg).await?
  };
  Ok(ApiResponse::success(items))
}

#[utoipa::path(
  post,
  tag = "Catalog",
  operation_id = "catalog_warehouse_create",
  summary = "Create warehouse",
  description = "Creates a warehouse under an existing base.",
  path = paths::catalog::WAREHOUSES,
  request_body = CreateWarehouseRequest,
  responses((status = 200, body = ApiResponse<WarehouseResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn warehouse_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateWarehouseRequest>>,
) -> ApiResult<WarehouseResponse> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.warehouse_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Catalog",
  operation_id = "catalog_warehouse_get",
  summary = "Get warehouse",
  path = paths::catalog::WAREHOUSES_BY_ID,
  params(("id" = Uuid, Path), ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")),
  responses((status = 200, body = ApiResponse<WarehouseResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn warehouse_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<WarehouseResponse> {
  let item = if embed.wants_names() {
    state
      .svc
      .catalog_service
      .warehouse_get_with_names(id)
      .await?
  } else {
    state.svc.catalog_service.warehouse_get(id).await?
  };
  Ok(ApiResponse::success(item))
}

#[utoipa::path(
  put,
  tag = "Catalog",
  operation_id = "catalog_warehouse_update",
  summary = "Update warehouse",
  path = paths::catalog::WAREHOUSES_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateWarehouseRequest,
  responses((status = 200, body = ApiResponse<WarehouseResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn warehouse_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateWarehouseRequest>>,
) -> ApiResult<WarehouseResponse> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.warehouse_update(id, &req).await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Catalog",
  operation_id = "catalog_warehouse_soft_delete",
  summary = "Soft delete warehouse",
  path = paths::catalog::WAREHOUSES_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn warehouse_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.catalog_service.warehouse_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Catalog",
  operation_id = "catalog_warehouse_hard_delete",
  summary = "Hard delete warehouse",
  path = paths::catalog::WAREHOUSES_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404), (status = 409))
)]
#[axum::debug_handler]
async fn warehouse_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.catalog_service.warehouse_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  post,
  tag = "Catalog",
  operation_id = "catalog_warehouse_restore",
  summary = "Restore soft-deleted warehouse",
  path = paths::catalog::WAREHOUSES_RESTORE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn warehouse_restore(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state
    .svc
    .catalog_service
    .warehouse_soft_delete_undo(id)
    .await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn warehouse_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(warehouse_list, warehouse_create))
    .routes(routes!(warehouse_get, warehouse_update))
    .routes(routes!(warehouse_soft_delete))
    .routes(routes!(warehouse_hard_delete))
    .routes(routes!(warehouse_restore))
    .with_state(state)
}
