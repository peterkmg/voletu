use super::*;

#[utoipa::path(
  get,
  tag = "Document - Transport",
  operation_id = "transport_truck_waybill_list",
  summary = "List truck waybills",
  description = "Returns truck waybill headers. Supports query filtering and pagination through common entity query params.",
  path = paths::transport::truck::WAYBILLS,
  responses((status = 200, body = ApiResponse<Vec<TruckWaybillResponse>>))
)]
#[axum::debug_handler]
async fn truck_waybill_list(
  State(state): State<Arc<ApiState>>,
) -> ApiResult<Vec<TruckWaybillResponse>> {
  Ok(ApiResponse::success(
    state.svc.document.truck_waybill_list().await?,
  ))
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
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<TruckWaybillResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn truck_waybill_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<TruckWaybillResponse> {
  Ok(ApiResponse::success(
    state.svc.document.truck_waybill_get(id).await?,
  ))
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
    .routes(routes!(truck_waybill_get, truck_waybill_update))
    .routes(routes!(truck_waybill_soft_delete))
    .routes(routes!(truck_waybill_hard_delete))
    .with_state(state)
}
