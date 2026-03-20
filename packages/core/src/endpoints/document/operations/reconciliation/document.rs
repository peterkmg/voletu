use super::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReconciliationQueryParams {
  document_number: Option<String>,
  status: Option<enums::DocumentStatus>,
  warehouse_id: Option<Uuid>,
  #[serde(flatten)]
  pagination: PaginationParams,
}

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "reconciliation_list",
  summary = "List reconciliations",
  description = "Returns reconciliation documents. Supports in-memory query filtering and pagination.",
  path = paths::operations::RECONCILIATIONS,
  params(
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<Vec<InventoryReconciliationResponse>>))
)]
#[axum::debug_handler]
async fn reconciliation_list(
  State(state): State<Arc<ApiState>>,
  Query(pagination): Query<PaginationParams>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<Vec<InventoryReconciliationResponse>> {
  let rows = if embed.wants_names() {
    state
      .svc
      .document
      .reconciliation_query_with_names(None, None, None, pagination.page, pagination.per_page)
      .await?
  } else {
    state
      .svc
      .document
      .reconciliation_query(None, None, None, pagination.page, pagination.per_page)
      .await?
  };
  Ok(ApiResponse::success(rows))
}

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "reconciliation_query",
  summary = "Query reconciliations",
  path = paths::operations::RECONCILIATIONS_QUERY,
  params(
    ("documentNumber" = Option<String>, Query),
    ("status" = Option<enums::DocumentStatus>, Query),
    ("warehouseId" = Option<Uuid>, Query),
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<Vec<InventoryReconciliationResponse>>), (status = 400))
)]
#[axum::debug_handler]
async fn reconciliation_query(
  State(state): State<Arc<ApiState>>,
  Query(query): Query<ReconciliationQueryParams>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<Vec<InventoryReconciliationResponse>> {
  let rows = if embed.wants_names() {
    state
      .svc
      .document
      .reconciliation_query_with_names(
        query.document_number.as_deref(),
        query.status,
        query.warehouse_id,
        query.pagination.page,
        query.pagination.per_page,
      )
      .await?
  } else {
    state
      .svc
      .document
      .reconciliation_query(
        query.document_number.as_deref(),
        query.status,
        query.warehouse_id,
        query.pagination.page,
        query.pagination.per_page,
      )
      .await?
  };
  Ok(ApiResponse::success(rows))
}

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "reconciliation_get",
  summary = "Get reconciliation",
  path = paths::operations::RECONCILIATIONS_BY_ID,
  params(
    ("id" = Uuid, Path),
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<InventoryReconciliationResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn reconciliation_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<InventoryReconciliationResponse> {
  let row = if embed.wants_names() {
    state.svc.document.reconciliation_get_with_names(id).await?
  } else {
    state.svc.document.reconciliation_get(id).await?
  };
  Ok(ApiResponse::success(row))
}

#[utoipa::path(
  put,
  tag = "Document - Operations",
  operation_id = "reconciliation_update",
  summary = "Update reconciliation",
  path = paths::operations::RECONCILIATIONS_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateInventoryReconciliationRequest,
  responses((status = 200, body = ApiResponse<InventoryReconciliationResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn reconciliation_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateInventoryReconciliationRequest>>,
) -> ApiResult<InventoryReconciliationResponse> {
  Ok(ApiResponse::success(
    state.svc.document.reconciliation_update(id, &req).await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Document - Operations",
  operation_id = "reconciliation_soft_delete",
  summary = "Soft delete reconciliation",
  path = paths::operations::RECONCILIATIONS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn reconciliation_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.reconciliation_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Document - Operations",
  operation_id = "reconciliation_hard_delete",
  summary = "Hard delete reconciliation",
  path = paths::operations::RECONCILIATIONS_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn reconciliation_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.reconciliation_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "reconciliation_create",
  summary = "Create reconciliation draft",
  description = "Creates a reconciliation document in draft state.",
  path = paths::operations::RECONCILIATIONS_SAVE,
  request_body = CreateInventoryReconciliationRequest,
  responses((status = 200, body = ApiResponse<InventoryReconciliationResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn reconciliation_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateInventoryReconciliationRequest>>,
) -> ApiResult<InventoryReconciliationResponse> {
  Ok(ApiResponse::success(
    state.svc.document.reconciliation_create(&req).await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "reconciliation_create_and_execute",
  summary = "Create and execute reconciliation",
  description = "Creates and executes a reconciliation in one transactional flow. Requires supervisor or higher role.",
  path = paths::operations::RECONCILIATIONS_SAVE_AND_EXECUTE,
  request_body = CreateInventoryReconciliationRequest,
  responses(
    (status = 200, body = ApiResponse<InventoryReconciliationResponse>, description = "Create+execute success envelope. Example: {\"success\":true,\"data\":{\"id\":\"...\",\"status\":\"Posted\"}}"),
    (status = 400, description = "Validation or bad request envelope. Example: {\"success\":false,\"error\":{\"code\":\"VALIDATION_ERROR\",\"message\":\"Validation error: ...\"}}"),
    (status = 403, description = "Forbidden envelope for insufficient role. Example: {\"success\":false,\"error\":{\"code\":\"FORBIDDEN\",\"message\":\"Forbidden: ...\"}}"),
    (status = 409, description = "Conflict envelope for domain constraints. Example: {\"success\":false,\"error\":{\"code\":\"CONFLICT\",\"message\":\"Conflict: ...\"}}")
  )
)]
#[axum::debug_handler]
async fn reconciliation_create_and_execute(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
  Valid(Json(req)): Valid<Json<CreateInventoryReconciliationRequest>>,
) -> ApiResult<InventoryReconciliationResponse> {
  ensure_supervisor_or_higher(&claims.role)?;
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .reconciliation_create_and_execute(&req, claims.uid)
      .await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "reconciliation_execute",
  summary = "Execute reconciliation",
  description = "Executes a draft reconciliation document. Requires supervisor or higher role.",
  path = paths::operations::RECONCILIATIONS_EXECUTE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, description = "Reconciliation executed"), (status = 403), (status = 404), (status = 409))
)]
#[axum::debug_handler]
async fn reconciliation_execute(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  ensure_supervisor_or_higher(&claims.role)?;
  state
    .svc
    .document
    .reconciliation_execute(id, claims.uid)
    .await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "reconciliation_revert",
  summary = "Revert reconciliation",
  description = "Reverts an executed reconciliation back to draft. Requires senior supervisor or higher role.",
  path = paths::operations::RECONCILIATIONS_REVERT_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, description = "Reconciliation reverted"), (status = 403), (status = 404), (status = 409))
)]
#[axum::debug_handler]
async fn reconciliation_revert(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  ensure_senior_supervisor_or_higher(&claims.role)?;
  state
    .svc
    .document
    .reconciliation_revert(id, claims.uid)
    .await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn reconciliation_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(reconciliation_list, reconciliation_create))
    .routes(routes!(reconciliation_query))
    .routes(routes!(reconciliation_get, reconciliation_update))
    .routes(routes!(reconciliation_soft_delete))
    .routes(routes!(reconciliation_hard_delete))
    .routes(routes!(reconciliation_create_and_execute))
    .routes(routes!(reconciliation_execute))
    .routes(routes!(reconciliation_revert))
    .with_state(state)
}
