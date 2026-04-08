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
  /// Signaled after each completed sync cycle. Zero cost if nobody listens.
  pub cycle_completed: Arc<Notify>,
  /// Monotonic counter incremented every time a sync cycle completes
  /// successfully. Unlike `cycle_completed`, this value is readable by
  /// late observers: the `/sync/await-cycle` endpoint captures the value
  /// at entry and polls for it to advance, which eliminates the race
  /// where a cycle completes between the endpoint's initial read and its
  /// notification registration.
  pub cycles_completed: Arc<AtomicU64>,
  /// Monotonic counter incremented every time a worker tick fires and
  /// completes its probe of the local DB + remote status, regardless of
  /// whether a sync cycle actually happened. Used by `/sync/await-cycle`
  /// to tell "the worker has checked for new work since the call
  /// started" versus "the worker hasn't had a chance yet". Without this,
  /// the endpoint would guess based on elapsed time, which is racy under
  /// parallel test load.
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
  /// Read the current cycle count. Used by `/sync/await-cycle` to
  /// capture a baseline at entry.
  pub fn cycle_count(&self) -> u64 {
    self.cycles_completed.load(Ordering::Acquire)
  }

  /// Increment the cycle counter. Called by the worker after each
  /// successful `sync_once`.
  pub fn bump_cycle_count(&self) {
    self.cycles_completed.fetch_add(1, Ordering::Release);
  }

  /// Read the current tick count (ticks observed regardless of whether
  /// a cycle happened). Used by `/sync/await-cycle` to detect that the
  /// worker has had at least one chance to probe remote state.
  pub fn tick_count(&self) -> u64 {
    self.ticks_observed.load(Ordering::Acquire)
  }

  /// Increment the tick counter. Called by the worker at the end of
  /// every tick processing, regardless of whether sync_once ran.
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
