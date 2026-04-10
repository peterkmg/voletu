use super::*;
use crate::{
  dtos::{EmbedParams, PaginationParams, PhysicalTransferDocumentQueryParams},
  endpoints::paths,
  services::document::specs::PhysicalTransferQuerySpec,
};

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "physical_transfer_list",
  summary = "List physical transfers",
  description = "Returns physical transfer documents with nested items. Supports in-memory query filtering and pagination.",
  path = paths::operations::PHYSICAL_TRANSFERS,
  params(
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<Vec<PhysicalTransferResponse>>))
)]
#[axum::debug_handler]
async fn physical_transfer_list(
  State(state): State<Arc<ApiState>>,
  Query(pagination): Query<PaginationParams>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<Vec<PhysicalTransferResponse>> {
  let query = PhysicalTransferQuerySpec::list(pagination.page, pagination.per_page);
  let rows = if embed.wants_names() {
    state
      .svc
      .document
      .physical_transfer_composite_query_with_names(query)
      .await?
  } else {
    state
      .svc
      .document
      .physical_transfer_composite_query(query)
      .await?
  };
  Ok(ApiResponse::success(rows))
}

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "physical_transfer_composite_get",
  summary = "Get physical transfer composite",
  path = paths::operations::PHYSICAL_TRANSFERS_COMPOSITE_BY_ID,
  params(
    ("id" = Uuid, Path),
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<PhysicalTransferResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn physical_transfer_composite_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<PhysicalTransferResponse> {
  let row = if embed.wants_names() {
    state
      .svc
      .document
      .physical_transfer_composite_get_with_names(id)
      .await?
  } else {
    state
      .svc
      .document
      .physical_transfer_composite_get(id)
      .await?
  };
  Ok(ApiResponse::success(row))
}

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "physical_transfer_query",
  summary = "Query physical transfers",
  path = paths::operations::PHYSICAL_TRANSFERS_QUERY,
  params(
    ("documentNumber" = Option<String>, Query),
    ("status" = Option<enums::DocumentStatus>, Query),
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<Vec<PhysicalTransferResponse>>), (status = 400))
)]
#[axum::debug_handler]
async fn physical_transfer_query(
  State(state): State<Arc<ApiState>>,
  Query(query): Query<PhysicalTransferDocumentQueryParams>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<Vec<PhysicalTransferResponse>> {
  let query = PhysicalTransferQuerySpec::from(query);
  let rows = if embed.wants_names() {
    state
      .svc
      .document
      .physical_transfer_composite_query_with_names(query)
      .await?
  } else {
    state
      .svc
      .document
      .physical_transfer_composite_query(query)
      .await?
  };
  Ok(ApiResponse::success(rows))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "physical_transfer_create",
  summary = "Create physical transfer draft",
  description = "Creates a physical transfer aggregate in draft state.",
  path = paths::operations::PHYSICAL_TRANSFERS_SAVE,
  request_body = CreatePhysicalTransferRequest,
  responses((status = 200, body = ApiResponse<PhysicalTransferResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn physical_transfer_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreatePhysicalTransferRequest>>,
) -> ApiResult<PhysicalTransferResponse> {
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .physical_transfer_composite_create(&req)
      .await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "physical_transfer_create_and_execute",
  summary = "Create and execute physical transfer",
  description = "Creates and executes a physical transfer in one transactional flow. Requires supervisor or higher role.",
  path = paths::operations::PHYSICAL_TRANSFERS_SAVE_AND_EXECUTE,
  request_body = CreatePhysicalTransferRequest,
  responses(
    (status = 200, body = ApiResponse<PhysicalTransferResponse>, description = "Create+execute success envelope. Example: {\"success\":true,\"data\":{\"id\":\"...\",\"status\":\"Executed\",\"items\":[...]}}"),
    (status = 400, description = "Validation or bad request envelope. Example: {\"success\":false,\"error\":{\"code\":\"VALIDATION_ERROR\",\"message\":\"Validation error: ...\"}}"),
    (status = 403, description = "Forbidden envelope for insufficient role. Example: {\"success\":false,\"error\":{\"code\":\"FORBIDDEN\",\"message\":\"Forbidden: ...\"}}"),
    (status = 409, description = "Conflict envelope for domain constraints. Example: {\"success\":false,\"error\":{\"code\":\"CONFLICT\",\"message\":\"Conflict: ...\"}}")
  )
)]
#[axum::debug_handler]
async fn physical_transfer_create_and_execute(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
  Valid(Json(req)): Valid<Json<CreatePhysicalTransferRequest>>,
) -> ApiResult<PhysicalTransferResponse> {
  ensure_supervisor_or_higher(&claims.role)?;
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .physical_transfer_composite_create_and_execute(&req, claims.uid)
      .await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "physical_transfer_execute",
  summary = "Execute physical transfer",
  description = "Executes a draft physical transfer. Requires supervisor or higher role.",
  path = paths::operations::PHYSICAL_TRANSFERS_EXECUTE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, description = "Physical transfer executed"), (status = 403), (status = 404), (status = 409))
)]
#[axum::debug_handler]
async fn physical_transfer_execute(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  ensure_supervisor_or_higher(&claims.role)?;
  state
    .svc
    .document
    .physical_transfer_execute(id, claims.uid)
    .await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "physical_transfer_revert",
  summary = "Revert physical transfer",
  description = "Reverts an executed physical transfer back to draft. Requires senior supervisor or higher role.",
  path = paths::operations::PHYSICAL_TRANSFERS_REVERT_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, description = "Physical transfer reverted"), (status = 403), (status = 404), (status = 409))
)]
#[axum::debug_handler]
async fn physical_transfer_revert(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  ensure_senior_supervisor_or_higher(&claims.role)?;
  state
    .svc
    .document
    .physical_transfer_revert(id, claims.uid)
    .await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn transfer_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(physical_transfer_list, physical_transfer_create))
    .routes(routes!(physical_transfer_composite_get))
    .routes(routes!(physical_transfer_query))
    .routes(routes!(physical_transfer_create_and_execute))
    .routes(routes!(physical_transfer_execute))
    .routes(routes!(physical_transfer_revert))
    .with_state(state)
}
