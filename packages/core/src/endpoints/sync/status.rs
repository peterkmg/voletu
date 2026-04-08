use std::sync::Arc;

use super::*;

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
struct SyncStatusQuery {
  /// Comma-separated base UUIDs the caller handles. Absent or empty means
  /// catalog-only scope.
  #[serde(default)]
  base_ids: Option<String>,
}

impl SyncStatusQuery {
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
  get,
  tag = "Sync",
  operation_id = "sync_status",
  summary = "Get sync status",
  description = "Returns node synchronization status and readiness information used by replication workers. Accepts optional baseIds to compute a scope-aware highest_matching_id.",
  path = paths::sync::STATUS,
  params(
    ("baseIds" = Option<String>, Query, description = "Comma-separated base UUIDs the caller handles")
  ),
  responses(
    (status = 200, body = ApiResponse<SyncStatusResponse>, description = "Sync status envelope")
  )
)]
#[axum::debug_handler]
async fn sync_status(
  State(state): State<Arc<ApiState>>,
  Valid(Query(req)): Valid<Query<SyncStatusQuery>>,
) -> ApiResult<SyncStatusResponse> {
  let base_ids = req.parse_base_ids();
  Ok(ApiResponse::success(
    state.svc.sync.sync_status(&base_ids).await?,
  ))
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

  // Capture initial counters + state atomically. The cycle counter and
  // tick counter are race-free primitives: every successful `sync_once`
  // increments `cycles_completed`, every worker tick (regardless of
  // whether it synced anything) increments `ticks_observed`. Readers
  // never miss an update because there's no subscription involved.
  let (initial_cycles, initial_ticks, initial_state, initial_last_sync) = {
    let status = state.worker_status.read().await;
    (
      status.cycle_count(),
      status.tick_count(),
      status.state,
      status.last_sync_at,
    )
  };

  // If the worker is Sleeping (not a peripheral / no central URL), no
  // cycle will ever happen. Return immediately.
  if matches!(initial_state, crate::worker::WorkerState::Sleeping) {
    return Ok(ApiResponse::success(AwaitCycleResponse {
      worker_state: format!("{:?}", initial_state),
      last_sync_at: initial_last_sync.map(|t| t.to_rfc3339()),
      completed: false,
    }));
  }

  // If the caller provided a `since` threshold that's already satisfied,
  // return immediately.
  if let (Some(since_ts), Some(last_sync)) = (since, initial_last_sync) {
    if last_sync >= since_ts {
      return Ok(ApiResponse::success(AwaitCycleResponse {
        worker_state: format!("{:?}", initial_state),
        last_sync_at: Some(last_sync.to_rfc3339()),
        completed: true,
      }));
    }
  }

  // Poll both counters. Break out as soon as either:
  //   (a) cycles_completed advances past initial_cycles → at least one
  //       full sync_once finished since the call, peripheral has pulled
  //       whatever Central had as of that cycle's probe; OR
  //   (b) ticks_observed advances at least twice past initial_ticks AND
  //       the worker state is OnlineIdle → the worker has definitely
  //       probed Central at least once since the call started, found no
  //       work, and settled. Requiring TWO ticks rather than one
  //       guarantees at least one probe happened entirely within the
  //       endpoint's wait window (the first observed advance may reflect
  //       a tick that began before the call).
  //
  // Also break if a `since` threshold gets satisfied, or the timeout
  // elapses. We poll instead of awaiting a `Notify` because notifications
  // are lost to listeners that aren't subscribed at the moment of
  // `notify_waiters()`.
  let deadline =
    tokio::time::Instant::now() + std::time::Duration::from_secs(timeout_secs);
  let poll_interval = std::time::Duration::from_millis(50);

  loop {
    let (cycles, ticks, cur_state, cur_last_sync) = {
      let status = state.worker_status.read().await;
      (
        status.cycle_count(),
        status.tick_count(),
        status.state,
        status.last_sync_at,
      )
    };

    // A cycle finished after the call started — definitely caught up to
    // whatever Central had at the time of the probe.
    if cycles > initial_cycles {
      return Ok(ApiResponse::success(AwaitCycleResponse {
        worker_state: format!("{:?}", cur_state),
        last_sync_at: cur_last_sync.map(|t| t.to_rfc3339()),
        completed: true,
      }));
    }

    if let (Some(since_ts), Some(last_sync)) = (since, cur_last_sync) {
      if last_sync >= since_ts {
        return Ok(ApiResponse::success(AwaitCycleResponse {
          worker_state: format!("{:?}", cur_state),
          last_sync_at: Some(last_sync.to_rfc3339()),
          completed: true,
        }));
      }
    }

    // At least two ticks have elapsed AND the worker is idle → the
    // worker has probed Central inside our wait window and found nothing
    // to do. Peripheral is caught up.
    if ticks >= initial_ticks.saturating_add(2)
      && matches!(cur_state, crate::worker::WorkerState::OnlineIdle)
    {
      return Ok(ApiResponse::success(AwaitCycleResponse {
        worker_state: format!("{:?}", cur_state),
        last_sync_at: cur_last_sync.map(|t| t.to_rfc3339()),
        completed: true,
      }));
    }

    if tokio::time::Instant::now() >= deadline {
      return Ok(ApiResponse::success(AwaitCycleResponse {
        worker_state: format!("{:?}", cur_state),
        last_sync_at: cur_last_sync.map(|t| t.to_rfc3339()),
        completed: false,
      }));
    }

    tokio::time::sleep(poll_interval).await;
  }
}

pub(super) fn status_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(sync_status))
    .routes(routes!(sync_await_cycle))
    .with_state(state)
}
