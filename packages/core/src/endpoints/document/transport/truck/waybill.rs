use super::*;
use crate::{
  endpoints::query::TruckWaybillDocumentQueryParams,
  services::document::query::TruckWaybillQuerySpec,
};

#[utoipa::path(
  get,
  tag = "Document - Transport",
  operation_id = "transport_truck_waybill_list",
  summary = "List truck waybills",
  description = "Returns truck waybill headers. Supports pagination.",
  path = paths::transport::truck::WAYBILLS,
  params(
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<Vec<TruckWaybillResponse>>))
)]
#[axum::debug_handler]
async fn truck_waybill_list(
  State(state): State<Arc<ApiState>>,
  Query(pagination): Query<PaginationParams>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<Vec<TruckWaybillResponse>> {
  let query = TruckWaybillQuerySpec::list(pagination.page, pagination.per_page);
  let rows = if embed.wants_names() {
    state
      .svc
      .document
      .truck_waybill_query_with_names(query)
      .await?
  } else {
    state.svc.document.truck_waybill_query(query).await?
  };
  Ok(ApiResponse::success(rows))
}

#[utoipa::path(
  get,
  tag = "Document - Transport",
  operation_id = "transport_truck_waybill_query",
  summary = "Query truck waybills",
  description = "Queries truck waybills by optional filters.",
  path = paths::transport::truck::WAYBILLS_QUERY,
  params(
    ("documentNumber" = Option<String>, Query),
    ("senderId" = Option<Uuid>, Query),
    ("page" = Option<u64>, Query),
    ("per_page" = Option<u64>, Query),
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<Vec<TruckWaybillResponse>>), (status = 400))
)]
#[axum::debug_handler]
async fn truck_waybill_query(
  State(state): State<Arc<ApiState>>,
  Query(query): Query<TruckWaybillDocumentQueryParams>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<Vec<TruckWaybillResponse>> {
  let query = TruckWaybillQuerySpec::from(query);
  let rows = if embed.wants_names() {
    state
      .svc
      .document
      .truck_waybill_query_with_names(query)
      .await?
  } else {
    state.svc.document.truck_waybill_query(query).await?
  };
  Ok(ApiResponse::success(rows))
}

#[utoipa::path(
  post,
  tag = "Document - Transport",
  operation_id = "transport_truck_waybill_create",
  summary = "Create truck waybill",
  description = "Creates a truck waybill header document.",
  path = paths::transport::truck::WAYBILLS,
  request_body = CreateTruckWaybillRequest,
  responses((status = 200, body = ApiResponse<TruckWaybillResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn truck_waybill_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateTruckWaybillRequest>>,
) -> ApiResult<TruckWaybillResponse> {
  Ok(ApiResponse::success(
    state.svc.document.truck_waybill_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Document - Transport",
  operation_id = "transport_truck_waybill_get",
  summary = "Get truck waybill",
  path = paths::transport::truck::WAYBILLS_BY_ID,
  params(
    ("id" = Uuid, Path),
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<TruckWaybillResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn truck_waybill_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<TruckWaybillResponse> {
  let row = if embed.wants_names() {
    state.svc.document.truck_waybill_get_with_names(id).await?
  } else {
    state.svc.document.truck_waybill_get(id).await?
  };
  Ok(ApiResponse::success(row))
}

#[utoipa::path(
  put,
  tag = "Document - Transport",
  operation_id = "transport_truck_waybill_update",
  summary = "Update truck waybill",
  path = paths::transport::truck::WAYBILLS_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateTruckWaybillRequest,
  responses((status = 200, body = ApiResponse<TruckWaybillResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn truck_waybill_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateTruckWaybillRequest>>,
) -> ApiResult<TruckWaybillResponse> {
  Ok(ApiResponse::success(
    state.svc.document.truck_waybill_update(id, &req).await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Document - Transport",
  operation_id = "transport_truck_waybill_soft_delete",
  summary = "Soft delete truck waybill",
  path = paths::transport::truck::WAYBILLS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn truck_waybill_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.truck_waybill_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Document - Transport",
  operation_id = "transport_truck_waybill_hard_delete",
  summary = "Hard delete truck waybill",
  path = paths::transport::truck::WAYBILLS_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn truck_waybill_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.truck_waybill_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn waybill_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(truck_waybill_list, truck_waybill_create))
    .routes(routes!(truck_waybill_query))
    .routes(routes!(truck_waybill_get, truck_waybill_update))
    .routes(routes!(truck_waybill_soft_delete))
    .routes(routes!(truck_waybill_hard_delete))
    .with_state(state)
}
