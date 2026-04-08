use reqwest::Client;
use tracing::debug;
use uuid::Uuid;

use crate::{
  config::SyncConfig,
  dtos::{
    PullAuditLogsResponse,
    PushAuditLogRequest,
    PushAuditLogsRequest,
    PushAuditLogsResponse,
    SyncStatusResponse,
  },
  enums::SyncDirection,
  services::sync::SyncService,
  utils::http::{get_api_json, post_api_json},
};

pub(super) async fn sync_once(
  client: &Client,
  sync_service: &SyncService,
  central_base_url: &str,
  _local_node_id: Uuid,
  local_base_ids: &[Uuid],
  config: &SyncConfig,
) -> anyhow::Result<u64> {
  let central_status: SyncStatusResponse = get_api_json(
    client,
    &format!("{}/sync/status", central_base_url),
    config.probe_timeout,
  )
  .await?;

  let watermarks = sync_service.list_sync_watermarks().await?;
  let push_after = super::topology::watermark_for(&watermarks, central_status.node_id, "PUSH");
  let outbound = sync_service
    .outbound_logs(push_after, Some(config.sync_batch_limit))
    .await?;
  let outbound_count = outbound.len() as u64;
  let last_outbound_id = outbound.last().map(|last| last.id);

  if !outbound.is_empty() {
    let pushed: PushAuditLogsResponse = post_api_json(
      client,
      &format!("{}/sync/push", central_base_url),
      &PushAuditLogsRequest { logs: outbound },
      config.request_timeout,
    )
    .await?;
    debug!(
      accepted = pushed.accepted,
      rejected = pushed.rejected,
      total = outbound_count,
      "sync push batch processed"
    );

    if let Some(last_outbound_id) = last_outbound_id {
      sync_service
        .upsert_watermark(
          central_status.node_id,
          SyncDirection::Push,
          last_outbound_id,
          String::new(),
        )
        .await?;
    }
  }

  let pull_after = super::topology::watermark_for(&watermarks, central_status.node_id, "PULL");
  let base_ids_param = local_base_ids
    .iter()
    .map(|id| id.to_string())
    .collect::<Vec<_>>()
    .join(",");
  let pulled: PullAuditLogsResponse = get_api_json(
    client,
    &format!(
      "{}/sync/pull?lastAuditLogId={}&baseIds={}&limit={}",
      central_base_url, pull_after, base_ids_param, config.sync_batch_limit
    ),
    config.request_timeout,
  )
  .await?;

  let pulled_count = pulled.logs.len() as u64;
  let pulled_highest_evaluated_id = pulled.highest_evaluated_id;

  if pulled_count > 0 {
    debug!(count = pulled_count, "applying pulled logs");
    let apply_payload = pulled
      .logs
      .into_iter()
      .map(PushAuditLogRequest::from)
      .collect::<Vec<_>>();
    let result = sync_service.push_logs(&apply_payload).await?;
    debug!(
      accepted = result.accepted,
      rejected = result.rejected,
      "sync pull batch applied"
    );
  }

  if pulled_highest_evaluated_id != pull_after {
    sync_service
      .upsert_watermark(
        central_status.node_id,
        SyncDirection::Pull,
        pulled_highest_evaluated_id,
        String::new(),
      )
      .await?;
  }

  Ok(outbound_count + pulled_count)
}
