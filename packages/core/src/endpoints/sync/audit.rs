use super::*;
use crate::dtos::{AuditLogQueryRequest, OutboundLogsQueryRequest};

#[utoipa::path(
  get,
  tag = "Sync",
  operation_id = "sync_audit_log_list",
  summary = "List audit logs",
  description = "Returns persisted local audit logs in ascending order for diagnostics and replication inspection.",
  path = paths::sync::AUDIT_LOGS,
  params(
    ("tableName" = Option<crate::enums::AuditTable>, Query, description = "Filter to a specific audited table"),
    ("recordId" = Option<Uuid>, Query, description = "Filter to audit events for one record"),
    ("originDbId" = Option<Uuid>, Query, description = "Filter to audit events originating from one node"),
    ("limit" = Option<u64>, Query, description = "Max number of logs to return"),
    ("offset" = Option<u64>, Query, description = "Skip this many matching logs before returning results")
  ),
  responses(
    (status = 200, body = ApiResponse<Vec<AuditLogResponse>>, description = "Audit log list envelope. Example: {\"success\":true,\"data\":[{\"id\":\"...\",\"action\":\"create\"}]}" ),
    (status = 400, description = "Validation envelope for malformed query params. Example: {\"success\":false,\"error\":{\"code\":\"VALIDATION_ERROR\",\"message\":\"Validation error: ...\"}}")
  )
)]
#[axum::debug_handler]
async fn audit_log_list(
  State(state): State<Arc<ApiState>>,
  Valid(Query(query)): Valid<Query<AuditLogQueryRequest>>,
) -> ApiResult<Vec<AuditLogResponse>> {
  Ok(ApiResponse::success(
    state.svc.sync.audit_log_query(query.into()).await?,
  ))
}

#[utoipa::path(
  get,
  tag = "Sync",
  operation_id = "sync_outbound_log_list",
  summary = "List outbound logs",
  description = "Returns outbound replication events after a specific audit log id, constrained by optional limit.",
  path = paths::sync::OUTBOUND,
  params(
    ("afterAuditLogId" = Uuid, Query, description = "Return logs with id greater than this value"),
    ("limit" = Option<u64>, Query, description = "Max number of logs to return")
  ),
  responses(
    (status = 200, body = ApiResponse<Vec<PushAuditLogRequest>>, description = "Outbound log list envelope. Example: {\"success\":true,\"data\":[{\"auditLogId\":\"...\",\"table\":\"products\"}]}"),
    (status = 400, description = "Validation envelope for malformed query params. Example: {\"success\":false,\"error\":{\"code\":\"VALIDATION_ERROR\",\"message\":\"Validation error: ...\"}}")
  )
)]
#[axum::debug_handler]
async fn outbound_log_list(
  State(state): State<Arc<ApiState>>,
  Valid(Query(req)): Valid<Query<OutboundLogsQueryRequest>>,
) -> ApiResult<Vec<PushAuditLogRequest>> {
  Ok(ApiResponse::success(
    state.svc.sync.outbound_logs(req.into()).await?,
  ))
}

pub(super) fn audit_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(audit_log_list))
    .routes(routes!(outbound_log_list))
    .with_state(state)
}
