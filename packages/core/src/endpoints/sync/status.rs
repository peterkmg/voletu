use std::sync::Arc;

use super::*;
use crate::dtos::{
  AwaitCycleQueryRequest,
  AwaitCycleResponse,
  SyncStatusQueryRequest,
  SyncStatusResponse,
};

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
  Valid(Query(req)): Valid<Query<SyncStatusQueryRequest>>,
) -> ApiResult<SyncStatusResponse> {
  Ok(ApiResponse::success(
    state.svc.sync.sync_status(req.into()).await?,
  ))
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
  Valid(Query(query)): Valid<Query<AwaitCycleQueryRequest>>,
) -> ApiResult<AwaitCycleResponse> {
  Ok(ApiResponse::success(
    state
      .svc
      .sync
      .await_cycle(&state.worker_status, query.into())
      .await?,
  ))
}

pub(super) fn status_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(sync_status))
    .routes(routes!(sync_await_cycle))
    .with_state(state)
}
