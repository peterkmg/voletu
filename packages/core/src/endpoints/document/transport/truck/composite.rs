use super::*;

#[utoipa::path(
  get,
  tag = "Document - Transport",
  operation_id = "transport_truck_waybill_composite_get",
  summary = "Get truck waybill composite",
  description = "Returns a truck waybill with nested items and weight docs.",
  path = paths::transport::truck::COMPOSITE_BY_ID,
  params(
    ("id" = Uuid, Path),
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<TruckWaybillCompositeResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn truck_waybill_composite_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<TruckWaybillCompositeResponse> {
  let row = if embed.wants_names() {
    state
      .svc
      .document
      .truck_waybill_composite_get_with_names(id)
      .await?
  } else {
    state.svc.document.truck_waybill_composite_get(id).await?
  };
  Ok(ApiResponse::success(row))
}

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

#[utoipa::path(
  put,
  tag = "Document - Transport",
  operation_id = "transport_truck_waybill_composite_update",
  summary = "Update truck waybill composite",
  description = "Updates a truck waybill aggregate. Header fields apply set_if_some semantics; the items list is treated as the full new state and is diff-applied (insert / update / delete).",
  path = paths::transport::truck::COMPOSITE_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateTruckWaybillCompositeRequest,
  responses(
    (status = 200, body = ApiResponse<TruckWaybillCompositeResponse>),
    (status = 400),
    (status = 404)
  )
)]
#[axum::debug_handler]
async fn truck_waybill_composite_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateTruckWaybillCompositeRequest>>,
) -> ApiResult<TruckWaybillCompositeResponse> {
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .truck_waybill_composite_update(id, &req)
      .await?,
  ))
}

pub(super) fn composite_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(
      truck_waybill_composite_get,
      truck_waybill_composite_create,
      truck_waybill_composite_update
    ))
    .with_state(state)
}
