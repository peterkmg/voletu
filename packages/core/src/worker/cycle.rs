use reqwest::Client;
use tracing::debug;
use uuid::Uuid;

use crate::{
  api::ApiError,
  config::SyncConfig,
  dtos::{
    PullAuditLogsResponse,
    PushAuditLogRequest,
    PushAuditLogsRequest,
    PushAuditLogsResponse,
    SyncStatusResponse,
  },
  enums::SyncDirection,
  services::sync::{
    helpers::compute_base_discriminant,
    specs::OutboundAuditLogsQuerySpec,
    SyncService,
  },
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
  let base_ids_param = crate::services::sync::helpers::join_uuid_csv(local_base_ids);

  // Scope-aware status: Central returns the highest id that matches our
  // scope (`highest_matching_id`) as well as the overall highest (diagnostic).
  let central_status: SyncStatusResponse = get_api_json(
    client,
    &format!(
      "{}/sync/status?baseIds={}",
      central_base_url, base_ids_param
    ),
    config.probe_timeout,
  )
  .await?;

  let watermarks = sync_service.list_sync_watermarks().await?;

  // PUSH phase: unchanged. Push has no scope concept — the peripheral pushes
  // every locally-originated log above the push watermark.
  let (push_after, _push_disc) =
    super::topology::watermark_for(&watermarks, central_status.node_id, "PUSH");
  let outbound = sync_service
    .outbound_logs(OutboundAuditLogsQuerySpec::new(
      push_after,
      Some(config.sync_batch_limit),
    ))
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

  // PULL phase: discriminant-aware cursor + atomic apply.
  let current_discriminant = compute_base_discriminant(local_base_ids);
  let (stored_last_id, stored_discriminant) =
    super::topology::watermark_for(&watermarks, central_status.node_id, "PULL");

  let pull_cursor = if stored_discriminant == current_discriminant {
    stored_last_id
  } else {
    debug!(
      stored = %stored_discriminant,
      current = %current_discriminant,
      "base discriminant changed, resetting pull cursor"
    );
    Uuid::nil()
  };

  let pulled: PullAuditLogsResponse = get_api_json(
    client,
    &format!(
      "{}/sync/pull?lastAuditLogId={}&baseIds={}&limit={}",
      central_base_url, pull_cursor, base_ids_param, config.sync_batch_limit
    ),
    config.request_timeout,
  )
  .await?;

  let pulled_count = pulled.logs.len() as u64;
  let pulled_highest_evaluated_id = pulled.highest_evaluated_id;

  // apply_pulled_logs commits log application + watermark advance atomically.
  // If the discriminant drifted while the pull was in flight, it returns
  // Conflict and we leave the watermark untouched — next tick retries.
  if pulled_count > 0 || pulled_highest_evaluated_id != pull_cursor {
    let apply_payload: Vec<_> = pulled
      .logs
      .into_iter()
      .map(PushAuditLogRequest::from)
      .collect();
    match sync_service
      .apply_pulled_logs(
        &apply_payload,
        central_status.node_id,
        pulled_highest_evaluated_id,
        current_discriminant.clone(),
      )
      .await
    {
      Ok(result) => {
        debug!(
          accepted = result.accepted,
          rejected = result.rejected,
          "sync pull batch applied"
        );
      }
      Err(ApiError::Conflict(msg)) => {
        debug!(reason = %msg, "pull apply aborted due to discriminant drift");
      }
      Err(other) => return Err(other.into()),
    }
  }

  Ok(outbound_count + pulled_count)
}
