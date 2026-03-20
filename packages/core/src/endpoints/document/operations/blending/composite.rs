use super::*;

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "blending_composite_get",
  summary = "Get blending composite",
  description = "Returns a blending document with nested components and results.",
  path = paths::blending::COMPOSITE_BY_ID,
  params(
    ("id" = Uuid, Path),
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<BlendingCompositeResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn blending_composite_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<BlendingCompositeResponse> {
  let row = if embed.wants_names() {
    state
      .svc
      .document
      .blending_composite_get_with_names(id)
      .await?
  } else {
    state.svc.document.blending_composite_get(id).await?
  };
  Ok(ApiResponse::success(row))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "blending_composite_create",
  path = paths::blending::COMPOSITE_SAVE,
  request_body = CreateBlendingCompositeRequest,
  responses((status = 200, body = ApiResponse<BlendingCompositeResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn blending_composite_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateBlendingCompositeRequest>>,
) -> ApiResult<BlendingCompositeResponse> {
  Ok(ApiResponse::success(
    state.svc.document.blending_composite_create(&req).await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "blending_composite_create_and_execute",
  summary = "Create and execute blending composite",
  description = "Creates a full blending aggregate and executes it in one transactional flow. Requires supervisor or higher role.",
  path = paths::blending::COMPOSITE_SAVE_AND_EXECUTE,
  request_body = CreateBlendingCompositeRequest,
  responses(
    (status = 200, body = ApiResponse<BlendingCompositeResponse>, description = "Composite create+execute success envelope. Example: {\"success\":true,\"data\":{\"document\":{\"status\":\"Posted\"},\"components\":[...],\"results\":[...]}}"),
    (status = 400, description = "Validation or bad request envelope. Example: {\"success\":false,\"error\":{\"code\":\"BAD_REQUEST\",\"message\":\"Bad request: ...\"}}"),
    (status = 409, description = "Conflict envelope for domain constraints. Example: {\"success\":false,\"error\":{\"code\":\"CONFLICT\",\"message\":\"Conflict: ...\"}}")
  )
)]
#[axum::debug_handler]
async fn blending_composite_create_and_execute(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
  Valid(Json(req)): Valid<Json<CreateBlendingCompositeRequest>>,
) -> ApiResult<BlendingCompositeResponse> {
  ensure_supervisor_or_higher(&claims.role)?;
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .blending_composite_create_and_execute(&req, claims.uid)
      .await?,
  ))
}

pub(super) fn composite_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(blending_composite_get))
    .routes(routes!(blending_composite_create))
    .routes(routes!(blending_composite_create_and_execute))
    .with_state(state)
}
