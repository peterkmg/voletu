use std::sync::Arc;

use reqwest::Client;
use sea_orm::DatabaseConnection;
use tokio::sync::RwLock;
use tracing::{debug, warn};
use uuid::Uuid;

use super::{cycle, state, topology, WorkerState, WorkerStatus};
use crate::{
  config::{ApiConfig, SyncConfig},
  dtos::SyncStatusResponse,
  services::sync::{helpers::compute_base_discriminant, specs::SyncStatusQuerySpec, SyncService},
  utils::http::{get_api_json, normalize_base_url},
};

pub(super) struct WorkerRuntime {
  db: Arc<DatabaseConnection>,
  client: Client,
  sync_service: SyncService,
  config: SyncConfig,
  state: WorkerState,
  is_online: bool,
  has_updates: bool,
  last_local_highest: Uuid,
}

impl WorkerRuntime {
  pub(super) fn default(
    db: Arc<DatabaseConnection>,
    cfg: Arc<ApiConfig>,
    config: SyncConfig,
  ) -> Self {
    Self {
      db: db.clone(),
      client: Client::new(),
      sync_service: SyncService::new(db, cfg),
      config,
      state: WorkerState::Sleeping,
      is_online: false,
      has_updates: false,
      last_local_highest: Uuid::nil(),
    }
  }
}

struct TickContext {
  local_node_id: Uuid,
  central_api_url: String,
  local_base_ids: Vec<Uuid>,
}

enum TickDecision {
  Sleeping,
  Backoff,
  Offline,
  OnlineIdle,
  RunCycle(TickContext),
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

pub(super) async fn run_tick(runtime: &mut WorkerRuntime) -> TickOutcome {
  let decision = evaluate_tick(runtime).await;
  apply_decision(runtime, decision).await
}

pub(super) async fn publish_tick_outcome(
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

async fn evaluate_tick(runtime: &mut WorkerRuntime) -> TickDecision {
  let context = match load_tick_context(runtime).await {
    Ok(Some(context)) => context,
    Ok(None) => return TickDecision::Sleeping,
    Err(error) => {
      warn!(%error, "sync worker could not prepare tick context");
      return TickDecision::Backoff;
    }
  };

  let remote_status = match probe_remote_status(runtime, &context).await {
    Ok(Some(status)) => status,
    Ok(None) => return TickDecision::Offline,
    Err(error) => {
      warn!(%error, "sync worker could not evaluate remote status");
      return TickDecision::Backoff;
    }
  };

  match detect_pending_work(runtime, &context, &remote_status).await {
    Ok(true) => TickDecision::RunCycle(context),
    Ok(false) => TickDecision::OnlineIdle,
    Err(error) => {
      warn!(%error, "sync worker could not detect pending work");
      TickDecision::Backoff
    }
  }
}

async fn apply_decision(runtime: &mut WorkerRuntime, decision: TickDecision) -> TickOutcome {
  match decision {
    TickDecision::Sleeping => {
      state::transition(&mut runtime.state, WorkerState::Sleeping);
      TickOutcome::state_only(runtime.state)
    }
    TickDecision::Backoff => {
      state::transition(&mut runtime.state, WorkerState::Backoff);
      TickOutcome::state_only(runtime.state)
    }
    TickDecision::Offline => {
      state::transition(&mut runtime.state, WorkerState::Offline);
      TickOutcome::state_only(runtime.state)
    }
    TickDecision::OnlineIdle => {
      state::transition(&mut runtime.state, WorkerState::OnlineIdle);
      TickOutcome::state_only(runtime.state)
    }
    TickDecision::RunCycle(context) => {
      state::transition(&mut runtime.state, WorkerState::Syncing);
      match cycle::sync_once(
        &runtime.client,
        &runtime.sync_service,
        &context.central_api_url,
        context.local_node_id,
        &context.local_base_ids,
        &runtime.config,
      )
      .await
      {
        Ok(changed) => {
          runtime.has_updates = false;
          state::transition(&mut runtime.state, WorkerState::OnlineIdle);
          debug!(changed, "sync worker cycle completed");
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

async fn load_tick_context(runtime: &mut WorkerRuntime) -> anyhow::Result<Option<TickContext>> {
  let db = runtime.db.as_ref();
  let (local_node_id, node_type, central_api_url) = topology::load_runtime_topology(db).await?;

  if !node_type.eq_ignore_ascii_case("PERIPHERAL") {
    return Ok(None);
  }

  let Some(central_api_url) = central_api_url.map(|value| normalize_base_url(&value)) else {
    return Ok(None);
  };

  let local_base_ids = topology::load_local_base_ids(db, local_node_id).await?;
  let local_status = runtime
    .sync_service
    .sync_status(SyncStatusQuerySpec::default())
    .await?;

  if local_status.highest_audit_log_id != runtime.last_local_highest {
    runtime.last_local_highest = local_status.highest_audit_log_id;
    runtime.has_updates = true;
  }

  Ok(Some(TickContext {
    local_node_id,
    central_api_url,
    local_base_ids,
  }))
}

async fn probe_remote_status(
  runtime: &mut WorkerRuntime,
  context: &TickContext,
) -> anyhow::Result<Option<SyncStatusResponse>> {
  let base_ids_param = crate::services::sync::helpers::join_uuid_csv(&context.local_base_ids);
  let remote_status = get_api_json::<SyncStatusResponse>(
    &runtime.client,
    &format!(
      "{}/sync/status?baseIds={}",
      context.central_api_url, base_ids_param
    ),
    runtime.config.probe_timeout,
  )
  .await;

  let online_now = remote_status.is_ok();
  if online_now != runtime.is_online {
    runtime.is_online = online_now;
    runtime.has_updates = true;
    debug!(
      online = runtime.is_online,
      "sync worker remote online state changed"
    );
  }

  if !runtime.is_online {
    return Ok(None);
  }

  Ok(Some(remote_status?))
}

async fn detect_pending_work(
  runtime: &mut WorkerRuntime,
  context: &TickContext,
  remote_status: &SyncStatusResponse,
) -> anyhow::Result<bool> {
  let watermarks = runtime.sync_service.list_sync_watermarks().await?;
  let (stored_last, stored_disc) =
    topology::watermark_for(&watermarks, remote_status.node_id, "PULL");
  let current_disc = compute_base_discriminant(&context.local_base_ids);
  let effective_cursor = if stored_disc == current_disc {
    stored_last
  } else {
    Uuid::nil()
  };

  if remote_status.highest_matching_id > effective_cursor {
    runtime.has_updates = true;
  }

  Ok(runtime.has_updates)
}
