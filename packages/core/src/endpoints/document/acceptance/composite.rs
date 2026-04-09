use std::sync::Arc;

use axum::{
  extract::{Path, Query, State},
  Extension, Json,
};
use axum_valid::Valid;
use uuid::Uuid;

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::{AcceptanceCompositeResponse, CreateAcceptanceCompositeRequest},
  endpoints::{paths, query::EmbedParams},
  services::common::ensure_supervisor_or_higher,
  utils::jwt::Claims,
};

#[utoipa::path(
  get,
  tag = "Document - Acceptance",
  operation_id = "acceptance_composite_get",
  summary = "Get acceptance composite",
  path = paths::acceptance::COMPOSITE_BY_ID,
  params(
    ("id" = Uuid, Path),
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<AcceptanceCompositeResponse>), (status = 404))
)]
#[axum::debug_handler]
pub(super) async fn acceptance_composite_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<AcceptanceCompositeResponse> {
  let row = if embed.wants_names() {
    state
      .svc
      .document
      .acceptance_composite_get_with_names(id)
      .await?
  } else {
    state.svc.document.acceptance_composite_get(id).await?
  };
  Ok(ApiResponse::success(row))
}

#[utoipa::path(
  post,
  tag = "Document - Acceptance",
  operation_id = "acceptance_composite_create",
  summary = "Create acceptance composite",
  path = paths::acceptance::COMPOSITE_SAVE,
  request_body = CreateAcceptanceCompositeRequest,
  responses((status = 200, body = ApiResponse<AcceptanceCompositeResponse>), (status = 400))
)]
#[axum::debug_handler]
pub(super) async fn acceptance_composite_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateAcceptanceCompositeRequest>>,
) -> ApiResult<AcceptanceCompositeResponse> {
  Ok(ApiResponse::success(
    state.svc.document.acceptance_composite_create(&req).await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Document - Acceptance",
  operation_id = "acceptance_composite_create_and_execute",
  summary = "Create and execute acceptance composite",
  path = paths::acceptance::COMPOSITE_SAVE_AND_EXECUTE,
  request_body = CreateAcceptanceCompositeRequest,
  responses((status = 200, body = ApiResponse<AcceptanceCompositeResponse>), (status = 400), (status = 409))
)]
#[axum::debug_handler]
pub(super) async fn acceptance_composite_create_and_execute(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
  Valid(Json(req)): Valid<Json<CreateAcceptanceCompositeRequest>>,
) -> ApiResult<AcceptanceCompositeResponse> {
  ensure_supervisor_or_higher(&claims.role)?;
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .acceptance_composite_create_and_execute(&req, claims.uid)
      .await?,
  ))
}
