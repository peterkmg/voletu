use std::time::Duration;

#[derive(Debug, Clone)]
pub struct SyncConfig {
  pub tick_interval: Duration,
  pub probe_timeout: Duration,
  pub request_timeout: Duration,
  pub sync_batch_limit: u64,
}

impl Default for SyncConfig {
  fn default() -> Self {
    Self {
      tick_interval: Duration::from_secs(3),
      probe_timeout: Duration::from_secs(3),
      request_timeout: Duration::from_secs(10),
      sync_batch_limit: 1000,
    }
  }
}
