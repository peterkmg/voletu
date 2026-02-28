use std::sync::Arc;

use axum::{
  extract::{Request, State},
  http::header::AUTHORIZATION,
  middleware::Next,
  response::Response,
};

use crate::{
  api::{ApiError, ApiState},
  context::audit::with_audit_context,
};

pub async fn auth_middleware(
  State(state): State<Arc<ApiState>>,
  mut req: Request,
  next: Next,
) -> Result<Response, ApiError> {
  let token = req
    .headers()
    .get(AUTHORIZATION)
    .and_then(|v| v.to_str().ok())
    .and_then(|s| s.strip_prefix("Bearer "))
    .ok_or_else(|| ApiError::Unauthorized("Missing or invalid Authorization header".to_string()))?
    .to_owned();

  let claims = state.token_svc.verify_access(&token).await?;

  req.extensions_mut().insert(claims.clone());

  let origin_db_id = state
    .cfg
    .read()
    .expect("ApiConfig lock poisoned")
    .node
    .database_id;

  Ok(
    with_audit_context(
      claims.uid,
      origin_db_id,
      || async move { next.run(req).await },
    )
    .await,
  )
}
