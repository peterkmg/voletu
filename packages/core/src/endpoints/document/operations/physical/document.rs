use std::sync::Arc;

use axum::{
  extract::{Path, Query, State},
  Extension,
  Json,
};
use axum_valid::Valid;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::{
    CreatePhysicalTransferRequest,
    EmbedParams,
    PhysicalTransferResponse,
    UpdatePhysicalTransferRequest,
  },
  endpoints::paths,
  services::common::{ensure_senior_supervisor_or_higher, ensure_supervisor_or_higher},
  utils::jwt::Claims,
};

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "physical_document_list",
  path = paths::operations::PHYSICAL_TRANSFER_DOCUMENTS,
  params(
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<Vec<PhysicalTransferResponse>>))
)]
#[axum::debug_handler]
async fn physical_document_list(
  State(state): State<Arc<ApiState>>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<Vec<PhysicalTransferResponse>> {
  let rows = if embed.wants_names() {
    state
      .svc
      .document
      .physical_transfer_list_with_names()
      .await?
  } else {
    state.svc.document.physical_transfer_list(None).await?
  };
  Ok(ApiResponse::success(rows))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "physical_document_create",
  path = paths::operations::PHYSICAL_TRANSFER_DOCUMENTS,
  request_body = CreatePhysicalTransferRequest,
  responses((status = 200, body = ApiResponse<PhysicalTransferResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn physical_document_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreatePhysicalTransferRequest>>,
) -> ApiResult<PhysicalTransferResponse> {
  Ok(ApiResponse::success(
    state.svc.document.physical_transfer_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Document - Operations",
  operation_id = "physical_document_get",
  path = paths::operations::PHYSICAL_TRANSFER_DOCUMENTS_BY_ID,
  params(
    ("id" = Uuid, Path),
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<PhysicalTransferResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn physical_document_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<PhysicalTransferResponse> {
  let row = if embed.wants_names() {
    state
      .svc
      .document
      .physical_transfer_get_with_names(id)
      .await?
  } else {
    state.svc.document.physical_transfer_get(id).await?
  };
  Ok(ApiResponse::success(row))
}

#[utoipa::path(
  put,
  tag = "Document - Operations",
  operation_id = "physical_document_update",
  path = paths::operations::PHYSICAL_TRANSFER_DOCUMENTS_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdatePhysicalTransferRequest,
  responses((status = 200, body = ApiResponse<PhysicalTransferResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn physical_document_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdatePhysicalTransferRequest>>,
) -> ApiResult<PhysicalTransferResponse> {
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .physical_transfer_update(id, &req)
      .await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Document - Operations",
  operation_id = "physical_document_soft_delete",
  path = paths::operations::PHYSICAL_TRANSFER_DOCUMENTS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn physical_document_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.physical_transfer_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Document - Operations",
  operation_id = "physical_document_hard_delete",
  path = paths::operations::PHYSICAL_TRANSFER_DOCUMENTS_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn physical_document_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.physical_transfer_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "physical_document_create_and_execute",
  path = paths::operations::PHYSICAL_TRANSFER_DOCUMENTS_SAVE_AND_EXECUTE,
  request_body = CreatePhysicalTransferRequest,
  responses((status = 200, body = ApiResponse<PhysicalTransferResponse>), (status = 400), (status = 403), (status = 409))
)]
#[axum::debug_handler]
async fn physical_document_create_and_execute(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
  Valid(Json(req)): Valid<Json<CreatePhysicalTransferRequest>>,
) -> ApiResult<PhysicalTransferResponse> {
  ensure_supervisor_or_higher(&claims.role)?;
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .physical_transfer_create_and_execute(&req, claims.uid)
      .await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "physical_document_execute",
  path = paths::operations::PHYSICAL_TRANSFER_DOCUMENTS_EXECUTE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 403), (status = 404), (status = 409))
)]
#[axum::debug_handler]
async fn physical_document_execute(
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
  operation_id = "physical_document_revert",
  path = paths::operations::PHYSICAL_TRANSFER_DOCUMENTS_REVERT_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 403), (status = 404), (status = 409))
)]
#[axum::debug_handler]
async fn physical_document_revert(
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

pub(super) fn document_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(physical_document_list, physical_document_create))
    .routes(routes!(physical_document_get, physical_document_update))
    .routes(routes!(physical_document_soft_delete))
    .routes(routes!(physical_document_hard_delete))
    .routes(routes!(physical_document_create_and_execute))
    .routes(routes!(physical_document_execute))
    .routes(routes!(physical_document_revert))
    .with_state(state)
}
