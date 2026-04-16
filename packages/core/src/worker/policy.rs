use tracing::debug;
use uuid::Uuid;

use super::{context::WorkerContext, topology};
use crate::{dtos::SyncStatusResponse, services::sync::SyncService};

pub(super) async fn evaluate_pending_sync_work(
  sync_service: &SyncService,
  context: &WorkerContext,
  remote_status: &SyncStatusResponse,
  has_pending_sync_work: bool,
) -> anyhow::Result<bool> {
  let watermarks = sync_service.list_sync_watermarks().await?;
  let (stored_last_audit_log_id, stored_base_discriminant) =
    topology::find_sync_watermark(&watermarks, remote_status.node_id, "PULL");

  let effective_pull_cursor = if stored_base_discriminant == context.local_base_discriminant {
    stored_last_audit_log_id
  } else {
    debug!(
      stored = %stored_base_discriminant,
      current = %context.local_base_discriminant,
      "base discriminant changed, resetting pull cursor"
    );
    Uuid::nil()
  };

  if remote_status.highest_matching_id > effective_pull_cursor {
    return Ok(true);
  }

  // Push side: if the PUSH watermark lags behind the highest local audit log id,
  // there are still local changes to send to central. Without this check, the
  // worker would idle after a single-batch push even though thousands of logs
  // remain in the peripheral's outbound queue.
  let (push_cursor, _) = topology::find_sync_watermark(&watermarks, remote_status.node_id, "PUSH");
  if context.local_highest_audit_log_id > push_cursor {
    return Ok(true);
  }

  Ok(has_pending_sync_work)
}
