use tracing::trace;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum WorkerState {
  Sleeping,
  Offline,
  OnlineIdle,
  Syncing,
  Backoff,
}

pub(super) fn transition(state: &mut WorkerState, next: WorkerState) {
  if *state != next {
    trace!(from = ?state, to = ?next, "sync worker state transition");
    *state = next;
  }
}
