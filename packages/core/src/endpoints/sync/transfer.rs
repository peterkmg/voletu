use super::*;

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
struct PullAuditLogsQuery {
  node_id: Uuid,
  last_audit_log_id: Uuid,
  limit: Option<u64>,
}

#[utoipa::path(
  post,
  tag = "Sync",
  operation_id = "sync_log_push",
  summary = "Push logs",
  description = "Accepts replication events from a peer node and applies them using sync ingestion rules.",
  path = paths::sync::PUSH,
  request_body = PushAuditLogsRequest,
  responses(
    (status = 200, body = ApiResponse<PushAuditLogsResponse>, description = "Push result envelope. Example: {\"success\":true,\"data\":{\"accepted\":12,\"rejected\":0}}"),
    (status = 400, description = "Validation or malformed payload envelope. Example: {\"success\":false,\"error\":{\"code\":\"VALIDATION_ERROR\",\"message\":\"Validation error: ...\"}}")
  )
)]
#[axum::debug_handler]
async fn log_push(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<PushAuditLogsRequest>>,
) -> ApiResult<PushAuditLogsResponse> {
  Ok(ApiResponse::success(
    state.svc.sync.push_logs(&req.logs).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Sync",
  operation_id = "sync_log_pull",
  summary = "Pull logs",
  description = "Returns replication events for a requesting node after its acknowledged watermark.",
  path = paths::sync::PULL,
  params(
    ("nodeId" = Uuid, Query, description = "Requesting node ID"),
    ("lastAuditLogId" = Uuid, Query, description = "Last processed audit log ID"),
    ("limit" = Option<u64>, Query, description = "Max number of logs to return")
  ),
  responses(
    (status = 200, body = ApiResponse<PullAuditLogsResponse>, description = "Pull result envelope containing logs and next cursor hints. Example: {\"success\":true,\"data\":{\"logs\":[...],\"hasMore\":false}}"),
    (status = 400, description = "Validation envelope for malformed query params. Example: {\"success\":false,\"error\":{\"code\":\"VALIDATION_ERROR\",\"message\":\"Validation error: ...\"}}"),
    (status = 404, description = "Not found envelope when the requesting node does not exist. Example: {\"success\":false,\"error\":{\"code\":\"NOT_FOUND\",\"message\":\"Not found: ...\"}}")
  )
)]
#[axum::debug_handler]
async fn log_pull(
  State(state): State<Arc<ApiState>>,
  Valid(Query(req)): Valid<Query<PullAuditLogsQuery>>,
) -> ApiResult<PullAuditLogsResponse> {
  Ok(ApiResponse::success(
    state
      .svc
      .sync
      .pull_logs(req.node_id, req.last_audit_log_id, req.limit)
      .await?,
  ))
}

pub(super) fn transfer_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(log_push))
    .routes(routes!(log_pull))
    .with_state(state)
}
