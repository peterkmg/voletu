use reqwest::Client;
use tracing::debug;
use uuid::Uuid;

use super::remote::SyncCycleRemoteStatus;
use crate::{
  api::ApiError,
  config::SyncConfig,
  dtos::{
    PullAuditLogsResponse,
    PushAuditLogRequest,
    PushAuditLogsResponse,
    SyncWatermarkResponse,
  },
  services::sync::SyncService,
  utils::http::get_api_json,
};

pub(super) struct PullPhaseOutcome {
  pub(super) pulled_log_count: u64,
}

pub(super) async fn pull_remote_logs(
  client: &Client,
  sync_service: &SyncService,
  central_sync_api_url: &str,
  remote_status: &SyncCycleRemoteStatus,
  watermarks: &[SyncWatermarkResponse],
  config: &SyncConfig,
) -> anyhow::Result<PullPhaseOutcome> {
  let pull_cursor = resolve_pull_cursor(remote_status, watermarks);
  let pulled_logs: PullAuditLogsResponse = get_api_json(
    client,
    &format!(
      "{}/sync/pull?lastAuditLogId={}&baseIds={}&limit={}",
      central_sync_api_url,
      pull_cursor,
      remote_status.assigned_base_ids_query,
      config.sync_batch_limit
    ),
    config.request_timeout,
  )
  .await?;

  let pulled_log_count = pulled_logs.logs.len() as u64;
  let highest_evaluated_log_id = pulled_logs.highest_evaluated_id;

  if pulled_log_count == 0 && highest_evaluated_log_id == pull_cursor {
    return Ok(PullPhaseOutcome { pulled_log_count });
  }

  let apply_payload: Vec<_> = pulled_logs
    .logs
    .into_iter()
    .map(PushAuditLogRequest::from)
    .collect();

  match sync_service
    .apply_pulled_logs(
      &apply_payload,
      remote_status.remote_node_id(),
      highest_evaluated_log_id,
      remote_status.assigned_base_discriminant.clone(),
    )
    .await
  {
    Ok(PushAuditLogsResponse { accepted, rejected }) => {
      debug!(accepted, rejected, "sync pull batch applied");
    }
    Err(ApiError::Conflict(message)) => {
      debug!(
        reason = %message,
        "pull apply aborted due to discriminant drift"
      );
    }
    Err(other) => return Err(other.into()),
  }

  Ok(PullPhaseOutcome { pulled_log_count })
}

fn resolve_pull_cursor(
  remote_status: &SyncCycleRemoteStatus,
  watermarks: &[SyncWatermarkResponse],
) -> Uuid {
  let (stored_last_id, stored_discriminant) =
    super::super::topology::find_sync_watermark(watermarks, remote_status.remote_node_id(), "PULL");

  if stored_discriminant == remote_status.assigned_base_discriminant {
    return stored_last_id;
  }

  debug!(
    stored = %stored_discriminant,
    current = %remote_status.assigned_base_discriminant,
    "base discriminant changed, resetting pull cursor"
  );
  Uuid::nil()
}
