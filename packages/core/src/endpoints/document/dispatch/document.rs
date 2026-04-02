use super::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DispatchQueryParams {
  document_number: Option<String>,
  status: Option<enums::DocumentStatus>,
  contractor_id: Option<Uuid>,
  dispatch_method: Option<enums::DispatchMethod>,
  dispatch_purpose: Option<enums::DispatchPurpose>,
  #[serde(flatten)]
  pagination: PaginationParams,
}

#[utoipa::path(
  get,
  tag = "Document - Dispatch",
  operation_id = "dispatch_document_list",
  summary = "List dispatch documents",
  description = "Returns dispatch documents. Supports in-memory query filtering and pagination via common entity query params.",
  path = paths::dispatch::ROOT,
  params(
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<Vec<DispatchResponse>>))
)]
#[axum::debug_handler]
async fn dispatch_document_list(
  State(state): State<Arc<ApiState>>,
  Query(pagination): Query<PaginationParams>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<Vec<DispatchResponse>> {
  let rows = if embed.wants_names() {
    state
      .svc
      .document
      .dispatch_document_query_with_names(
        None,
        None,
        None,
        None,
        None,
        pagination.page,
        pagination.per_page,
      )
      .await?
  } else {
    state
      .svc
      .document
      .dispatch_document_query(
        None,
        None,
        None,
        None,
        None,
        pagination.page,
        pagination.per_page,
      )
      .await?
  };
  Ok(ApiResponse::success(rows))
}

#[utoipa::path(
  post,
  tag = "Document - Dispatch",
  operation_id = "dispatch_document_create",
  summary = "Create dispatch draft",
  description = "Creates a dispatch document in draft state.",
  path = paths::dispatch::SAVE,
  request_body = CreateDispatchRequest,
  responses((status = 200, body = ApiResponse<DispatchResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn dispatch_document_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateDispatchRequest>>,
) -> ApiResult<DispatchResponse> {
  Ok(ApiResponse::success(
    state.svc.document.dispatch_document_create(&req).await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Document - Dispatch",
  operation_id = "dispatch_document_create_and_execute",
  summary = "Create and execute dispatch document",
  description = "Creates and executes a dispatch document in one step. Requires supervisor or higher role.",
  path = paths::dispatch::SAVE_AND_EXECUTE,
  request_body = CreateDispatchRequest,
  responses((status = 200, body = ApiResponse<DispatchResponse>), (status = 400), (status = 403), (status = 409))
)]
#[axum::debug_handler]
async fn dispatch_document_create_and_execute(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
  Valid(Json(req)): Valid<Json<CreateDispatchRequest>>,
) -> ApiResult<DispatchResponse> {
  ensure_supervisor_or_higher(&claims.role)?;
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .dispatch_document_create_and_execute(&req, claims.uid)
      .await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Document - Dispatch",
  operation_id = "dispatch_document_query",
  summary = "Query dispatch documents",
  description = "Queries dispatch documents by optional filters.",
  path = paths::dispatch::QUERY,
  params(
    ("documentNumber" = Option<String>, Query),
    ("status" = Option<enums::DocumentStatus>, Query),
    ("contractorId" = Option<Uuid>, Query),
    ("dispatchMethod" = Option<enums::DispatchMethod>, Query),
    ("dispatchPurpose" = Option<enums::DispatchPurpose>, Query),
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<Vec<DispatchResponse>>), (status = 400))
)]
#[axum::debug_handler]
async fn dispatch_document_query(
  State(state): State<Arc<ApiState>>,
  Query(query): Query<DispatchQueryParams>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<Vec<DispatchResponse>> {
  let rows = if embed.wants_names() {
    state
      .svc
      .document
      .dispatch_document_query_with_names(
        query.document_number.as_deref(),
        query.status,
        query.contractor_id,
        query.dispatch_method,
        query.dispatch_purpose,
        query.pagination.page,
        query.pagination.per_page,
      )
      .await?
  } else {
    state
      .svc
      .document
      .dispatch_document_query(
        query.document_number.as_deref(),
        query.status,
        query.contractor_id,
        query.dispatch_method,
        query.dispatch_purpose,
        query.pagination.page,
        query.pagination.per_page,
      )
      .await?
  };
  Ok(ApiResponse::success(rows))
}

#[utoipa::path(
  get,
  tag = "Document - Dispatch",
  operation_id = "dispatch_document_get",
  summary = "Get dispatch document",
  path = paths::dispatch::BY_ID,
  params(
    ("id" = Uuid, Path),
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<DispatchResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn dispatch_document_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<DispatchResponse> {
  let row = if embed.wants_names() {
    state
      .svc
      .document
      .dispatch_document_get_with_names(id)
      .await?
  } else {
    state.svc.document.dispatch_document_get(id).await?
  };
  Ok(ApiResponse::success(row))
}

#[utoipa::path(
  put,
  tag = "Document - Dispatch",
  operation_id = "dispatch_document_update",
  summary = "Update dispatch document",
  path = paths::dispatch::BY_ID,
  params(("id" = Uuid, Path)),
  request_body = crate::dtos::UpdateDispatchRequest,
  responses((status = 200, body = ApiResponse<DispatchResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn dispatch_document_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<crate::dtos::UpdateDispatchRequest>>,
) -> ApiResult<DispatchResponse> {
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .dispatch_document_update(id, &req)
      .await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Document - Dispatch",
  operation_id = "dispatch_document_soft_delete",
  summary = "Soft delete dispatch document",
  path = paths::dispatch::BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn dispatch_document_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.dispatch_document_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Document - Dispatch",
  operation_id = "dispatch_document_hard_delete",
  summary = "Hard delete dispatch document",
  path = paths::dispatch::HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn dispatch_document_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.dispatch_document_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  post,
  tag = "Document - Dispatch",
  operation_id = "dispatch_document_execute",
  summary = "Execute dispatch document",
  description = "Executes a dispatch draft document and applies outbound ledger effects. Requires supervisor or higher role.",
  path = paths::dispatch::EXECUTE_BY_ID,
  params(("id" = Uuid, Path)),
  responses(
    (status = 200, description = "Dispatch document executed. Example: {\"success\":true,\"data\":null}"),
    (status = 403, description = "Forbidden envelope. Example: {\"success\":false,\"error\":{\"code\":\"FORBIDDEN\",\"message\":\"Forbidden: ...\"}}"),
    (status = 404, description = "Not found envelope. Example: {\"success\":false,\"error\":{\"code\":\"NOT_FOUND\",\"message\":\"Not found: Dispatch document '<id>' not found\"}}"),
    (status = 409, description = "Conflict envelope for invalid state or balance rules. Example: {\"success\":false,\"error\":{\"code\":\"CONFLICT\",\"message\":\"Conflict: ...\"}}")
  )
)]
#[axum::debug_handler]
async fn dispatch_document_execute(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  ensure_supervisor_or_higher(&claims.role)?;
  state
    .svc
    .document
    .dispatch_document_execute(id, claims.uid)
    .await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  post,
  tag = "Document - Dispatch",
  operation_id = "dispatch_document_revert",
  summary = "Revert dispatch document",
  description = "Reverts an executed dispatch document back to draft and compensates ledger effects. Requires senior supervisor or higher role.",
  path = paths::dispatch::REVERT_BY_ID,
  params(("id" = Uuid, Path)),
  responses(
    (status = 200, description = "Dispatch document reverted. Example: {\"success\":true,\"data\":null}"),
    (status = 403, description = "Forbidden envelope. Example: {\"success\":false,\"error\":{\"code\":\"FORBIDDEN\",\"message\":\"Forbidden: ...\"}}"),
    (status = 404, description = "Not found envelope. Example: {\"success\":false,\"error\":{\"code\":\"NOT_FOUND\",\"message\":\"Not found: Dispatch document '<id>' not found\"}}"),
    (status = 409, description = "Conflict envelope for invalid state. Example: {\"success\":false,\"error\":{\"code\":\"CONFLICT\",\"message\":\"Conflict: ...\"}}")
  )
)]
#[axum::debug_handler]
async fn dispatch_document_revert(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  ensure_senior_supervisor_or_higher(&claims.role)?;
  state
    .svc
    .document
    .dispatch_document_revert(id, claims.uid)
    .await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn document_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(dispatch_document_list, dispatch_document_create))
    .routes(routes!(dispatch_document_create_and_execute))
    .routes(routes!(dispatch_document_query))
    .routes(routes!(dispatch_document_get, dispatch_document_update))
    .routes(routes!(dispatch_document_soft_delete))
    .routes(routes!(dispatch_document_hard_delete))
    .routes(routes!(dispatch_document_execute))
    .routes(routes!(dispatch_document_revert))
    .with_state(state)
}
