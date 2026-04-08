use super::*;

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
struct UpsertWatermarkRequest {
  target_node_id: Uuid,
  direction: SyncDirection,
  last_audit_log_id: Uuid,
  /// Canonical base discriminant to store alongside the cursor. Optional;
  /// defaults to empty string ("catalog-only scope") when omitted. This
  /// endpoint is a manual override — the normal pull path goes through
  /// `apply_pulled_logs`, which sets the discriminant atomically from the
  /// peripheral's actual assignments.
  #[serde(default)]
  base_discriminant: Option<String>,
}

#[utoipa::path(
  get,
  tag = "Sync",
  operation_id = "sync_watermark_list",
  summary = "List sync watermarks",
  description = "Returns high-water marks tracked per target node and direction for incremental replication.",
  path = paths::sync::WATERMARKS,
  responses(
    (status = 200, body = ApiResponse<Vec<SyncWatermarkResponse>>, description = "Watermark list envelope. Example: {\"success\":true,\"data\":[{\"targetNodeId\":\"...\",\"direction\":\"Outbound\",\"lastAuditLogId\":\"...\"}]}" )
  )
)]
#[axum::debug_handler]
async fn watermark_list(
  State(state): State<Arc<ApiState>>,
) -> ApiResult<Vec<SyncWatermarkResponse>> {
  Ok(ApiResponse::success(
    state.svc.sync.list_sync_watermarks().await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Sync",
  operation_id = "sync_watermark_upsert",
  summary = "Upsert sync watermark",
  description = "Creates or updates a synchronization watermark for a node and direction pair.",
  path = paths::sync::WATERMARKS,
  request_body = UpsertWatermarkRequest,
  responses(
    (status = 200, body = ApiResponse<SyncWatermarkResponse>, description = "Upsert success envelope. Example: {\"success\":true,\"data\":{\"targetNodeId\":\"...\",\"direction\":\"Inbound\",\"lastAuditLogId\":\"...\"}}"),
    (status = 400, description = "Validation envelope for malformed request payload. Example: {\"success\":false,\"error\":{\"code\":\"VALIDATION_ERROR\",\"message\":\"Validation error: ...\"}}")
  )
)]
#[axum::debug_handler]
async fn watermark_upsert(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<UpsertWatermarkRequest>>,
) -> ApiResult<SyncWatermarkResponse> {
  Ok(ApiResponse::success(
    state
      .svc
      .sync
      .upsert_watermark(
        req.target_node_id,
        req.direction,
        req.last_audit_log_id,
        req.base_discriminant.unwrap_or_default(),
      )
      .await?,
  ))
}

pub(super) fn watermark_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(watermark_list))
    .routes(routes!(watermark_upsert))
    .with_state(state)
}
