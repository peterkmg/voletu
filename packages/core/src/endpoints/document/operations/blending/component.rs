use super::*;

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "blending_component_list",
  summary = "List blending components",
  description = "Returns blending component rows.",
  path = paths::blending::COMPONENTS,
  responses((status = 200, body = ApiResponse<Vec<BlendingComponentResponse>>))
)]
#[axum::debug_handler]
async fn blending_component_list(
  State(state): State<Arc<ApiState>>,
) -> ApiResult<Vec<BlendingComponentResponse>> {
  let rows = state.svc.document.blending_component_list().await?;
  Ok(ApiResponse::success(rows))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "blending_component_create",
  summary = "Create blending component",
  description = "Creates a blending component row under an existing blending document.",
  path = paths::blending::COMPONENTS,
  request_body = CreateBlendingComponentRequest,
  responses((status = 200, body = ApiResponse<BlendingComponentResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn blending_component_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateBlendingComponentRequest>>,
) -> ApiResult<BlendingComponentResponse> {
  Ok(ApiResponse::success(
    state.svc.document.blending_component_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "blending_component_get",
  summary = "Get blending component",
  path = paths::blending::COMPONENTS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<BlendingComponentResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn blending_component_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<BlendingComponentResponse> {
  Ok(ApiResponse::success(
    state.svc.document.blending_component_get(id).await?,
  ))
}

#[utoipa::path(
  put,
  tag = "Document - Operations",
  operation_id = "blending_component_update",
  summary = "Update blending component",
  path = paths::blending::COMPONENTS_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateBlendingComponentRequest,
  responses((status = 200, body = ApiResponse<BlendingComponentResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn blending_component_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateBlendingComponentRequest>>,
) -> ApiResult<BlendingComponentResponse> {
  Ok(ApiResponse::success(
    state.svc.document.blending_component_update(id, &req).await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Document - Operations",
  operation_id = "blending_component_soft_delete",
  summary = "Soft delete blending component",
  path = paths::blending::COMPONENTS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn blending_component_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.blending_component_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Document - Operations",
  operation_id = "blending_component_hard_delete",
  summary = "Hard delete blending component",
  path = paths::blending::COMPONENTS_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn blending_component_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.blending_component_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn component_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(blending_component_list, blending_component_create))
    .routes(routes!(blending_component_get, blending_component_update))
    .routes(routes!(blending_component_soft_delete))
    .routes(routes!(blending_component_hard_delete))
    .with_state(state)
}
