use super::*;

#[utoipa::path(
  get,
  tag = "Document - Transport",
  operation_id = "transport_truck_waybill_item_list",
  summary = "List truck waybill items",
  description = "Returns truck waybill line items.",
  path = paths::transport::truck::ITEMS,
  responses((status = 200, body = ApiResponse<Vec<TruckWaybillItemResponse>>))
)]
#[axum::debug_handler]
async fn truck_waybill_item_list(
  State(state): State<Arc<ApiState>>,
) -> ApiResult<Vec<TruckWaybillItemResponse>> {
  Ok(ApiResponse::success(
    state.svc.document.truck_waybill_item_list().await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Document - Transport",
  operation_id = "transport_truck_waybill_item_create",
  summary = "Create truck waybill item",
  description = "Creates a line item under an existing truck waybill.",
  path = paths::transport::truck::ITEMS,
  request_body = CreateTruckWaybillItemRequest,
  responses((status = 200, body = ApiResponse<TruckWaybillItemResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn truck_waybill_item_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateTruckWaybillItemRequest>>,
) -> ApiResult<TruckWaybillItemResponse> {
  Ok(ApiResponse::success(
    state.svc.document.truck_waybill_item_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Document - Transport",
  operation_id = "transport_truck_waybill_item_get",
  summary = "Get truck waybill item",
  path = paths::transport::truck::ITEMS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<TruckWaybillItemResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn truck_waybill_item_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<TruckWaybillItemResponse> {
  Ok(ApiResponse::success(
    state.svc.document.truck_waybill_item_get(id).await?,
  ))
}

#[utoipa::path(
  put,
  tag = "Document - Transport",
  operation_id = "transport_truck_waybill_item_update",
  summary = "Update truck waybill item",
  path = paths::transport::truck::ITEMS_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateTruckWaybillItemRequest,
  responses((status = 200, body = ApiResponse<TruckWaybillItemResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn truck_waybill_item_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateTruckWaybillItemRequest>>,
) -> ApiResult<TruckWaybillItemResponse> {
  Ok(ApiResponse::success(
    state.svc.document.truck_waybill_item_update(id, &req).await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Document - Transport",
  operation_id = "transport_truck_waybill_item_soft_delete",
  summary = "Soft delete truck waybill item",
  path = paths::transport::truck::ITEMS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn truck_waybill_item_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.truck_waybill_item_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Document - Transport",
  operation_id = "transport_truck_waybill_item_hard_delete",
  summary = "Hard delete truck waybill item",
  path = paths::transport::truck::ITEMS_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn truck_waybill_item_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.truck_waybill_item_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn item_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(truck_waybill_item_list, truck_waybill_item_create))
    .routes(routes!(truck_waybill_item_get, truck_waybill_item_update))
    .routes(routes!(truck_waybill_item_soft_delete))
    .routes(routes!(truck_waybill_item_hard_delete))
    .with_state(state)
}
