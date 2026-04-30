use std::sync::{
  atomic::{AtomicU64, Ordering},
  Arc,
};

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
  pub cycle_completed: Arc<Notify>,
  pub cycles_completed: Arc<AtomicU64>,
  pub ticks_observed: Arc<AtomicU64>,
}

impl Clone for WorkerStatus {
  fn clone(&self) -> Self {
    Self {
      state: self.state,
      last_sync_at: self.last_sync_at,
      cycle_completed: Arc::clone(&self.cycle_completed),
      cycles_completed: Arc::clone(&self.cycles_completed),
      ticks_observed: Arc::clone(&self.ticks_observed),
    }
  }
}

impl Default for WorkerStatus {
  fn default() -> Self {
    Self {
      state: WorkerState::Sleeping,
      last_sync_at: None,
      cycle_completed: Arc::new(Notify::new()),
      cycles_completed: Arc::new(AtomicU64::new(0)),
      ticks_observed: Arc::new(AtomicU64::new(0)),
    }
  }
}

impl WorkerStatus {
  pub fn cycle_count(&self) -> u64 {
    self.cycles_completed.load(Ordering::Acquire)
  }

  pub fn bump_cycle_count(&self) {
    self.cycles_completed.fetch_add(1, Ordering::Release);
  }

  pub fn tick_count(&self) -> u64 {
    self.ticks_observed.load(Ordering::Acquire)
  }

  pub fn bump_tick_count(&self) {
    self.ticks_observed.fetch_add(1, Ordering::Release);
  }
}

pub(super) fn transition(state: &mut WorkerState, next: WorkerState) {
  if *state != next {
    trace!(from = ?state, to = ?next, "sync worker state transition");
    *state = next;
  }
}
