use std::sync::Arc;

use axum::{
  extract::{Path, Query, State},
  Extension,
  Json,
};
use axum_valid::Valid;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::{AcceptanceResponse, CreateAcceptanceRequest},
  endpoints::{
    paths,
    query::{EmbedParams, NullableFilter, PaginationParams},
  },
  enums,
  services::common::{ensure_senior_supervisor_or_higher, ensure_supervisor_or_higher},
  utils::jwt::Claims,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct AcceptanceQueryParams {
  pub(super) document_number: Option<String>,
  pub(super) status: Option<enums::DocumentStatus>,
  pub(super) truck_waybill_id: Option<NullableFilter>,
  pub(super) rail_waybill_id: Option<NullableFilter>,
  pub(super) transit_dispatch_id: Option<NullableFilter>,
  #[serde(flatten)]
  pub(super) pagination: PaginationParams,
}

#[utoipa::path(
  get,
  tag = "Document - Acceptance",
  operation_id = "acceptance_document_list",
  summary = "List acceptance documents",
  path = paths::acceptance::ROOT,
  params(
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<Vec<AcceptanceResponse>>))
)]
#[axum::debug_handler]
pub(super) async fn acceptance_document_list(
  State(state): State<Arc<ApiState>>,
  Query(pagination): Query<PaginationParams>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<Vec<AcceptanceResponse>> {
  let rows = if embed.wants_names() {
    state
      .svc
      .document
      .acceptance_document_query_with_names(
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
      .acceptance_document_query(
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
  tag = "Document - Acceptance",
  operation_id = "acceptance_document_create",
  summary = "Create acceptance draft",
  path = paths::acceptance::SAVE,
  request_body = CreateAcceptanceRequest,
  responses((status = 200, body = ApiResponse<AcceptanceResponse>), (status = 400))
)]
#[axum::debug_handler]
pub(super) async fn acceptance_document_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateAcceptanceRequest>>,
) -> ApiResult<AcceptanceResponse> {
  Ok(ApiResponse::success(
    state.svc.document.acceptance_document_create(&req).await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Document - Acceptance",
  operation_id = "acceptance_document_create_and_execute",
  summary = "Create and execute acceptance document",
  path = paths::acceptance::SAVE_AND_EXECUTE,
  request_body = CreateAcceptanceRequest,
  responses((status = 200, body = ApiResponse<AcceptanceResponse>), (status = 400), (status = 403), (status = 409))
)]
#[axum::debug_handler]
pub(super) async fn acceptance_document_create_and_execute(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
  Valid(Json(req)): Valid<Json<CreateAcceptanceRequest>>,
) -> ApiResult<AcceptanceResponse> {
  ensure_supervisor_or_higher(&claims.role)?;
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .acceptance_document_create_and_execute(&req, claims.uid)
      .await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Document - Acceptance",
  operation_id = "acceptance_document_query",
  summary = "Query acceptance documents",
  path = paths::acceptance::QUERY,
  params(
    ("documentNumber" = Option<String>, Query),
    ("status" = Option<enums::DocumentStatus>, Query),
    ("truckWaybillId" = Option<String>, Query, description = "Pass 'isNull' or 'isNotNull'"),
    ("railWaybillId" = Option<String>, Query, description = "Pass 'isNull' or 'isNotNull'"),
    ("transitDispatchId" = Option<String>, Query, description = "Pass 'isNull' or 'isNotNull'"),
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<Vec<AcceptanceResponse>>), (status = 400))
)]
#[axum::debug_handler]
pub(super) async fn acceptance_document_query(
  State(state): State<Arc<ApiState>>,
  Query(query): Query<AcceptanceQueryParams>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<Vec<AcceptanceResponse>> {
  let rows = if embed.wants_names() {
    state
      .svc
      .document
      .acceptance_document_query_with_names(
        query.document_number.as_deref(),
        query.status,
        query.truck_waybill_id,
        query.rail_waybill_id,
        query.transit_dispatch_id,
        query.pagination.page,
        query.pagination.per_page,
      )
      .await?
  } else {
    state
      .svc
      .document
      .acceptance_document_query(
        query.document_number.as_deref(),
        query.status,
        query.truck_waybill_id,
        query.rail_waybill_id,
        query.transit_dispatch_id,
        query.pagination.page,
        query.pagination.per_page,
      )
      .await?
  };
  Ok(ApiResponse::success(rows))
}

#[utoipa::path(
  get,
  tag = "Document - Acceptance",
  operation_id = "acceptance_document_get",
  summary = "Get acceptance document",
  path = paths::acceptance::BY_ID,
  params(
    ("id" = Uuid, Path),
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<AcceptanceResponse>), (status = 404))
)]
#[axum::debug_handler]
pub(super) async fn acceptance_document_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<AcceptanceResponse> {
  let row = if embed.wants_names() {
    state
      .svc
      .document
      .acceptance_document_get_with_names(id)
      .await?
  } else {
    state.svc.document.acceptance_document_get(id).await?
  };
  Ok(ApiResponse::success(row))
}

#[utoipa::path(
  put,
  tag = "Document - Acceptance",
  operation_id = "acceptance_document_update",
  summary = "Update acceptance document",
  path = paths::acceptance::BY_ID,
  params(("id" = Uuid, Path)),
  request_body = crate::dtos::UpdateAcceptanceRequest,
  responses((status = 200, body = ApiResponse<AcceptanceResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
pub(super) async fn acceptance_document_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<crate::dtos::UpdateAcceptanceRequest>>,
) -> ApiResult<AcceptanceResponse> {
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .acceptance_document_update(id, &req)
      .await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Document - Acceptance",
  operation_id = "acceptance_document_soft_delete",
  summary = "Soft delete acceptance document",
  path = paths::acceptance::BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
pub(super) async fn acceptance_document_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state
    .svc
    .document
    .acceptance_document_soft_delete(id)
    .await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Document - Acceptance",
  operation_id = "acceptance_document_hard_delete",
  summary = "Hard delete acceptance document",
  path = paths::acceptance::HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
pub(super) async fn acceptance_document_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state
    .svc
    .document
    .acceptance_document_hard_delete(id)
    .await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  post,
  tag = "Document - Acceptance",
  operation_id = "acceptance_document_execute",
  summary = "Execute acceptance document",
  path = paths::acceptance::EXECUTE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 403), (status = 404), (status = 409))
)]
#[axum::debug_handler]
pub(super) async fn acceptance_document_execute(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  ensure_supervisor_or_higher(&claims.role)?;
  state
    .svc
    .document
    .acceptance_document_execute(id, claims.uid)
    .await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  post,
  tag = "Document - Acceptance",
  operation_id = "acceptance_document_revert",
  summary = "Revert acceptance document",
  path = paths::acceptance::REVERT_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 403), (status = 404), (status = 409))
)]
#[axum::debug_handler]
pub(super) async fn acceptance_document_revert(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  ensure_senior_supervisor_or_higher(&claims.role)?;
  state
    .svc
    .document
    .acceptance_document_revert(id, claims.uid)
    .await?;
  Ok(ApiResponse::success(()))
}
