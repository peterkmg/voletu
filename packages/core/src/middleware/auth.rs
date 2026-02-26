use std::sync::Arc;

use axum::{
  extract::{Request, State},
  middleware::Next,
  response::Response,
};
use axum_extra::{
  extract::TypedHeader,
  headers::{authorization::Bearer, Authorization},
};

use crate::{
  api::{ApiError, ApiState},
  context::audit::with_audit_context,
};

pub async fn auth_middleware(
  State(state): State<Arc<ApiState>>,
  TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
  mut req: Request,
  next: Next,
) -> Result<Response, ApiError> {
  let claims = state.jwt_service.verify_access(auth.token()).await?;

  req.extensions_mut().insert(claims.clone());

  Ok(
    with_audit_context(claims.uid, state.cfg.node.database_id, || async move {
      next.run(req).await
    })
    .await,
  )
}
