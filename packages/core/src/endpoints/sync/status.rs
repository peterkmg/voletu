use std::sync::Arc;

use reqwest::Client;
use uuid::Uuid;

use super::*;
use crate::{
  api::ApiError,
  dtos::{SyncStatusResponse, SyncWatermarkResponse},
  enums::SyncDirection,
  services::{
    sync::{helpers::compute_base_discriminant, query::SyncStatusQuerySpec},
    system::node_bases::load_node_base_ids,
  },
  utils::http::get_api_json,
};

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

impl From<SyncStatusQuery> for SyncStatusQuerySpec {
  fn from(query: SyncStatusQuery) -> Self {
    Self::new(query.parse_base_ids())
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
  Ok(ApiResponse::success(
    state.svc.sync.sync_status(req.into()).await?,
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

#[derive(Debug, Clone, Copy)]
struct PendingFence {
  target_node_id: Uuid,
  target_audit_log_id: Uuid,
}

#[derive(Debug, Default)]
struct PendingReplicationTargets {
  pull: Option<PendingFence>,
  push: Option<PendingFence>,
}

fn base_ids_query(base_ids: &[Uuid]) -> String {
  base_ids
    .iter()
    .map(|id| id.to_string())
    .collect::<Vec<_>>()
    .join(",")
}

fn watermark_for(
  watermarks: &[SyncWatermarkResponse],
  target_node_id: Uuid,
  direction: SyncDirection,
) -> (Uuid, String) {
  let watermark = watermarks
    .iter()
    .find(|wm| wm.target_node_id == target_node_id && wm.direction == direction);

  match watermark {
    Some(wm) => (wm.last_audit_log_id, wm.base_discriminant.clone()),
    None => (Uuid::nil(), String::new()),
  }
}

async fn load_local_base_ids(state: &ApiState) -> Result<Vec<Uuid>, ApiError> {
  load_node_base_ids(state.db.as_ref(), state.cfg.node.db_id)
    .await
    .map_err(Into::into)
}

async fn pending_replication_targets(
  state: &Arc<ApiState>,
  timeout_secs: u64,
) -> Result<Option<PendingReplicationTargets>, ApiError> {
  if !state.cfg.node.node_type.eq_ignore_ascii_case("PERIPHERAL") {
    return Ok(None);
  }

  let Some(central_api_url) = state
    .cfg
    .node
    .central_api_url
    .as_deref()
    .map(str::trim)
    .filter(|value| !value.is_empty())
  else {
    return Ok(None);
  };

  let local_base_ids = load_local_base_ids(state).await?;
  let central_status: SyncStatusResponse = match get_api_json(
    &Client::new(),
    &format!(
      "{}/sync/status?baseIds={}",
      central_api_url.trim_end_matches('/'),
      base_ids_query(&local_base_ids)
    ),
    std::time::Duration::from_secs(timeout_secs.clamp(1, 5)),
  )
  .await
  {
    Ok(status) => status,
    Err(_) => return Ok(None),
  };

  let watermarks = state.svc.sync.list_sync_watermarks().await?;
  let local_status = state
    .svc
    .sync
    .sync_status(SyncStatusQuerySpec::default())
    .await?;
  let current_discriminant = compute_base_discriminant(&local_base_ids);

  let (push_cursor, _) = watermark_for(&watermarks, central_status.node_id, SyncDirection::Push);
  let push = (local_status.highest_audit_log_id > push_cursor).then_some(PendingFence {
    target_node_id: central_status.node_id,
    target_audit_log_id: local_status.highest_audit_log_id,
  });

  let (stored_pull_cursor, stored_discriminant) =
    watermark_for(&watermarks, central_status.node_id, SyncDirection::Pull);
  let effective_pull_cursor = if stored_discriminant == current_discriminant {
    stored_pull_cursor
  } else {
    Uuid::nil()
  };
  let pull = (central_status.highest_matching_id > effective_pull_cursor).then_some(PendingFence {
    target_node_id: central_status.node_id,
    target_audit_log_id: central_status.highest_matching_id,
  });

  Ok(Some(PendingReplicationTargets { pull, push }))
}

async fn fence_satisfied(
  state: &Arc<ApiState>,
  fence: PendingFence,
  direction: SyncDirection,
) -> Result<bool, ApiError> {
  let watermarks = state.svc.sync.list_sync_watermarks().await?;
  let (cursor, stored_discriminant) = watermark_for(&watermarks, fence.target_node_id, direction);

  if matches!(direction, SyncDirection::Pull) {
    let current_discriminant = compute_base_discriminant(&load_local_base_ids(state).await?);
    if stored_discriminant != current_discriminant {
      return Ok(false);
    }
  }

  Ok(cursor >= fence.target_audit_log_id)
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

  let pending_targets = pending_replication_targets(&state, timeout_secs).await?;

  // If the caller provided a `since` threshold that's already satisfied,
  // return immediately.
  if pending_targets
    .as_ref()
    .map(|targets| targets.pull.is_none() && targets.push.is_none())
    .unwrap_or(true)
  {
    if let (Some(since_ts), Some(last_sync)) = (since, initial_last_sync) {
      if last_sync >= since_ts {
        return Ok(ApiResponse::success(AwaitCycleResponse {
          worker_state: format!("{:?}", initial_state),
          last_sync_at: Some(last_sync.to_rfc3339()),
          completed: true,
        }));
      }
    }
  }

  // When work was already pending at call entry, wait for the relevant
  // watermark(s) to reach the target ids instead of treating "some cycle
  // finished" as proof that the specific change we care about was processed.
  if let Some(targets) = &pending_targets {
    let has_pending_work = targets.pull.is_some() || targets.push.is_some();
    if has_pending_work {
      let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(timeout_secs);
      let poll_interval = std::time::Duration::from_millis(50);

      loop {
        let (cur_state, cur_last_sync) = {
          let status = state.worker_status.read().await;
          (status.state, status.last_sync_at)
        };

        let pull_done = match targets.pull {
          Some(fence) => fence_satisfied(&state, fence, SyncDirection::Pull).await?,
          None => true,
        };
        let push_done = match targets.push {
          Some(fence) => fence_satisfied(&state, fence, SyncDirection::Push).await?,
          None => true,
        };

        if pull_done && push_done {
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
  let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(timeout_secs);
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
