use super::*;

#[utoipa::path(
  get,
  tag = "Document - Dispatch",
  operation_id = "dispatch_composite_get",
  summary = "Get dispatch composite",
  description = "Returns a dispatch document with nested items and storage measurements.",
  path = paths::dispatch::COMPOSITE_BY_ID,
  params(
    ("id" = Uuid, Path),
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<DispatchCompositeResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn dispatch_composite_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<DispatchCompositeResponse> {
  let row = if embed.wants_names() {
    state
      .svc
      .document
      .dispatch_composite_get_with_names(id)
      .await?
  } else {
    state.svc.document.dispatch_composite_get(id).await?
  };
  Ok(ApiResponse::success(row))
}

#[utoipa::path(
  post,
  tag = "Document - Dispatch",
  operation_id = "dispatch_composite_create",
  summary = "Create dispatch composite",
  description = "Creates a full dispatch aggregate without executing it.",
  path = paths::dispatch::COMPOSITE_SAVE,
  request_body = CreateDispatchCompositeRequest,
  responses((status = 200, body = ApiResponse<DispatchCompositeResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn dispatch_composite_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateDispatchCompositeRequest>>,
) -> ApiResult<DispatchCompositeResponse> {
  Ok(ApiResponse::success(
    state.svc.document.dispatch_composite_create(&req).await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Document - Dispatch",
  operation_id = "dispatch_composite_create_and_execute",
  summary = "Create and execute dispatch composite",
  description = "Creates a full dispatch aggregate and executes it in one transactional flow. Requires supervisor or higher role.",
  path = paths::dispatch::COMPOSITE_SAVE_AND_EXECUTE,
  request_body = CreateDispatchCompositeRequest,
  responses(
    (status = 200, body = ApiResponse<DispatchCompositeResponse>, description = "Composite create+execute success envelope. Example: {\"success\":true,\"data\":{\"document\":{...},\"items\":[...],\"storageMeasurements\":[...]}}"),
    (status = 400, description = "Validation or bad request envelope. Example: {\"success\":false,\"error\":{\"code\":\"VALIDATION_ERROR\",\"message\":\"Validation error: ...\"}}"),
    (status = 409, description = "Conflict envelope for domain constraints. Example: {\"success\":false,\"error\":{\"code\":\"CONFLICT\",\"message\":\"Conflict: ...\"}}")
  )
)]
#[axum::debug_handler]
async fn dispatch_composite_create_and_execute(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
  Valid(Json(req)): Valid<Json<CreateDispatchCompositeRequest>>,
) -> ApiResult<DispatchCompositeResponse> {
  ensure_supervisor_or_higher(&claims.role)?;
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .dispatch_composite_create_and_execute(&req, claims.uid)
      .await?,
  ))
}

pub(super) fn composite_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(dispatch_composite_get))
    .routes(routes!(dispatch_composite_create))
    .routes(routes!(dispatch_composite_create_and_execute))
    .with_state(state)
}
