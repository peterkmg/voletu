use std::{
  collections::hash_map::Entry,
  sync::Arc,
  time::{Duration, Instant},
};

use axum::{
  extract::{Request, State},
  http::header::HeaderName,
  middleware::Next,
  response::Response,
};
use uuid::Uuid;

use crate::api::{ApiError, ApiState};

static IDEMPOTENCY_KEY_HEADER: HeaderName = HeaderName::from_static("idempotency-key");
const IDEMPOTENCY_TTL: Duration = Duration::from_secs(60);

fn is_mutating(method: &axum::http::Method) -> bool {
  matches!(
    *method,
    axum::http::Method::POST
      | axum::http::Method::PUT
      | axum::http::Method::PATCH
      | axum::http::Method::DELETE
  )
}

pub async fn idempotency_middleware(
  State(state): State<Arc<ApiState>>,
  req: Request,
  next: Next,
) -> Result<Response, ApiError> {
  if !is_mutating(req.method()) {
    return Ok(next.run(req).await);
  }

  let header = req
    .headers()
    .get(&IDEMPOTENCY_KEY_HEADER)
    .ok_or_else(|| ApiError::BadRequest("Idempotency-Key header is required".to_string()))?;

  let header_val = header
    .to_str()
    .map(str::trim)
    .map_err(|_| ApiError::BadRequest("Idempotency-Key must be a valid UUID".to_string()))?;

  if header_val.is_empty() {
    return Err(ApiError::BadRequest(
      "Idempotency-Key header must not be empty".to_string(),
    ));
  }

  let key = Uuid::parse_str(header_val)
    .map_err(|_| ApiError::BadRequest("Idempotency-Key must be a valid UUID".to_string()))?;

  let now = Instant::now();
  let expires_at = now + IDEMPOTENCY_TTL;

  {
    let mut cache = state
      .idempotency_cache
      .lock()
      .unwrap_or_else(|e| e.into_inner());

    cache.retain(|_, expires_at| *expires_at > now);

    match cache.entry(key) {
      Entry::Occupied(entry) if *entry.get() > now => {
        return Err(ApiError::Conflict(
          "Duplicate idempotent request was already processed".to_string(),
        ));
      }
      Entry::Occupied(mut entry) => {
        entry.insert(expires_at);
      }
      Entry::Vacant(entry) => {
        entry.insert(expires_at);
      }
    }
  }

  let response = next.run(req).await;

  if !response.status().is_success() {
    let mut cache = state
      .idempotency_cache
      .lock()
      .unwrap_or_else(|e| e.into_inner());
    cache.remove(&key);
  }

  Ok(response)
}
