use super::*;

#[utoipa::path(
  get,
  tag = "Document - Dispatch",
  operation_id = "dispatch_item_list",
  summary = "List dispatch items",
  description = "Returns dispatch item rows. Supports common entity query filtering and pagination.",
  path = paths::dispatch::ITEMS,
  responses((status = 200, body = ApiResponse<Vec<DispatchItemResponse>>))
)]
#[axum::debug_handler]
async fn dispatch_item_list(
  State(state): State<Arc<ApiState>>,
) -> ApiResult<Vec<DispatchItemResponse>> {
  Ok(ApiResponse::success(
    state.svc.document.dispatch_item_list().await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Document - Dispatch",
  operation_id = "dispatch_item_create",
  summary = "Create dispatch item",
  description = "Creates an item under an existing dispatch draft document.",
  path = paths::dispatch::ITEMS,
  request_body = CreateDispatchItemRequest,
  responses((status = 200, body = ApiResponse<DispatchItemResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn dispatch_item_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateDispatchItemRequest>>,
) -> ApiResult<DispatchItemResponse> {
  Ok(ApiResponse::success(
    state.svc.document.dispatch_item_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Document - Dispatch",
  operation_id = "dispatch_item_get",
  summary = "Get dispatch item",
  path = paths::dispatch::ITEMS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<DispatchItemResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn dispatch_item_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<DispatchItemResponse> {
  Ok(ApiResponse::success(
    state.svc.document.dispatch_item_get(id).await?,
  ))
}

#[utoipa::path(
  put,
  tag = "Document - Dispatch",
  operation_id = "dispatch_item_update",
  summary = "Update dispatch item",
  path = paths::dispatch::ITEMS_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = crate::dtos::UpdateDispatchItemRequest,
  responses((status = 200, body = ApiResponse<DispatchItemResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn dispatch_item_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<crate::dtos::UpdateDispatchItemRequest>>,
) -> ApiResult<DispatchItemResponse> {
  Ok(ApiResponse::success(
    state.svc.document.dispatch_item_update(id, &req).await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Document - Dispatch",
  operation_id = "dispatch_item_soft_delete",
  summary = "Soft delete dispatch item",
  path = paths::dispatch::ITEMS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn dispatch_item_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.dispatch_item_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Document - Dispatch",
  operation_id = "dispatch_item_hard_delete",
  summary = "Hard delete dispatch item",
  path = paths::dispatch::ITEMS_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn dispatch_item_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.dispatch_item_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn item_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(dispatch_item_list, dispatch_item_create))
    .routes(routes!(dispatch_item_get, dispatch_item_update))
    .routes(routes!(dispatch_item_soft_delete))
    .routes(routes!(dispatch_item_hard_delete))
    .with_state(state)
}
