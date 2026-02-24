use std::sync::Arc;

use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;
use axum_extra::extract::TypedHeader;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use chrono::Utc;

use crate::api::{ApiError, ApiState};

pub async fn auth_middleware(
  State(state): State<Arc<ApiState>>,
  TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
  mut req: Request,
  next: Next,
) -> Result<Response, ApiError> {
  let claims = state.jwt_service.verify_access(auth.token()).await?;

  if claims.exp < Utc::now().timestamp() as usize {
    return Err(ApiError::Unauthorized("Token expired".to_string()));
  }

  req.extensions_mut().insert(claims.clone());

  Ok(next.run(req).await)
}
