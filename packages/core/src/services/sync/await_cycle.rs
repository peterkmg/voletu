use std::{sync::Arc, time::Duration};

use chrono::{DateTime, Utc};
use reqwest::Client;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::{
  helpers::{compute_base_discriminant, join_uuid_csv},
  specs::{AwaitCycleQuerySpec, SyncStatusQuerySpec},
  SyncService,
};
use crate::{
  api::ApiError,
  dtos::{AwaitCycleResponse, SyncStatusResponse, SyncWatermarkResponse},
  enums::SyncDirection,
  services::system::node_bases::load_node_base_ids,
  utils::http::get_api_json,
  worker::{WorkerState, WorkerStatus},
};

const AWAIT_CYCLE_POLL_INTERVAL: Duration = Duration::from_millis(50);

#[derive(Debug, Clone, Copy)]
struct PendingFence {
  target_node_id: Uuid,
  target_audit_log_id: Uuid,
}

#[derive(Debug, Default, Clone, Copy)]
struct PendingReplicationTargets {
  pull: Option<PendingFence>,
  push: Option<PendingFence>,
}

impl PendingReplicationTargets {
  fn has_pending_work(self) -> bool {
    self.pull.is_some() || self.push.is_some()
  }
}

#[derive(Debug, Clone, Copy)]
struct WorkerSnapshot {
  cycles: u64,
  ticks: u64,
  state: WorkerState,
  last_sync_at: Option<DateTime<Utc>>,
}

impl WorkerSnapshot {
  async fn read(worker_status: &Arc<RwLock<WorkerStatus>>) -> Self {
    let status = worker_status.read().await;
    Self {
      cycles: status.cycle_count(),
      ticks: status.tick_count(),
      state: status.state,
      last_sync_at: status.last_sync_at,
    }
  }

  fn to_response(self, completed: bool) -> AwaitCycleResponse {
    AwaitCycleResponse {
      worker_state: format!("{:?}", self.state),
      last_sync_at: self.last_sync_at.map(|t| t.to_rfc3339()),
      completed,
    }
  }
}

#[derive(Debug, Clone, Copy)]
enum AwaitCycleOutcome {
  Sleeping(WorkerSnapshot),
  SatisfiedImmediately(WorkerSnapshot),
  SatisfiedAfterFence(WorkerSnapshot),
  SatisfiedAfterCycle(WorkerSnapshot),
  SatisfiedAfterSince(WorkerSnapshot),
  SatisfiedAfterIdleProbe(WorkerSnapshot),
  TimedOut(WorkerSnapshot),
}

impl AwaitCycleOutcome {
  fn into_response(self) -> AwaitCycleResponse {
    match self {
      Self::Sleeping(snapshot) | Self::TimedOut(snapshot) => snapshot.to_response(false),
      Self::SatisfiedImmediately(snapshot)
      | Self::SatisfiedAfterFence(snapshot)
      | Self::SatisfiedAfterCycle(snapshot)
      | Self::SatisfiedAfterSince(snapshot)
      | Self::SatisfiedAfterIdleProbe(snapshot) => snapshot.to_response(true),
    }
  }
}

struct AwaitCycleCoordinator<'a> {
  sync: &'a SyncService,
  worker_status: &'a Arc<RwLock<WorkerStatus>>,
}

impl<'a> AwaitCycleCoordinator<'a> {
  fn new(sync: &'a SyncService, worker_status: &'a Arc<RwLock<WorkerStatus>>) -> Self {
    Self {
      sync,
      worker_status,
    }
  }

  async fn run(&self, query: AwaitCycleQuerySpec) -> Result<AwaitCycleOutcome, ApiError> {
    let initial = WorkerSnapshot::read(self.worker_status).await;

    if matches!(initial.state, WorkerState::Sleeping) {
      return Ok(AwaitCycleOutcome::Sleeping(initial));
    }

    let pending_targets = self.pending_replication_targets(query.timeout_secs).await?;
    if self.since_satisfied(initial, query.since)
      && pending_targets
        .map(|targets| !targets.has_pending_work())
        .unwrap_or(true)
    {
      return Ok(AwaitCycleOutcome::SatisfiedImmediately(initial));
    }

    let deadline = tokio::time::Instant::now() + Duration::from_secs(query.timeout_secs);

    if let Some(targets) = pending_targets.filter(|targets| targets.has_pending_work()) {
      return self.wait_for_pending_fences(targets, deadline).await;
    }

    self
      .wait_for_worker_progress(initial, query.since, deadline)
      .await
  }

