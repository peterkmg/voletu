use super::*;

#[utoipa::path(
  get,
  tag = "Catalog",
  operation_id = "catalog_warehouse_list",
  summary = "List warehouses",
  description = "Returns warehouses linked to bases.",
  path = paths::catalog::WAREHOUSES,
  responses((status = 200, body = ApiResponse<Vec<WarehouseResponse>>))
)]
#[axum::debug_handler]
async fn warehouse_list(State(state): State<Arc<ApiState>>) -> ApiResult<Vec<WarehouseResponse>> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.warehouse_list().await?,
  ))
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
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<WarehouseResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn warehouse_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<WarehouseResponse> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.warehouse_get(id).await?,
  ))
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

pub(super) fn warehouse_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(warehouse_list, warehouse_create))
    .routes(routes!(warehouse_get, warehouse_update))
    .routes(routes!(warehouse_soft_delete))
    .routes(routes!(warehouse_hard_delete))
    .with_state(state)
}
