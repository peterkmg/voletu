use std::sync::Arc;

use serde::Serialize;
use tokio::sync::Notify;
use tracing::trace;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum WorkerState {
  Sleeping,
  Offline,
  OnlineIdle,
  Syncing,
  Backoff,
}

#[derive(Debug)]
pub struct WorkerStatus {
  pub state: WorkerState,
  pub last_sync_at: Option<chrono::DateTime<chrono::Utc>>,
  /// Signaled after each completed sync cycle. Zero cost if nobody listens.
  pub cycle_completed: Arc<Notify>,
}

impl Clone for WorkerStatus {
  fn clone(&self) -> Self {
    Self {
      state: self.state,
      last_sync_at: self.last_sync_at,
      cycle_completed: Arc::clone(&self.cycle_completed),
    }
  }
}

impl Default for WorkerStatus {
  fn default() -> Self {
    Self {
      state: WorkerState::Sleeping,
      last_sync_at: None,
      cycle_completed: Arc::new(Notify::new()),
    }
  }
}

pub(super) fn transition(state: &mut WorkerState, next: WorkerState) {
  if *state != next {
    trace!(from = ?state, to = ?next, "sync worker state transition");
    *state = next;
  }
}
