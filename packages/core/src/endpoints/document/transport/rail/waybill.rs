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
  dtos::{
    CreateRailWaybillRequest,
    EmbedParams,
    PaginationParams,
    RailWaybillDocumentQueryParams,
    RailWaybillResponse,
    UpdateRailWaybillRequest,
  },
  endpoints::paths,
  services::document::specs::RailWaybillQuerySpec,
};

#[utoipa::path(
  get,
  tag = "Document - Transport",
  operation_id = "transport_rail_waybill_list",
  summary = "List rail waybills",
  description = "Returns rail waybill headers. Supports pagination.",
  path = paths::transport::rail::WAYBILLS,
  params(
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<Vec<RailWaybillResponse>>))
)]
#[axum::debug_handler]
async fn rail_waybill_list(
  State(state): State<Arc<ApiState>>,
  Query(pagination): Query<PaginationParams>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<Vec<RailWaybillResponse>> {
  let query = RailWaybillQuerySpec::list(pagination.page, pagination.per_page);
  let rows = if embed.wants_names() {
    state
      .svc
      .document
      .rail_waybill_query_with_names(query)
      .await?
  } else {
    state.svc.document.rail_waybill_query(query).await?
  };
  Ok(ApiResponse::success(rows))
}

#[utoipa::path(
  get,
  tag = "Document - Transport",
  operation_id = "transport_rail_waybill_query",
  summary = "Query rail waybills",
  description = "Queries rail waybills by optional filters.",
  path = paths::transport::rail::WAYBILLS_QUERY,
  params(
    ("documentNumber" = Option<String>, Query),
    ("senderId" = Option<Uuid>, Query),
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<Vec<RailWaybillResponse>>), (status = 400))
)]
#[axum::debug_handler]
async fn rail_waybill_query(
  State(state): State<Arc<ApiState>>,
  Query(query): Query<RailWaybillDocumentQueryParams>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<Vec<RailWaybillResponse>> {
  let query = RailWaybillQuerySpec::from(query);
  let rows = if embed.wants_names() {
    state
      .svc
      .document
      .rail_waybill_query_with_names(query)
      .await?
  } else {
    state.svc.document.rail_waybill_query(query).await?
  };
  Ok(ApiResponse::success(rows))
}

#[utoipa::path(
  post,
  tag = "Document - Transport",
  operation_id = "transport_rail_waybill_create",
  summary = "Create rail waybill",
  description = "Creates a rail waybill header.",
  path = paths::transport::rail::WAYBILLS,
  request_body = CreateRailWaybillRequest,
  responses((status = 200, body = ApiResponse<RailWaybillResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn rail_waybill_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateRailWaybillRequest>>,
) -> ApiResult<RailWaybillResponse> {
  Ok(ApiResponse::success(
    state.svc.document.rail_waybill_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Document - Transport",
  operation_id = "transport_rail_waybill_get",
  summary = "Get rail waybill",
  path = paths::transport::rail::WAYBILLS_BY_ID,
  params(
    ("id" = Uuid, Path),
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<RailWaybillResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn rail_waybill_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<RailWaybillResponse> {
  let row = if embed.wants_names() {
    state.svc.document.rail_waybill_get_with_names(id).await?
  } else {
    state.svc.document.rail_waybill_get(id).await?
  };
  Ok(ApiResponse::success(row))
}

#[utoipa::path(
  put,
  tag = "Document - Transport",
  operation_id = "transport_rail_waybill_update",
  summary = "Update rail waybill",
  path = paths::transport::rail::WAYBILLS_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateRailWaybillRequest,
  responses((status = 200, body = ApiResponse<RailWaybillResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn rail_waybill_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateRailWaybillRequest>>,
) -> ApiResult<RailWaybillResponse> {
  Ok(ApiResponse::success(
    state.svc.document.rail_waybill_update(id, &req).await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Document - Transport",
  operation_id = "transport_rail_waybill_soft_delete",
  summary = "Soft delete rail waybill",
  path = paths::transport::rail::WAYBILLS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn rail_waybill_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.rail_waybill_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Document - Transport",
  operation_id = "transport_rail_waybill_hard_delete",
  summary = "Hard delete rail waybill",
  path = paths::transport::rail::WAYBILLS_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn rail_waybill_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.rail_waybill_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn waybill_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(rail_waybill_list, rail_waybill_create))
    .routes(routes!(rail_waybill_query))
    .routes(routes!(rail_waybill_get, rail_waybill_update))
    .routes(routes!(rail_waybill_soft_delete))
    .routes(routes!(rail_waybill_hard_delete))
    .with_state(state)
}
