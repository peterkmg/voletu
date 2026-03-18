use super::*;

#[utoipa::path(
  get,
  tag = "Catalog",
  operation_id = "catalog_product_type_list",
  summary = "List product types",
  description = "Returns product type references used for catalog validation and storage compatibility checks.",
  path = paths::catalog::PRODUCT_TYPES,
  responses((status = 200, body = ApiResponse<Vec<ProductTypeResponse>>))
)]
#[axum::debug_handler]
async fn product_type_list(
  State(state): State<Arc<ApiState>>,
) -> ApiResult<Vec<ProductTypeResponse>> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.product_type_list().await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Catalog",
  operation_id = "catalog_product_type_create",
  summary = "Create product type",
  description = "Creates a new product type reference.",
  path = paths::catalog::PRODUCT_TYPES,
  request_body = CreateProductTypeRequest,
  responses((status = 200, body = ApiResponse<ProductTypeResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn product_type_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateProductTypeRequest>>,
) -> ApiResult<ProductTypeResponse> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.product_type_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Catalog",
  operation_id = "catalog_product_type_get",
  summary = "Get product type",
  path = paths::catalog::PRODUCT_TYPES_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<ProductTypeResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn product_type_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<ProductTypeResponse> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.product_type_get(id).await?,
  ))
}

#[utoipa::path(
  put,
  tag = "Catalog",
  operation_id = "catalog_product_type_update",
  summary = "Update product type",
  path = paths::catalog::PRODUCT_TYPES_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateProductTypeRequest,
  responses((status = 200, body = ApiResponse<ProductTypeResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn product_type_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateProductTypeRequest>>,
) -> ApiResult<ProductTypeResponse> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.product_type_update(id, &req).await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Catalog",
  operation_id = "catalog_product_type_soft_delete",
  summary = "Soft delete product type",
  path = paths::catalog::PRODUCT_TYPES_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn product_type_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.catalog_service.product_type_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Catalog",
  operation_id = "catalog_product_type_hard_delete",
  summary = "Hard delete product type",
  path = paths::catalog::PRODUCT_TYPES_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404), (status = 409))
)]
#[axum::debug_handler]
async fn product_type_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.catalog_service.product_type_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn product_type_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(product_type_list, product_type_create))
    .routes(routes!(product_type_get, product_type_update))
    .routes(routes!(product_type_soft_delete))
    .routes(routes!(product_type_hard_delete))
    .with_state(state)
}
