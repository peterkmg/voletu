use super::*;

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "adjustment_list",
  summary = "List reconciliation adjustments",
  description = "Returns reconciliation adjustment rows.",
  path = paths::operations::RECONCILIATION_ADJUSTMENTS,
  responses((status = 200, body = ApiResponse<Vec<InventoryAdjustmentResponse>>))
)]
#[axum::debug_handler]
async fn adjustment_list(
  State(state): State<Arc<ApiState>>,
) -> ApiResult<Vec<InventoryAdjustmentResponse>> {
  let rows = state.svc.document.adjustment_list(None).await?;
  Ok(ApiResponse::success(rows))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "adjustment_create",
  summary = "Create reconciliation adjustment",
  description = "Creates an adjustment under a reconciliation document.",
  path = paths::operations::RECONCILIATION_ADJUSTMENTS_SAVE,
  request_body = CreateInventoryAdjustmentRequest,
  responses((status = 200, body = ApiResponse<InventoryAdjustmentResponse>), (status = 400), (status = 409))
)]
#[axum::debug_handler]
async fn adjustment_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateInventoryAdjustmentRequest>>,
) -> ApiResult<InventoryAdjustmentResponse> {
  Ok(ApiResponse::success(
    state.svc.document.adjustment_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "adjustment_get",
  summary = "Get reconciliation adjustment",
  path = paths::operations::RECONCILIATION_ADJUSTMENTS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<InventoryAdjustmentResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn adjustment_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<InventoryAdjustmentResponse> {
  Ok(ApiResponse::success(
    state.svc.document.adjustment_get(id).await?,
  ))
}

#[utoipa::path(
  put,
  tag = "Document - Operations",
  operation_id = "adjustment_update",
  summary = "Update reconciliation adjustment",
  path = paths::operations::RECONCILIATION_ADJUSTMENTS_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateInventoryAdjustmentRequest,
  responses((status = 200, body = ApiResponse<InventoryAdjustmentResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn adjustment_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateInventoryAdjustmentRequest>>,
) -> ApiResult<InventoryAdjustmentResponse> {
  Ok(ApiResponse::success(
    state.svc.document.adjustment_update(id, &req).await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Document - Operations",
  operation_id = "adjustment_soft_delete",
  summary = "Soft delete reconciliation adjustment",
  path = paths::operations::RECONCILIATION_ADJUSTMENTS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn adjustment_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.adjustment_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Document - Operations",
  operation_id = "adjustment_hard_delete",
  summary = "Hard delete reconciliation adjustment",
  path = paths::operations::RECONCILIATION_ADJUSTMENTS_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn adjustment_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.adjustment_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn adjustment_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(adjustment_list, adjustment_create))
    .routes(routes!(adjustment_get, adjustment_update))
    .routes(routes!(adjustment_soft_delete))
    .routes(routes!(adjustment_hard_delete))
    .with_state(state)
}
