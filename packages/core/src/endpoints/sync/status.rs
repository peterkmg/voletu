use super::*;

#[utoipa::path(
  get,
  tag = "Sync",
  operation_id = "sync_status",
  summary = "Get sync status",
  description = "Returns node synchronization status and readiness information used by replication workers.",
  path = paths::sync::STATUS,
  responses(
    (status = 200, body = ApiResponse<SyncStatusResponse>, description = "Sync status envelope. Example: {\"success\":true,\"data\":{\"nodeId\":\"...\",\"ready\":true}}")
  )
)]
#[axum::debug_handler]
async fn sync_status(State(state): State<Arc<ApiState>>) -> ApiResult<SyncStatusResponse> {
  Ok(ApiResponse::success(state.svc.sync.sync_status().await?))
}

pub(super) fn status_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new().routes(routes!(sync_status)).with_state(state)
}
