use super::*;

#[utoipa::path(
  get,
  tag = "Document - Transport",
  operation_id = "transport_truck_weight_doc_list",
  summary = "List truck weight docs",
  description = "Returns truck weight documents associated with waybills.",
  path = paths::transport::truck::WEIGHT_DOCS,
  responses((status = 200, body = ApiResponse<Vec<TruckWeightDocResponse>>))
)]
#[axum::debug_handler]
async fn truck_weight_doc_list(
  State(state): State<Arc<ApiState>>,
) -> ApiResult<Vec<TruckWeightDocResponse>> {
  Ok(ApiResponse::success(
    state.svc.document.truck_weight_doc_list().await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Document - Transport",
  operation_id = "transport_truck_weight_doc_create",
  summary = "Create truck weight doc",
  description = "Creates a truck weight document for an existing truck waybill.",
  path = paths::transport::truck::WEIGHT_DOCS,
  request_body = CreateTruckWeightDocRequest,
  responses((status = 200, body = ApiResponse<TruckWeightDocResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn truck_weight_doc_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateTruckWeightDocRequest>>,
) -> ApiResult<TruckWeightDocResponse> {
  Ok(ApiResponse::success(
    state.svc.document.truck_weight_doc_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Document - Transport",
  operation_id = "transport_truck_weight_doc_get",
  summary = "Get truck weight doc",
  path = paths::transport::truck::WEIGHT_DOCS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<TruckWeightDocResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn truck_weight_doc_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<TruckWeightDocResponse> {
  Ok(ApiResponse::success(
    state.svc.document.truck_weight_doc_get(id).await?,
  ))
}

#[utoipa::path(
  put,
  tag = "Document - Transport",
  operation_id = "transport_truck_weight_doc_update",
  summary = "Update truck weight doc",
  path = paths::transport::truck::WEIGHT_DOCS_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateTruckWeightDocRequest,
  responses((status = 200, body = ApiResponse<TruckWeightDocResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn truck_weight_doc_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateTruckWeightDocRequest>>,
) -> ApiResult<TruckWeightDocResponse> {
  Ok(ApiResponse::success(
    state.svc.document.truck_weight_doc_update(id, &req).await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Document - Transport",
  operation_id = "transport_truck_weight_doc_soft_delete",
  summary = "Soft delete truck weight doc",
  path = paths::transport::truck::WEIGHT_DOCS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn truck_weight_doc_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.truck_weight_doc_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Document - Transport",
  operation_id = "transport_truck_weight_doc_hard_delete",
  summary = "Hard delete truck weight doc",
  path = paths::transport::truck::WEIGHT_DOCS_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn truck_weight_doc_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.truck_weight_doc_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn weight_doc_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(truck_weight_doc_list, truck_weight_doc_create))
    .routes(routes!(truck_weight_doc_get, truck_weight_doc_update))
    .routes(routes!(truck_weight_doc_soft_delete))
    .routes(routes!(truck_weight_doc_hard_delete))
    .with_state(state)
}
