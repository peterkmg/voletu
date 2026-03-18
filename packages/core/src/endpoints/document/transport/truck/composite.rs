use super::*;

#[utoipa::path(
  post,
  tag = "Document - Transport",
  operation_id = "transport_truck_waybill_composite_create",
  summary = "Create truck intake composite",
  description = "Creates a truck waybill aggregate in one request (waybill plus optional items and weight docs). This endpoint is create-only and does not execute ledger-affecting flows.",
  path = paths::transport::truck::COMPOSITE_CREATE,
  request_body = TruckWaybillCompositeRequest,
  responses((status = 200, body = ApiResponse<TruckWaybillCompositeResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn truck_waybill_composite_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<TruckWaybillCompositeRequest>>,
) -> ApiResult<TruckWaybillCompositeResponse> {
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .truck_waybill_composite_create(&req)
      .await?,
  ))
}

pub(super) fn composite_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(truck_waybill_composite_create))
    .with_state(state)
}
