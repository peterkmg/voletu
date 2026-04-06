use super::*;

#[utoipa::path(
  get,
  tag = "Document - Transport",
  operation_id = "transport_rail_waybill_composite_get",
  summary = "Get rail waybill composite",
  description = "Returns a rail waybill with nested wagon manifests.",
  path = paths::transport::rail::COMPOSITE_BY_ID,
  params(
    ("id" = Uuid, Path),
    ("embed" = Option<String>, Query, description = "Pass 'names' to include resolved FK names")
  ),
  responses((status = 200, body = ApiResponse<RailWaybillCompositeResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn rail_waybill_composite_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Query(embed): Query<EmbedParams>,
) -> ApiResult<RailWaybillCompositeResponse> {
  let row = if embed.wants_names() {
    state.svc.document.rail_waybill_composite_get_with_names(id).await?
  } else {
    state.svc.document.rail_waybill_composite_get(id).await?
  };
  Ok(ApiResponse::success(row))
}

#[utoipa::path(
  post,
  tag = "Document - Transport",
  operation_id = "transport_rail_waybill_composite_create",
  summary = "Create rail intake composite",
  description = "Creates a rail waybill aggregate in one request (waybill plus optional manifests, measurements, and weights). This endpoint is create-only and does not execute ledger-affecting flows.",
  path = paths::transport::rail::COMPOSITE_CREATE,
  request_body = RailWaybillCompositeRequest,
  responses((status = 200, body = ApiResponse<RailWaybillCompositeResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn rail_waybill_composite_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<RailWaybillCompositeRequest>>,
) -> ApiResult<RailWaybillCompositeResponse> {
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .rail_waybill_composite_create(&req)
      .await?,
  ))
}

pub(super) fn composite_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(rail_waybill_composite_get, rail_waybill_composite_create))
    .with_state(state)
}
