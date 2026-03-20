use super::*;

#[utoipa::path(
  get,
  tag = "Catalog",
  operation_id = "catalog_product_list",
  summary = "List products",
  description = "Returns product references used in transport and operations documents.",
  path = paths::catalog::PRODUCTS,
  responses((status = 200, body = ApiResponse<Vec<ProductResponse>>))
)]
#[axum::debug_handler]
async fn product_list(State(state): State<Arc<ApiState>>) -> ApiResult<Vec<ProductResponse>> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.product_list().await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Catalog",
  operation_id = "catalog_product_create",
  summary = "Create product",
  description = "Creates a product row linked to an existing product group and optional manufacturer.",
  path = paths::catalog::PRODUCTS,
  request_body = CreateProductRequest,
  responses((status = 200, body = ApiResponse<ProductResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn product_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateProductRequest>>,
) -> ApiResult<ProductResponse> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.product_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Catalog",
  operation_id = "catalog_product_get",
  summary = "Get product",
  path = paths::catalog::PRODUCTS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<ProductResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn product_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<ProductResponse> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.product_get(id).await?,
  ))
}

#[utoipa::path(
  put,
  tag = "Catalog",
  operation_id = "catalog_product_update",
  summary = "Update product",
  path = paths::catalog::PRODUCTS_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateProductRequest,
  responses((status = 200, body = ApiResponse<ProductResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn product_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateProductRequest>>,
) -> ApiResult<ProductResponse> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.product_update(id, &req).await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Catalog",
  operation_id = "catalog_product_soft_delete",
  summary = "Soft delete product",
  path = paths::catalog::PRODUCTS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn product_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.catalog_service.product_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Catalog",
  operation_id = "catalog_product_hard_delete",
  summary = "Hard delete product",
  path = paths::catalog::PRODUCTS_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404), (status = 409))
)]
#[axum::debug_handler]
async fn product_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.catalog_service.product_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn product_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(product_list, product_create))
    .routes(routes!(product_get, product_update))
    .routes(routes!(product_soft_delete))
    .routes(routes!(product_hard_delete))
    .with_state(state)
}