  async fn pending_replication_targets(
    &self,
    timeout_secs: u64,
  ) -> Result<Option<PendingReplicationTargets>, ApiError> {
    if !self
      .sync
      .cfg
      .node
      .node_type
      .eq_ignore_ascii_case("PERIPHERAL")
    {
      return Ok(None);
    }

    let Some(central_api_url) = self
      .sync
      .cfg
      .node
      .central_api_url
      .as_deref()
      .map(str::trim)
      .filter(|value| !value.is_empty())
    else {
      return Ok(None);
    };

    let local_base_ids = self.load_local_base_ids().await?;
    let central_status: SyncStatusResponse = match get_api_json(
      &Client::new(),
      &format!(
        "{}/sync/status?baseIds={}",
        central_api_url.trim_end_matches('/'),
        join_uuid_csv(&local_base_ids)
      ),
      Duration::from_secs(timeout_secs.clamp(1, 5)),
    )
    .await
    {
      Ok(status) => status,
      Err(_) => return Ok(None),
    };

    let watermarks = self.sync.list_sync_watermarks().await?;

    let local_status = self
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

    let pull =
      (central_status.highest_matching_id > effective_pull_cursor).then_some(PendingFence {
        target_node_id: central_status.node_id,
        target_audit_log_id: central_status.highest_matching_id,
      });

    Ok(Some(PendingReplicationTargets { pull, push }))
  }

  async fn wait_for_pending_fences(
    &self,
    targets: PendingReplicationTargets,
    deadline: tokio::time::Instant,
  ) -> Result<AwaitCycleOutcome, ApiError> {
    loop {
      let snapshot = WorkerSnapshot::read(self.worker_status).await;

      let pull_done = match targets.pull {
        Some(fence) => self.fence_satisfied(fence, SyncDirection::Pull).await?,
        None => true,
      };

      let push_done = match targets.push {
        Some(fence) => self.fence_satisfied(fence, SyncDirection::Push).await?,
        None => true,
      };

      if pull_done && push_done {
        return Ok(AwaitCycleOutcome::SatisfiedAfterFence(snapshot));
      }

      if tokio::time::Instant::now() >= deadline {
        return Ok(AwaitCycleOutcome::TimedOut(snapshot));
      }

      tokio::time::sleep(AWAIT_CYCLE_POLL_INTERVAL).await;
    }
  }

  async fn wait_for_worker_progress(
    &self,
    initial: WorkerSnapshot,
    since: Option<DateTime<Utc>>,
    deadline: tokio::time::Instant,
  ) -> Result<AwaitCycleOutcome, ApiError> {
    loop {
      let snapshot = WorkerSnapshot::read(self.worker_status).await;

      if snapshot.cycles > initial.cycles {
        return Ok(AwaitCycleOutcome::SatisfiedAfterCycle(snapshot));
      }

      if self.since_satisfied(snapshot, since) {
        return Ok(AwaitCycleOutcome::SatisfiedAfterSince(snapshot));
      }

      if snapshot.ticks >= initial.ticks.saturating_add(2)
        && matches!(snapshot.state, WorkerState::OnlineIdle)
      {
        return Ok(AwaitCycleOutcome::SatisfiedAfterIdleProbe(snapshot));
      }

      if tokio::time::Instant::now() >= deadline {
        return Ok(AwaitCycleOutcome::TimedOut(snapshot));
      }

      tokio::time::sleep(AWAIT_CYCLE_POLL_INTERVAL).await;
    }
  }

  async fn fence_satisfied(
    &self,
    fence: PendingFence,
    direction: SyncDirection,
  ) -> Result<bool, ApiError> {
    let watermarks = self.sync.list_sync_watermarks().await?;
    let (cursor, stored_discriminant) = watermark_for(&watermarks, fence.target_node_id, direction);

    if matches!(direction, SyncDirection::Pull) {
      let current_discriminant = compute_base_discriminant(&self.load_local_base_ids().await?);
      if stored_discriminant != current_discriminant {
        return Ok(false);
      }
    }

    Ok(cursor >= fence.target_audit_log_id)
  }

  async fn load_local_base_ids(&self) -> Result<Vec<Uuid>, ApiError> {
    load_node_base_ids(self.sync.db.as_ref(), self.sync.cfg.node.db_id)
      .await
      .map_err(Into::into)
  }

  fn since_satisfied(&self, snapshot: WorkerSnapshot, since: Option<DateTime<Utc>>) -> bool {
    matches!((since, snapshot.last_sync_at), (Some(since_ts), Some(last_sync)) if last_sync >= since_ts)
  }
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

impl SyncService {
  pub async fn await_cycle(
    &self,
    worker_status: &Arc<RwLock<WorkerStatus>>,
    query: AwaitCycleQuerySpec,
  ) -> Result<AwaitCycleResponse, ApiError> {
    AwaitCycleCoordinator::new(self, worker_status)
      .run(query)
      .await
      .map(AwaitCycleOutcome::into_response)
  }
}
