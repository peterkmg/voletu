use super::*;

#[utoipa::path(
  get,
  tag = "Document - Transport",
  operation_id = "transport_rail_manifest_list",
  summary = "List rail manifests",
  description = "Returns wagon manifest rows linked to rail waybills.",
  path = paths::transport::rail::MANIFESTS,
  responses((status = 200, body = ApiResponse<Vec<RailWagonManifestResponse>>))
)]
#[axum::debug_handler]
async fn rail_manifest_list(
  State(state): State<Arc<ApiState>>,
) -> ApiResult<Vec<RailWagonManifestResponse>> {
  Ok(ApiResponse::success(
    state.svc.document.rail_manifest_list(None).await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Document - Transport",
  operation_id = "transport_rail_manifest_create",
  summary = "Create rail manifest",
  description = "Creates a wagon manifest under an existing rail waybill.",
  path = paths::transport::rail::MANIFESTS,
  request_body = CreateRailWagonManifestRequest,
  responses((status = 200, body = ApiResponse<RailWagonManifestResponse>), (status = 400))
)]
#[axum::debug_handler]
async fn rail_manifest_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateRailWagonManifestRequest>>,
) -> ApiResult<RailWagonManifestResponse> {
  Ok(ApiResponse::success(
    state.svc.document.rail_manifest_create(&req).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Document - Transport",
  operation_id = "transport_rail_manifest_get",
  summary = "Get rail manifest",
  path = paths::transport::rail::MANIFESTS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200, body = ApiResponse<RailWagonManifestResponse>), (status = 404))
)]
#[axum::debug_handler]
async fn rail_manifest_get(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<RailWagonManifestResponse> {
  Ok(ApiResponse::success(
    state.svc.document.rail_manifest_get(id).await?,
  ))
}

#[utoipa::path(
  put,
  tag = "Document - Transport",
  operation_id = "transport_rail_manifest_update",
  summary = "Update rail manifest",
  path = paths::transport::rail::MANIFESTS_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateRailWagonManifestRequest,
  responses((status = 200, body = ApiResponse<RailWagonManifestResponse>), (status = 400), (status = 404))
)]
#[axum::debug_handler]
async fn rail_manifest_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateRailWagonManifestRequest>>,
) -> ApiResult<RailWagonManifestResponse> {
  Ok(ApiResponse::success(
    state.svc.document.rail_manifest_update(id, &req).await?,
  ))
}

#[utoipa::path(
  delete,
  tag = "Document - Transport",
  operation_id = "transport_rail_manifest_soft_delete",
  summary = "Soft delete rail manifest",
  path = paths::transport::rail::MANIFESTS_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn rail_manifest_soft_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.rail_manifest_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

#[utoipa::path(
  delete,
  tag = "Document - Transport",
  operation_id = "transport_rail_manifest_hard_delete",
  summary = "Hard delete rail manifest",
  path = paths::transport::rail::MANIFESTS_HARD_DELETE_BY_ID,
  params(("id" = Uuid, Path)),
  responses((status = 200), (status = 404))
)]
#[axum::debug_handler]
async fn rail_manifest_hard_delete(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
) -> ApiResult<()> {
  state.svc.document.rail_manifest_hard_delete(id).await?;
  Ok(ApiResponse::success(()))
}

pub(super) fn manifest_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(rail_manifest_list, rail_manifest_create))
    .routes(routes!(rail_manifest_get, rail_manifest_update))
    .routes(routes!(rail_manifest_soft_delete))
    .routes(routes!(rail_manifest_hard_delete))
    .with_state(state)
}
