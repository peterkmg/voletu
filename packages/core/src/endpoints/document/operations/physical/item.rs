use super::*;

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "physical_item_list",
  path = paths::operations::PHYSICAL_TRANSFER_ITEMS,
  responses((status = 200, body = ApiResponse<Vec<PhysicalTransferItemResponse>>))
)]
#[axum::debug_handler]
async fn physical_item_list(
  State(state): State<Arc<ApiState>>,
) -> ApiResult<Vec<PhysicalTransferItemResponse>> {
  let rows = state.svc.document.physical_item_list(None).await?;
  Ok(ApiResponse::success(rows))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "physical_item_create",
  path = paths::operations::PHYSICAL_TRANSFER_ITEMS,
  request_body = CreatePhysicalTransferItemRequest,
  responses((status = 200, body = ApiResponse<PhysicalTransferItemResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn physical_item_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreatePhysicalTransferItemRequest>>,
) -> ApiResult<PhysicalTransferItemResponse> {
  Ok(ApiResponse::success(
    state.svc.document.physical_item_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "physical_item_get",
  path = paths::operations::PHYSICAL_TRANSFER_ITEMS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<PhysicalTransferItemResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn physical_item_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<PhysicalTransferItemResponse> {
  Ok(ApiResponse::success(
    state.svc.document.physical_item_get(id).await?,
  ))
}

#[utoipa::path(
  put,
  tag = "Document - Operations",
  operation_id = "physical_item_update",
  path = paths::operations::PHYSICAL_TRANSFER_ITEMS_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdatePhysicalTransferItemRequest,
  responses((status = 200, body = ApiResponse<PhysicalTransferItemResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn physical_item_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdatePhysicalTransferItemRequest>>,
) -> ApiResult<PhysicalTransferItemResponse> {
  Ok(ApiResponse::success(
    state.svc.document.physical_item_update(id, &req).await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Document - Operations",
  operation_id = "physical_item_soft_delete",
  path = paths::operations::PHYSICAL_TRANSFER_ITEMS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn physical_item_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.physical_item_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Document - Operations",
  operation_id = "physical_item_hard_delete",
  path = paths::operations::PHYSICAL_TRANSFER_ITEMS_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn physical_item_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.physical_item_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn item_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(physical_item_list, physical_item_create))
    .routes(routes!(physical_item_get, physical_item_update))
    .routes(routes!(physical_item_soft_delete))
    .routes(routes!(physical_item_hard_delete))
    .with_state(state)
}
