use serde::Serialize;
use tracing::trace;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum WorkerState {
  Sleeping,
  Offline,
  OnlineIdle,
  Syncing,
  Backoff,
}

#[derive(Debug, Clone)]
pub struct WorkerStatus {
  pub state: WorkerState,
  pub last_sync_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for WorkerStatus {
  fn default() -> Self {
    Self {
      state: WorkerState::Sleeping,
      last_sync_at: None,
    }
  }
}

pub(super) fn transition(state: &mut WorkerState, next: WorkerState) {
  if *state != next {
    trace!(from = ?state, to = ?next, "sync worker state transition");
    *state = next;
  }
}
