use super::*;

#[utoipa::path(
  get,
  tag = "Document - Transport",
  operation_id = "transport_rail_measurement_list",
  summary = "List rail measurements",
  description = "Returns rail wagon measurement rows.",
  path = paths::transport::rail::MEASUREMENTS,
  responses((status = 200, body = ApiResponse<Vec<RailWagonMeasurementResponse>>))
)]
#[axum::debug_handler]
async fn rail_measurement_list(
  State(state): State<Arc<ApiState>>,
) -> ApiResult<Vec<RailWagonMeasurementResponse>> {
  Ok(ApiResponse::success(
    state.svc.document.rail_measurement_list(None).await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Document - Transport",
  operation_id = "transport_rail_measurement_create",
  summary = "Create rail measurement",
  description = "Creates a wagon measurement under an existing wagon manifest.",
  path = paths::transport::rail::MEASUREMENTS,
  request_body = CreateRailWagonMeasurementRequest,
  responses((status = 200, body = ApiResponse<RailWagonMeasurementResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn rail_measurement_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateRailWagonMeasurementRequest>>,
) -> ApiResult<RailWagonMeasurementResponse> {
  Ok(ApiResponse::success(
    state.svc.document.rail_measurement_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Document - Transport",
  operation_id = "transport_rail_measurement_get",
  summary = "Get rail measurement",
  path = paths::transport::rail::MEASUREMENTS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<RailWagonMeasurementResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn rail_measurement_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<RailWagonMeasurementResponse> {
  Ok(ApiResponse::success(
    state.svc.document.rail_measurement_get(id).await?,
  ))
}

#[utoipa::path(
  put,
  tag = "Document - Transport",
  operation_id = "transport_rail_measurement_update",
  summary = "Update rail measurement",
  path = paths::transport::rail::MEASUREMENTS_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateRailWagonMeasurementRequest,
  responses((status = 200, body = ApiResponse<RailWagonMeasurementResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn rail_measurement_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateRailWagonMeasurementRequest>>,
) -> ApiResult<RailWagonMeasurementResponse> {
  Ok(ApiResponse::success(
    state.svc.document.rail_measurement_update(id, &req).await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Document - Transport",
  operation_id = "transport_rail_measurement_soft_delete",
  summary = "Soft delete rail measurement",
  path = paths::transport::rail::MEASUREMENTS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn rail_measurement_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.rail_measurement_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Document - Transport",
  operation_id = "transport_rail_measurement_hard_delete",
  summary = "Hard delete rail measurement",
  path = paths::transport::rail::MEASUREMENTS_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn rail_measurement_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.rail_measurement_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn measurement_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(rail_measurement_list, rail_measurement_create))
    .routes(routes!(rail_measurement_get, rail_measurement_update))
    .routes(routes!(rail_measurement_soft_delete))
    .routes(routes!(rail_measurement_hard_delete))
    .with_state(state)
}
