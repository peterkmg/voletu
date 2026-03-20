use super::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OwnershipTransferQueryParams {
  status: Option<enums::DocumentStatus>,
  #[serde(flatten)]
  pagination: PaginationParams,
}

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "ownership_transfer_list",
  summary = "List ownership transfers",
  description = "Returns ownership transfer documents with nested items. Supports in-memory query filtering and pagination.",
  path = paths::operations::OWNERSHIP_TRANSFERS,
  params(
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query)
  ),
  responses((status = 200, body = ApiResponse<Vec<OwnershipTransferResponse>>))
)]
#[axum::debug_handler]
async fn ownership_transfer_list(
  State(state): State<Arc<ApiState>>,
  Query(pagination): Query<PaginationParams>,
) -> ApiResult<Vec<OwnershipTransferResponse>> {
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .ownership_transfer_composite_query(None, pagination.page, pagination.per_page)
      .await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "ownership_transfer_composite_get",
  summary = "Get ownership transfer composite",
  path = paths::operations::OWNERSHIP_TRANSFERS_COMPOSITE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<OwnershipTransferResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn ownership_transfer_composite_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<OwnershipTransferResponse> {
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .ownership_transfer_composite_get(id)
      .await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "ownership_transfer_query",
  summary = "Query ownership transfers",
  path = paths::operations::OWNERSHIP_TRANSFERS_QUERY,
  params(
    ("status" = Option<enums::DocumentStatus>, Query),
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query)
  ),
  responses((status = 200, body = ApiResponse<Vec<OwnershipTransferResponse>>), (status = 400))
)]
#[axum::debug_handler]
async fn ownership_transfer_query(
  State(state): State<Arc<ApiState>>,
  Query(query): Query<OwnershipTransferQueryParams>,
) -> ApiResult<Vec<OwnershipTransferResponse>> {
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .ownership_transfer_composite_query(
        query.status,
        query.pagination.page,
        query.pagination.per_page,
      )
      .await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "ownership_transfer_create",
  summary = "Create ownership transfer draft",
  description = "Creates an ownership transfer aggregate in draft state.",
  path = paths::operations::OWNERSHIP_TRANSFERS_SAVE,
  request_body = CreateOwnershipTransferRequest,
  responses((status = 200, body = ApiResponse<OwnershipTransferResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn ownership_transfer_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateOwnershipTransferRequest>>,
) -> ApiResult<OwnershipTransferResponse> {
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .ownership_transfer_composite_create(&req)
      .await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "ownership_transfer_create_and_execute",
  summary = "Create and execute ownership transfer",
  description = "Creates and executes an ownership transfer in one transactional flow. Requires supervisor or higher role.",
  path = paths::operations::OWNERSHIP_TRANSFERS_SAVE_AND_EXECUTE,
  request_body = CreateOwnershipTransferRequest,
  responses(
    (status = 200, body = ApiResponse<OwnershipTransferResponse>, description = "Create+execute success envelope. Example: {\"success\":true,\"data\":{\"id\":\"...\",\"status\":\"Posted\",\"items\":[...]}}"),
    (status = 400, description = "Validation or bad request envelope. Example: {\"success\":false,\"error\":{\"code\":\"VALIDATION_ERROR\",\"message\":\"Validation error: ...\"}}"),
    (status = 403, description = "Forbidden envelope for insufficient role. Example: {\"success\":false,\"error\":{\"code\":\"FORBIDDEN\",\"message\":\"Forbidden: ...\"}}"),
    (status = 409, description = "Conflict envelope for domain constraints. Example: {\"success\":false,\"error\":{\"code\":\"CONFLICT\",\"message\":\"Conflict: ...\"}}")
  )
)]
#[axum::debug_handler]
async fn ownership_transfer_create_and_execute(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
  Valid(Json(req)): Valid<Json<CreateOwnershipTransferRequest>>,
) -> ApiResult<OwnershipTransferResponse> {
  ensure_supervisor_or_higher(&claims.role)?;
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .ownership_transfer_composite_create_and_execute(&req, claims.uid)
      .await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "ownership_transfer_execute",
  summary = "Execute ownership transfer",
  description = "Executes a draft ownership transfer. Requires supervisor or higher role.",
  path = paths::operations::OWNERSHIP_TRANSFERS_EXECUTE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, description = "Ownership transfer executed"), (status = 403), (status = 404), (status = 409))
)]
#[axum::debug_handler]
async fn ownership_transfer_execute(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  ensure_supervisor_or_higher(&claims.role)?;
  state
    .svc
    .document
    .ownership_transfer_execute(id, claims.uid)
    .await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "ownership_transfer_revert",
  summary = "Revert ownership transfer",
  description = "Reverts an executed ownership transfer back to draft. Requires senior supervisor or higher role.",
  path = paths::operations::OWNERSHIP_TRANSFERS_REVERT_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, description = "Ownership transfer reverted"), (status = 403), (status = 404), (status = 409))
)]
#[axum::debug_handler]
async fn ownership_transfer_revert(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  ensure_senior_supervisor_or_higher(&claims.role)?;
  state
    .svc
    .document
    .ownership_transfer_revert(id, claims.uid)
    .await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn transfer_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(ownership_transfer_list, ownership_transfer_create))
    .routes(routes!(ownership_transfer_composite_get))
    .routes(routes!(ownership_transfer_query))
    .routes(routes!(ownership_transfer_create_and_execute))
    .routes(routes!(ownership_transfer_execute))
    .routes(routes!(ownership_transfer_revert))
    .with_state(state)
}
