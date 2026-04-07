use super::*;

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
struct PullAuditLogsQuery {
  last_audit_log_id: Uuid,
  /// Comma-separated base UUIDs the requesting node handles. Empty = catalog-only sync.
  #[serde(default)]
  base_ids: Option<String>,
  limit: Option<u64>,
}

impl PullAuditLogsQuery {
  fn parse_base_ids(&self) -> Vec<Uuid> {
    self
      .base_ids
      .as_deref()
      .unwrap_or("")
      .split(',')
      .filter_map(|s| {
        let trimmed = s.trim();
        if trimmed.is_empty() {
          None
        } else {
          Uuid::try_parse(trimmed).ok()
        }
      })
      .collect()
  }
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
  description = "Returns replication events filtered by the requesting node's base assignments. Central receives all pushed data; peripherals request only their assigned bases.",
  path = paths::sync::PULL,
  params(
    ("lastAuditLogId" = Uuid, Query, description = "Last processed audit log ID"),
    ("baseIds" = Option<String>, Query, description = "Comma-separated base UUIDs the requesting node handles. Empty = catalog-only sync."),
    ("limit" = Option<u64>, Query, description = "Max number of logs to return")
  ),
  responses(
    (status = 200, body = ApiResponse<PullAuditLogsResponse>, description = "Pull result envelope containing logs and next cursor."),
    (status = 400, description = "Validation envelope for malformed query params.")
  )
)]
#[axum::debug_handler]
async fn log_pull(
  State(state): State<Arc<ApiState>>,
  Valid(Query(req)): Valid<Query<PullAuditLogsQuery>>,
) -> ApiResult<PullAuditLogsResponse> {
  let base_ids = req.parse_base_ids();
  Ok(ApiResponse::success(
    state
      .svc
      .sync
      .pull_logs(req.last_audit_log_id, &base_ids, req.limit)
      .await?,
  ))
}

pub(super) fn transfer_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(log_push))
    .routes(routes!(log_pull))
    .with_state(state)
}
