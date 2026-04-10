use std::sync::Arc;

use reqwest::Client;
use sea_orm::DatabaseConnection;
use tokio::sync::RwLock;
use tracing::{debug, warn};
use uuid::Uuid;

use super::{
  context::{load_worker_context, WorkerContext},
  policy::evaluate_pending_sync_work,
  probe::{probe_remote_sync_status, RemoteSyncProbe},
  state,
  sync_cycle,
  WorkerState,
  WorkerStatus,
};
use crate::{
  config::{ApiConfig, SyncConfig},
  services::sync::SyncService,
};

pub(super) struct WorkerRuntime {
  pub(super) db: Arc<DatabaseConnection>,
  pub(super) client: Client,
  pub(super) sync_service: SyncService,
  pub(super) config: SyncConfig,
  pub(super) state: WorkerState,
  pub(super) remote_is_online: bool,
  pub(super) has_pending_sync_work: bool,
  pub(super) last_observed_local_audit_log_id: Uuid,
}

impl WorkerRuntime {
  pub(super) fn new(db: Arc<DatabaseConnection>, cfg: Arc<ApiConfig>, config: SyncConfig) -> Self {
    Self {
      db: db.clone(),
      client: Client::new(),
      sync_service: SyncService::new(db, cfg),
      config,
      state: WorkerState::Sleeping,
      remote_is_online: false,
      has_pending_sync_work: false,
      last_observed_local_audit_log_id: Uuid::nil(),
    }
  }
}

enum WorkerDecision {
  Sleep,
  Backoff,
  MarkOffline,
  StayIdle,
  StartSyncCycle(WorkerContext),
}

pub(super) struct TickOutcome {
  next_state: WorkerState,
  last_sync_at: Option<chrono::DateTime<chrono::Utc>>,
  cycle_completed: bool,
}

impl TickOutcome {
  fn state_only(next_state: WorkerState) -> Self {
    Self {
      next_state,
      last_sync_at: None,
      cycle_completed: false,
    }
  }

  fn cycle_completed(next_state: WorkerState) -> Self {
    Self {
      next_state,
      last_sync_at: Some(chrono::Utc::now()),
      cycle_completed: true,
    }
  }
}

pub(super) async fn execute_worker_tick(runtime: &mut WorkerRuntime) -> TickOutcome {
  let decision = match decide_worker_action(runtime).await {
    Ok(decision) => decision,
    Err(error) => {
      warn!(%error, "sync worker tick evaluation failed");
      WorkerDecision::Backoff
    }
  };

  apply_worker_decision(runtime, decision).await
}

pub(super) async fn publish_worker_tick_outcome(
  shared_status: &Arc<RwLock<WorkerStatus>>,
  outcome: TickOutcome,
) {
  let notify = {
    let mut status = shared_status.write().await;
    status.state = outcome.next_state;
    if let Some(last_sync_at) = outcome.last_sync_at {
      status.last_sync_at = Some(last_sync_at);
    }
    if outcome.cycle_completed {
      status.bump_cycle_count();
      Some(Arc::clone(&status.cycle_completed))
    } else {
      None
    }
  };

  shared_status.read().await.bump_tick_count();

  if let Some(notify) = notify {
    notify.notify_waiters();
  }
}

async fn decide_worker_action(runtime: &mut WorkerRuntime) -> anyhow::Result<WorkerDecision> {
  let Some(loaded_context) = load_worker_context(runtime).await? else {
    return Ok(WorkerDecision::Sleep);
  };
  let context = loaded_context.context;

  if loaded_context.local_progress_changed {
    runtime.last_observed_local_audit_log_id = context.local_highest_audit_log_id;
    runtime.has_pending_sync_work = true;
  }

  match probe_remote_sync_status(&runtime.client, &runtime.config, &context).await {
    RemoteSyncProbe::Offline => {
      if runtime.remote_is_online {
        runtime.has_pending_sync_work = true;
      }
      runtime.remote_is_online = false;
      Ok(WorkerDecision::MarkOffline)
    }
    RemoteSyncProbe::Online(remote_status) => {
      if !runtime.remote_is_online {
        runtime.remote_is_online = true;
        runtime.has_pending_sync_work = true;
        debug!("sync worker remote online state changed");
      }

      runtime.has_pending_sync_work = evaluate_pending_sync_work(
        &runtime.sync_service,
        &context,
        &remote_status,
        runtime.has_pending_sync_work,
      )
      .await?;

      if runtime.has_pending_sync_work {
        Ok(WorkerDecision::StartSyncCycle(context))
      } else {
        Ok(WorkerDecision::StayIdle)
      }
    }
  }
}

async fn apply_worker_decision(
  runtime: &mut WorkerRuntime,
  decision: WorkerDecision,
) -> TickOutcome {
  match decision {
    WorkerDecision::Sleep => {
      state::transition(&mut runtime.state, WorkerState::Sleeping);
      TickOutcome::state_only(runtime.state)
    }
    WorkerDecision::Backoff => {
      state::transition(&mut runtime.state, WorkerState::Backoff);
      TickOutcome::state_only(runtime.state)
    }
    WorkerDecision::MarkOffline => {
      state::transition(&mut runtime.state, WorkerState::Offline);
      TickOutcome::state_only(runtime.state)
    }
    WorkerDecision::StayIdle => {
      state::transition(&mut runtime.state, WorkerState::OnlineIdle);
      TickOutcome::state_only(runtime.state)
    }
    WorkerDecision::StartSyncCycle(context) => {
      state::transition(&mut runtime.state, WorkerState::Syncing);
      match sync_cycle::run_sync_cycle(
        &runtime.client,
        &runtime.sync_service,
        &context.central_sync_api_url,
        &context.local_base_ids,
        &runtime.config,
      )
      .await
      {
        Ok(result) => {
          runtime.has_pending_sync_work = false;
          state::transition(&mut runtime.state, WorkerState::OnlineIdle);
          debug!(
            changed = result.changed_log_count(),
            "sync worker cycle completed"
          );
          TickOutcome::cycle_completed(runtime.state)
        }
        Err(error) => {
          warn!(%error, "sync worker cycle failed");
          state::transition(&mut runtime.state, WorkerState::Backoff);
          TickOutcome::state_only(runtime.state)
        }
      }
    }
  }
}
