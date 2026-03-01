use std::sync::Mutex;

use tokio::sync::oneshot;

use crate::api::ApiError;

pub fn request_restart(restart_tx: &Mutex<Option<oneshot::Sender<()>>>) -> Result<(), ApiError> {
  let restart_tx = restart_tx
    .lock()
    .expect("restart channel lock poisoned")
    .take()
    .ok_or_else(|| ApiError::Conflict("API restart is already in progress".to_string()))?;

  if restart_tx.send(()).is_err() {
    tracing::warn!("restart signal receiver is unavailable; treating restart request as initiated");
  }

  Ok(())
}
