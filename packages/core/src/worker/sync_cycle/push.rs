use reqwest::Client;
use tracing::debug;

use super::remote::SyncCycleRemoteStatus;
use crate::{
  config::SyncConfig,
  dtos::{PushAuditLogsRequest, PushAuditLogsResponse, SyncWatermarkResponse},
  enums::SyncDirection,
  services::sync::{specs::OutboundAuditLogsQuerySpec, SyncService},
  utils::http::post_api_json,
};

pub(super) struct PushPhaseOutcome {
  pub(super) pushed_log_count: u64,
}

pub(super) async fn push_outbound_logs(
  client: &Client,
  sync_service: &SyncService,
  central_sync_api_url: &str,
  remote_status: &SyncCycleRemoteStatus,
  watermarks: &[SyncWatermarkResponse],
  config: &SyncConfig,
) -> anyhow::Result<PushPhaseOutcome> {
  let (push_after, _) =
    super::super::topology::find_sync_watermark(watermarks, remote_status.remote_node_id(), "PUSH");
  let outbound_logs = sync_service
    .outbound_logs(OutboundAuditLogsQuerySpec::new(
      push_after,
      Some(config.sync_batch_limit),
    ))
    .await?;
  let pushed_log_count = outbound_logs.len() as u64;
  let last_outbound_log_id = outbound_logs.last().map(|last| last.id);

  if outbound_logs.is_empty() {
    return Ok(PushPhaseOutcome { pushed_log_count });
  }

  let push_response: PushAuditLogsResponse = post_api_json(
    client,
    &format!("{}/sync/push", central_sync_api_url),
    &PushAuditLogsRequest {
      logs: outbound_logs,
    },
    config.request_timeout,
  )
  .await?;
  debug!(
    accepted = push_response.accepted,
    rejected = push_response.rejected,
    total = pushed_log_count,
    "sync push batch processed"
  );

  if let Some(last_outbound_log_id) = last_outbound_log_id {
    sync_service
      .upsert_watermark(
        remote_status.remote_node_id(),
        SyncDirection::Push,
        last_outbound_log_id,
        String::new(),
      )
      .await?;
  }

  Ok(PushPhaseOutcome { pushed_log_count })
}
