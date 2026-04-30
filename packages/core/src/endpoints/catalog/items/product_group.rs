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
    CreateProductGroupRequest,
    EmbedParams,
    PaginationParams,
    ProductGroupResponse,
    UpdateProductGroupRequest,
  },
  endpoints::paths,
  services::common::normalize_pagination,
};

#[utoipa::path(
  get,
  tag = "Catalog",
  operation_id = "catalog_product_group_list",
  summary = "List product groups",
  description = "Returns product groups linked to product types.",
  path = paths::catalog::PRODUCT_GROUPS,
  params(
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names"),
    ("page" = Option<u64>, Query, description = "Page number (1-based)"),
    ("per_page" = Option<u64>, Query, description = "Items per page"),
  ),
  responses((status = 200, body = ApiResponse<Vec<ProductGroupResponse>>))
)]
#[axum::debug_handler]
async fn product_group_list(
  State(state): State<Arc<ApiState>>,
  Query(embed): Query<EmbedParams>,
  Query(pagination): Query<PaginationParams>,
) -> ApiResult<Vec<ProductGroupResponse>> {
  let pg = if pagination.page.is_some() || pagination.per_page.is_some() {
    Some(normalize_pagination(pagination.page, pagination.per_page)?)
  } else {
    None
  };
  let items = if embed.wants_names() {
    state
      .svc
      .catalog_service
      .product_group_list_with_names(pg)
      .await?
  } else {
    state.svc.catalog_service.product_group_list(pg).await?
  };
  Ok(ApiResponse::success(items))
}

#[utoipa::path(
  post,
  tag = "Catalog",
  operation_id = "catalog_product_group_create",
  summary = "Create product group",
  description = "Creates a product group and links it to an existing product type.",
  path = paths::catalog::PRODUCT_GROUPS,
  request_body = CreateProductGroupRequest,
  responses((status = 200, body = ApiResponse<ProductGroupResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn product_group_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateProductGroupRequest>>,
) -> ApiResult<ProductGroupResponse> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.product_group_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Catalog",
  operation_id = "catalog_product_group_get",
  summary = "Get product group",
  path = paths::catalog::PRODUCT_GROUPS_BY_ID,
  params(("id" = Uuid, Path), ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")),
  responses((status = 200, body = ApiResponse<ProductGroupResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn product_group_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<ProductGroupResponse> {
  let item = if embed.wants_names() {
    state
      .svc
      .catalog_service
      .product_group_get_with_names(id)
      .await?
  } else {
    state.svc.catalog_service.product_group_get(id).await?
  };
  Ok(ApiResponse::success(item))
}

#[utoipa::path(
  put,
  tag = "Catalog",
  operation_id = "catalog_product_group_update",
  summary = "Update product group",
  path = paths::catalog::PRODUCT_GROUPS_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateProductGroupRequest,
  responses((status = 200, body = ApiResponse<ProductGroupResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn product_group_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateProductGroupRequest>>,
) -> ApiResult<ProductGroupResponse> {
  Ok(ApiResponse::success(
    state
      .svc
      .catalog_service
      .product_group_update(id, &req)
      .await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Catalog",
  operation_id = "catalog_product_group_soft_delete",
  summary = "Soft delete product group",
  path = paths::catalog::PRODUCT_GROUPS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn product_group_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state
    .svc
    .catalog_service
    .product_group_soft_delete(id)
    .await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Catalog",
  operation_id = "catalog_product_group_hard_delete",
  summary = "Hard delete product group",
  path = paths::catalog::PRODUCT_GROUPS_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404), (status = 409))
)]
#[axum::debug_handler]
async fn product_group_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state
    .svc
    .catalog_service
    .product_group_hard_delete(id)
    .await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  post,
  tag = "Catalog",
  operation_id = "catalog_product_group_restore",
  summary = "Restore soft-deleted product group",
  path = paths::catalog::PRODUCT_GROUPS_RESTORE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn product_group_restore(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state
    .svc
    .catalog_service
    .product_group_soft_delete_undo(id)
    .await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn product_group_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(product_group_list, product_group_create))
    .routes(routes!(product_group_get, product_group_update))
    .routes(routes!(product_group_soft_delete))
    .routes(routes!(product_group_hard_delete))
    .routes(routes!(product_group_restore))
    .with_state(state)
}
