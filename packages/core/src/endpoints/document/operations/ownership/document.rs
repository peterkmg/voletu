use super::*;

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "ownership_document_list",
  path = paths::operations::OWNERSHIP_TRANSFER_DOCUMENTS,
  responses((status = 200, body = ApiResponse<Vec<OwnershipTransferResponse>>))
)]
#[axum::debug_handler]
async fn ownership_document_list(
  State(state): State<Arc<ApiState>>,
) -> ApiResult<Vec<OwnershipTransferResponse>> {
  let rows = state.svc.document.ownership_transfer_list().await?;
  Ok(ApiResponse::success(rows))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "ownership_document_create",
  path = paths::operations::OWNERSHIP_TRANSFER_DOCUMENTS,
  request_body = CreateOwnershipTransferRequest,
  responses((status = 200, body = ApiResponse<OwnershipTransferResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn ownership_document_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateOwnershipTransferRequest>>,
) -> ApiResult<OwnershipTransferResponse> {
  Ok(ApiResponse::success(
    state.svc.document.ownership_transfer_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "ownership_document_get",
  path = paths::operations::OWNERSHIP_TRANSFER_DOCUMENTS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<OwnershipTransferResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn ownership_document_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<OwnershipTransferResponse> {
  Ok(ApiResponse::success(
    state.svc.document.ownership_transfer_get(id).await?,
  ))
}

#[utoipa::path(
  put,
  tag = "Document - Operations",
  operation_id = "ownership_document_update",
  path = paths::operations::OWNERSHIP_TRANSFER_DOCUMENTS_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateOwnershipTransferRequest,
  responses((status = 200, body = ApiResponse<OwnershipTransferResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn ownership_document_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateOwnershipTransferRequest>>,
) -> ApiResult<OwnershipTransferResponse> {
  Ok(ApiResponse::success(
    state.svc.document.ownership_transfer_update(id, &req).await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Document - Operations",
  operation_id = "ownership_document_soft_delete",
  path = paths::operations::OWNERSHIP_TRANSFER_DOCUMENTS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn ownership_document_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.ownership_transfer_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Document - Operations",
  operation_id = "ownership_document_hard_delete",
  path = paths::operations::OWNERSHIP_TRANSFER_DOCUMENTS_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn ownership_document_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.ownership_transfer_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "ownership_document_create_and_execute",
  path = paths::operations::OWNERSHIP_TRANSFER_DOCUMENTS_SAVE_AND_EXECUTE,
  request_body = CreateOwnershipTransferRequest,
  responses((status = 200, body = ApiResponse<OwnershipTransferResponse>), (status = 400), (status = 403), (status = 409))
)]
#[axum::debug_handler]
async fn ownership_document_create_and_execute(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
  Valid(Json(req)): Valid<Json<CreateOwnershipTransferRequest>>,
) -> ApiResult<OwnershipTransferResponse> {
  ensure_supervisor_or_higher(&claims.role)?;
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .ownership_transfer_create_and_execute(&req, claims.uid)
      .await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "ownership_document_execute",
  path = paths::operations::OWNERSHIP_TRANSFER_DOCUMENTS_EXECUTE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 403), (status = 404), (status = 409))
)]
#[axum::debug_handler]
async fn ownership_document_execute(
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
  operation_id = "ownership_document_revert",
  path = paths::operations::OWNERSHIP_TRANSFER_DOCUMENTS_REVERT_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 403), (status = 404), (status = 409))
)]
#[axum::debug_handler]
async fn ownership_document_revert(
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

pub(super) fn document_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(ownership_document_list, ownership_document_create))
    .routes(routes!(ownership_document_get, ownership_document_update))
    .routes(routes!(ownership_document_soft_delete))
    .routes(routes!(ownership_document_hard_delete))
    .routes(routes!(ownership_document_create_and_execute))
    .routes(routes!(ownership_document_execute))
    .routes(routes!(ownership_document_revert))
    .with_state(state)
}
