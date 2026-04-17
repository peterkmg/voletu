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
    state
      .svc
      .document
      .rail_waybill_composite_get_with_names(id)
      .await?
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

#[utoipa::path(
  put,
  tag = "Document - Transport",
  operation_id = "transport_rail_waybill_composite_update",
  summary = "Update rail waybill composite",
  description = "Updates a rail waybill aggregate. Header fields apply set_if_some semantics; the manifests list is treated as the full new state and is diff-applied (insert / update / delete), with nested measurements and weights diffed recursively for each surviving manifest.",
  path = paths::transport::rail::COMPOSITE_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateRailWaybillCompositeRequest,
  responses(
    (status = 200, body = ApiResponse<RailWaybillCompositeResponse>),
    (status = 400),
    (status = 404)
  )
)]
#[axum::debug_handler]
async fn rail_waybill_composite_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateRailWaybillCompositeRequest>>,
) -> ApiResult<RailWaybillCompositeResponse> {
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .rail_waybill_composite_update(id, &req)
      .await?,
  ))
}

pub(super) fn composite_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(
      rail_waybill_composite_get,
      rail_waybill_composite_create,
      rail_waybill_composite_update
    ))
    .with_state(state)
}
