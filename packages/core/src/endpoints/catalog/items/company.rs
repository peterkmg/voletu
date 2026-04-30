use std::sync::Arc;

use axum::{
  extract::{Path, Query, State},
  Json,
};
use axum_valid::Valid;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::{CompanyResponse, CreateCompanyRequest, PaginationParams, UpdateCompanyRequest},
  endpoints::paths,
  services::common::normalize_pagination,
};

#[utoipa::path(
  get,
  tag = "Catalog",
  operation_id = "catalog_company_list",
  summary = "List companies",
  description = "Returns catalog companies. Supports optional in-memory query filtering via q, field filters, and pagination query params.",
  path = paths::catalog::COMPANIES,
  params(
    ("page" = Option<u64>, Query, description = "Page number (1-based)"),
    ("per_page" = Option<u64>, Query, description = "Items per page"),
  ),
  responses((status = 200, body = ApiResponse<Vec<CompanyResponse>>))
)]
#[axum::debug_handler]
async fn company_list(
  State(state): State<Arc<ApiState>>,
  Query(pagination): Query<PaginationParams>,
) -> ApiResult<Vec<CompanyResponse>> {
  let pg = if pagination.page.is_some() || pagination.per_page.is_some() {
    Some(normalize_pagination(pagination.page, pagination.per_page)?)
  } else {
    None
  };
  Ok(ApiResponse::success(
    state.svc.catalog_service.company_list(pg).await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Catalog",
  operation_id = "catalog_company_create",
  summary = "Create company",
  description = "Creates a new company reference row used by document and operations flows.",
  path = paths::catalog::COMPANIES,
  request_body = CreateCompanyRequest,
  responses((status = 200, body = ApiResponse<CompanyResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn company_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateCompanyRequest>>,
) -> ApiResult<CompanyResponse> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.company_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Catalog",
  operation_id = "catalog_company_get",
  summary = "Get company",
  path = paths::catalog::COMPANIES_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<CompanyResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn company_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<CompanyResponse> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.company_get(id).await?,
  ))
}

#[utoipa::path(
  put,
  tag = "Catalog",
  operation_id = "catalog_company_update",
  summary = "Update company",
  path = paths::catalog::COMPANIES_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateCompanyRequest,
  responses((status = 200, body = ApiResponse<CompanyResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn company_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateCompanyRequest>>,
) -> ApiResult<CompanyResponse> {
  Ok(ApiResponse::success(
    state.svc.catalog_service.company_update(id, &req).await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Catalog",
  operation_id = "catalog_company_soft_delete",
  summary = "Soft delete company",
  path = paths::catalog::COMPANIES_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn company_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.catalog_service.company_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Catalog",
  operation_id = "catalog_company_hard_delete",
  summary = "Hard delete company",
  path = paths::catalog::COMPANIES_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404), (status = 409))
)]
#[axum::debug_handler]
async fn company_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.catalog_service.company_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  post,
  tag = "Catalog",
  operation_id = "catalog_company_restore",
  summary = "Restore soft-deleted company",
  path = paths::catalog::COMPANIES_RESTORE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn company_restore(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state
    .svc
    .catalog_service
    .company_soft_delete_undo(id)
    .await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn company_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(company_list, company_create))
    .routes(routes!(company_get, company_update))
    .routes(routes!(company_soft_delete))
    .routes(routes!(company_hard_delete))
    .routes(routes!(company_restore))
    .with_state(state)
}
