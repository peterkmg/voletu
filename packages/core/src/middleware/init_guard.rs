use std::sync::{atomic::Ordering, Arc};

use axum::{
  extract::{Request, State},
  middleware::Next,
  response::Response,
};

use crate::{api::{ApiError, ApiState}, endpoints::paths};

const INIT_EXEMPT_PATHS: &[&str] = &[
  paths::node::INITIALIZE,
  paths::auth::CHANGE_PASSWORD,
];

pub async fn init_guard_middleware(
  State(state): State<Arc<ApiState>>,
  req: Request,
  next: Next,
) -> Result<Response, ApiError> {
  if state.is_initialized.load(Ordering::Relaxed) {
    return Ok(next.run(req).await);
  }

  let path = req.uri().path();
  if INIT_EXEMPT_PATHS.contains(&path) {
    return Ok(next.run(req).await);
  }

  Err(ApiError::NodeNotInitialized)
}
