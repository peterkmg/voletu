use super::*;

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "blending_result_list",
  summary = "List blending results",
  description = "Returns blending result rows.",
  path = paths::blending::RESULTS,
  responses((status = 200, body = ApiResponse<Vec<BlendingResultResponse>>))
)]
#[axum::debug_handler]
async fn blending_result_list(
  State(state): State<Arc<ApiState>>,
) -> ApiResult<Vec<BlendingResultResponse>> {
  let rows = state.svc.document.blending_result_list().await?;
  Ok(ApiResponse::success(rows))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "blending_result_create",
  summary = "Create blending result",
  description = "Creates a blending result row under an existing blending document.",
  path = paths::blending::RESULTS,
  request_body = CreateBlendingResultRequest,
  responses((status = 200, body = ApiResponse<BlendingResultResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn blending_result_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateBlendingResultRequest>>,
) -> ApiResult<BlendingResultResponse> {
  Ok(ApiResponse::success(
    state.svc.document.blending_result_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "blending_result_get",
  summary = "Get blending result",
  path = paths::blending::RESULTS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<BlendingResultResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn blending_result_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<BlendingResultResponse> {
  Ok(ApiResponse::success(
    state.svc.document.blending_result_get(id).await?,
  ))
}

#[utoipa::path(
  put,
  tag = "Document - Operations",
  operation_id = "blending_result_update",
  summary = "Update blending result",
  path = paths::blending::RESULTS_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateBlendingResultRequest,
  responses((status = 200, body = ApiResponse<BlendingResultResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn blending_result_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateBlendingResultRequest>>,
) -> ApiResult<BlendingResultResponse> {
  Ok(ApiResponse::success(
    state.svc.document.blending_result_update(id, &req).await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Document - Operations",
  operation_id = "blending_result_soft_delete",
  summary = "Soft delete blending result",
  path = paths::blending::RESULTS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn blending_result_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.blending_result_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Document - Operations",
  operation_id = "blending_result_hard_delete",
  summary = "Hard delete blending result",
  path = paths::blending::RESULTS_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn blending_result_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.blending_result_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn result_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(blending_result_list, blending_result_create))
    .routes(routes!(blending_result_get, blending_result_update))
    .routes(routes!(blending_result_soft_delete))
    .routes(routes!(blending_result_hard_delete))
    .with_state(state)
}
