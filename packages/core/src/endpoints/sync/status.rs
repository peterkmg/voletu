use std::sync::Arc;

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

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
struct AwaitCycleQuery {
  /// Maximum time to wait in seconds (default: 15, max: 60).
  #[validate(range(min = 1, max = 60))]
  timeout: Option<u64>,
  /// If provided, return immediately if last_sync_at is already after this timestamp.
  /// Format: RFC 3339 (e.g., "2026-01-01T00:00:00Z").
  since: Option<String>,
}

/// Response shape for the await-cycle endpoint.
#[derive(Debug, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct AwaitCycleResponse {
  worker_state: String,
  last_sync_at: Option<String>,
  completed: bool,
}

#[utoipa::path(
  get,
  tag = "Sync",
  operation_id = "sync_await_cycle",
  summary = "Wait for next sync cycle completion",
  description = "Blocks until the sync worker completes its next cycle, then returns the worker status. \
    Returns immediately with completed=false if the worker is not running (Sleeping state). \
    Useful for orchestration and testing.",
  path = paths::sync::AWAIT_CYCLE,
  params(
    ("timeout" = Option<u64>, Query, description = "Max wait in seconds (default: 15, max: 60)")
  ),
  responses(
    (status = 200, body = ApiResponse<AwaitCycleResponse>, description = "Cycle completed (completed=true) or timed out / worker inactive (completed=false)"),
  )
)]
#[axum::debug_handler]
async fn sync_await_cycle(
  State(state): State<Arc<ApiState>>,
  Valid(Query(query)): Valid<Query<AwaitCycleQuery>>,
) -> ApiResult<AwaitCycleResponse> {
  let timeout_secs = query.timeout.unwrap_or(15);

  let since = query.since.as_deref().and_then(|s| {
    chrono::DateTime::parse_from_rfc3339(s)
      .ok()
      .map(|dt| dt.with_timezone(&chrono::Utc))
  });

  // Read current status and get the notify handle.
  let (notify, current_state, current_last_sync) = {
    let status = state.worker_status.read().await;
    (
      Arc::clone(&status.cycle_completed),
      status.state,
      status.last_sync_at,
    )
  };

  // Register interest in the NEXT notification BEFORE checking conditions.
  // This prevents the race where a cycle completes between our status read and the await.
  let notification = notify.notified();

  // If worker is sleeping (not a peripheral or no central URL), return immediately
  if matches!(current_state, crate::worker::WorkerState::Sleeping) {
    return Ok(ApiResponse::success(AwaitCycleResponse {
      worker_state: format!("{:?}", current_state),
      last_sync_at: current_last_sync.map(|t| t.to_rfc3339()),
      completed: false,
    }));
  }

  // If a sync already completed after the `since` timestamp, return immediately.
  if let (Some(since_ts), Some(last_sync)) = (since, current_last_sync) {
    if last_sync >= since_ts {
      return Ok(ApiResponse::success(AwaitCycleResponse {
        worker_state: format!("{:?}", current_state),
        last_sync_at: Some(last_sync.to_rfc3339()),
        completed: true,
      }));
    }
  }

  // Wait for the next cycle completion (notification was registered above)
  let result =
    tokio::time::timeout(std::time::Duration::from_secs(timeout_secs), notification).await;

  let status = state.worker_status.read().await;
  Ok(ApiResponse::success(AwaitCycleResponse {
    worker_state: format!("{:?}", status.state),
    last_sync_at: status.last_sync_at.map(|t| t.to_rfc3339()),
    completed: result.is_ok(),
  }))
}

pub(super) fn status_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(sync_status))
    .routes(routes!(sync_await_cycle))
    .with_state(state)
}
