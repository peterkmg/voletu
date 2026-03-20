use super::*;

#[utoipa::path(
  get,
  tag = "Catalog",
  operation_id = "catalog_port_list",
  summary = "List ports",
  description = "Returns port references used by dispatch and export-oriented flows.",
  path = paths::catalog::PORTS,
  params(
    ("page" = Option<u64>, Query, description = "Page number (1-based)"),
    ("per_page" = Option<u64>, Query, description = "Items per page"),
  ),
  responses((status = 200, body = ApiResponse<Vec<PortResponse>>))
)]
#[axum::debug_handler]
async fn port_list(
  State(state): State<Arc<ApiState>>,
  Query(pagination): Query<PaginationParams>,
) -> ApiResult<Vec<PortResponse>> {
  let pg = if pagination.page.is_some() || pagination.per_page.is_some() {
    Some(crate::services::common::normalize_pagination(pagination.page, pagination.per_page)?)
  } else {
    None
  };
  Ok(ApiResponse::success(
    state.svc.catalog_service.port_list(pg).await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Catalog",
  operation_id = "catalog_port_create",
  summary = "Create port",
  description = "Creates a maritime port reference.",
  path = paths::catalog::PORTS,
  request_body = CreatePortRequest,
  responses((status = 200, body = ApiResponse<PortResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn port_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreatePortRequest>>,
) -> ApiResult<PortResponse> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.port_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Catalog",
  operation_id = "catalog_port_get",
  summary = "Get port",
  path = paths::catalog::PORTS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<PortResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn port_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<PortResponse> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.port_get(id).await?,
  ))
}

#[utoipa::path(
  put,
  tag = "Catalog",
  operation_id = "catalog_port_update",
  summary = "Update port",
  path = paths::catalog::PORTS_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdatePortRequest,
  responses((status = 200, body = ApiResponse<PortResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn port_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdatePortRequest>>,
) -> ApiResult<PortResponse> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.port_update(id, &req).await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Catalog",
  operation_id = "catalog_port_soft_delete",
  summary = "Soft delete port",
  path = paths::catalog::PORTS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn port_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.catalog_service.port_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Catalog",
  operation_id = "catalog_port_hard_delete",
  summary = "Hard delete port",
  path = paths::catalog::PORTS_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404), (status = 409))
)]
#[axum::debug_handler]
async fn port_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.catalog_service.port_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  post,
  tag = "Catalog",
  operation_id = "catalog_port_restore",
  summary = "Restore soft-deleted port",
  path = paths::catalog::PORTS_RESTORE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn port_restore(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.catalog_service.port_soft_delete_undo(id).await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn port_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(port_list, port_create))
    .routes(routes!(port_get, port_update))
    .routes(routes!(port_soft_delete))
    .routes(routes!(port_hard_delete))
    .routes(routes!(port_restore))
    .with_state(state)
}
