use super::*;

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "ownership_item_list",
  path = paths::operations::OWNERSHIP_TRANSFER_ITEMS,
  responses((status = 200, body = ApiResponse<Vec<OwnershipTransferItemResponse>>))
)]
#[axum::debug_handler]
async fn ownership_item_list(
  State(state): State<Arc<ApiState>>,
) -> ApiResult<Vec<OwnershipTransferItemResponse>> {
  let rows = state.svc.document.ownership_item_list().await?;
  Ok(ApiResponse::success(rows))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "ownership_item_create",
  path = paths::operations::OWNERSHIP_TRANSFER_ITEMS,
  request_body = CreateOwnershipTransferItemRequest,
  responses((status = 200, body = ApiResponse<OwnershipTransferItemResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn ownership_item_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateOwnershipTransferItemRequest>>,
) -> ApiResult<OwnershipTransferItemResponse> {
  Ok(ApiResponse::success(
    state.svc.document.ownership_item_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "ownership_item_get",
  path = paths::operations::OWNERSHIP_TRANSFER_ITEMS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<OwnershipTransferItemResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn ownership_item_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<OwnershipTransferItemResponse> {
  Ok(ApiResponse::success(state.svc.document.ownership_item_get(id).await?))
}

#[utoipa::path(
  put,
  tag = "Document - Operations",
  operation_id = "ownership_item_update",
  path = paths::operations::OWNERSHIP_TRANSFER_ITEMS_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateOwnershipTransferItemRequest,
  responses((status = 200, body = ApiResponse<OwnershipTransferItemResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn ownership_item_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateOwnershipTransferItemRequest>>,
) -> ApiResult<OwnershipTransferItemResponse> {
  Ok(ApiResponse::success(
    state.svc.document.ownership_item_update(id, &req).await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Document - Operations",
  operation_id = "ownership_item_soft_delete",
  path = paths::operations::OWNERSHIP_TRANSFER_ITEMS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn ownership_item_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.ownership_item_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Document - Operations",
  operation_id = "ownership_item_hard_delete",
  path = paths::operations::OWNERSHIP_TRANSFER_ITEMS_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn ownership_item_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.ownership_item_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn item_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(ownership_item_list, ownership_item_create))
    .routes(routes!(ownership_item_get, ownership_item_update))
    .routes(routes!(ownership_item_soft_delete))
    .routes(routes!(ownership_item_hard_delete))
    .with_state(state)
}
