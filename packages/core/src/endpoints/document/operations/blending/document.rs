use super::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BlendingQueryParams {
  document_number: Option<String>,
  status: Option<enums::DocumentStatus>,
  contractor_id: Option<Uuid>,
  #[serde(flatten)]
  pagination: PaginationParams,
}

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "blending_document_list",
  summary = "List blending documents",
  description = "Returns blending documents. Supports in-memory query filtering and pagination.",
  path = paths::blending::ROOT,
  params(
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query)
  ),
  responses((status = 200, body = ApiResponse<Vec<BlendingResponse>>))
)]
#[axum::debug_handler]
async fn blending_document_list(
  State(state): State<Arc<ApiState>>,
  Query(pagination): Query<PaginationParams>,
) -> ApiResult<Vec<BlendingResponse>> {
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .blending_document_query(None, None, None, pagination.page, pagination.per_page)
      .await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "blending_document_create",
  summary = "Create blending draft",
  description = "Creates a blending document in draft state.",
  path = paths::blending::SAVE,
  request_body = CreateBlendingRequest,
  responses((status = 200, body = ApiResponse<BlendingResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn blending_document_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateBlendingRequest>>,
) -> ApiResult<BlendingResponse> {
  Ok(ApiResponse::success(
    state.svc.document.blending_document_create(&req).await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "blending_document_create_and_execute",
  summary = "Create and execute blending document",
  path = paths::blending::SAVE_AND_EXECUTE,
  request_body = CreateBlendingRequest,
  responses((status = 200, body = ApiResponse<BlendingResponse>), (status = 400), (status = 403), (status = 409))
)]
#[axum::debug_handler]
async fn blending_document_create_and_execute(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
  Valid(Json(req)): Valid<Json<CreateBlendingRequest>>,
) -> ApiResult<BlendingResponse> {
  ensure_supervisor_or_higher(&claims.role)?;
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .blending_document_create_and_execute(&req, claims.uid)
      .await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "blending_document_query",
  summary = "Query blending documents",
  path = paths::blending::QUERY,
  params(
    ("documentNumber" = Option<String>, Query),
    ("status" = Option<enums::DocumentStatus>, Query),
    ("contractorId" = Option<Uuid>, Query),
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query)
  ),
  responses((status = 200, body = ApiResponse<Vec<BlendingResponse>>), (status = 400))
)]
#[axum::debug_handler]
async fn blending_document_query(
  State(state): State<Arc<ApiState>>,
  Query(query): Query<BlendingQueryParams>,
) -> ApiResult<Vec<BlendingResponse>> {
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .blending_document_query(
        query.document_number.as_deref(),
        query.status,
        query.contractor_id,
        query.pagination.page,
        query.pagination.per_page,
      )
      .await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "blending_document_get",
  summary = "Get blending document",
  path = paths::blending::BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<BlendingResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn blending_document_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<BlendingResponse> {
  Ok(ApiResponse::success(
    state.svc.document.blending_document_get(id).await?,
  ))
}

#[utoipa::path(
  put,
  tag = "Document - Operations",
  operation_id = "blending_document_update",
  summary = "Update blending document",
  path = paths::blending::BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateBlendingRequest,
  responses((status = 200, body = ApiResponse<BlendingResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn blending_document_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateBlendingRequest>>,
) -> ApiResult<BlendingResponse> {
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .blending_document_update(id, &req)
      .await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Document - Operations",
  operation_id = "blending_document_soft_delete",
  summary = "Soft delete blending document",
  path = paths::blending::BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn blending_document_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.blending_document_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Document - Operations",
  operation_id = "blending_document_hard_delete",
  summary = "Hard delete blending document",
  path = paths::blending::HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn blending_document_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.blending_document_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "blending_document_execute",
  summary = "Execute blending document",
  description = "Executes a draft blending document and applies its ledger effects. Requires supervisor or higher role.",
  path = paths::blending::EXECUTE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, description = "Blending document executed"), (status = 403), (status = 404), (status = 409))
)]
#[axum::debug_handler]
async fn blending_document_execute(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  ensure_supervisor_or_higher(&claims.role)?;
  state
    .svc
    .document
    .blending_document_execute(id, claims.uid)
    .await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "blending_document_revert",
  summary = "Revert blending document",
  description = "Reverts an executed blending document back to draft. Requires senior supervisor or higher role.",
  path = paths::blending::REVERT_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, description = "Blending document reverted"), (status = 403), (status = 404), (status = 409))
)]
#[axum::debug_handler]
async fn blending_document_revert(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  ensure_senior_supervisor_or_higher(&claims.role)?;
  state
    .svc
    .document
    .blending_document_revert(id, claims.uid)
    .await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn document_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(blending_document_list, blending_document_create))
    .routes(routes!(blending_document_create_and_execute))
    .routes(routes!(blending_document_query))
    .routes(routes!(blending_document_get, blending_document_update))
    .routes(routes!(blending_document_soft_delete))
    .routes(routes!(blending_document_hard_delete))
    .routes(routes!(blending_document_execute))
    .routes(routes!(blending_document_revert))
    .with_state(state)
}
