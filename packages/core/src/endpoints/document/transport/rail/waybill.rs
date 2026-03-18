use super::*;

#[utoipa::path(
  get,
  tag = "Document - Transport",
  operation_id = "transport_rail_waybill_list",
  summary = "List rail waybills",
  description = "Returns rail waybill headers. Supports query filtering and pagination through common entity query params.",
  path = paths::transport::rail::WAYBILLS,
  responses((status = 200, body = ApiResponse<Vec<RailWaybillResponse>>))
)]
#[axum::debug_handler]
async fn rail_waybill_list(
  State(state): State<Arc<ApiState>>,
) -> ApiResult<Vec<RailWaybillResponse>> {
  Ok(ApiResponse::success(state.svc.document.rail_waybill_list().await?))
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
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<RailWaybillResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn rail_waybill_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<RailWaybillResponse> {
  Ok(ApiResponse::success(state.svc.document.rail_waybill_get(id).await?))
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
    .routes(routes!(rail_waybill_get, rail_waybill_update))
    .routes(routes!(rail_waybill_soft_delete))
    .routes(routes!(rail_waybill_hard_delete))
    .with_state(state)
}
