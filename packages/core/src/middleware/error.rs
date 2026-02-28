use axum::{
  body,
  extract::Request,
  http::StatusCode,
  middleware::Next,
  response::{IntoResponse, Response},
  Json,
};

use crate::api::{error::HandledByApiError, response::ApiResponse};

/// Catches error responses that were **not** produced by [`ApiError`] (framework
/// rejections such as extractor failures, method-not-allowed, etc.) and wraps
/// them in the standard `ApiResponse` envelope.
///
/// Responses already handled by `ApiError::into_response` carry a
/// [`HandledByApiError`] extension and pass through unchanged — no body reading
/// required. For bare framework rejections, the original axum error body is
/// consumed to surface a descriptive message (path/query parse errors carry the
/// field name and reason; JSON deserialization errors carry the serde detail).
/// Re-wrapped responses are also tagged so the middleware is idempotent when
/// layered multiple times.
///
/// [`ApiError`]: crate::api::ApiError
pub async fn error_middleware(req: Request, next: Next) -> Response {
  let response = next.run(req).await;
  let status = response.status();

  if status.is_success() || response.extensions().get::<HandledByApiError>().is_some() {
    return response;
  }

  // Framework rejection: consume the original (small, plain-text) body so we
  // can surface the axum extractor's descriptive error message rather than the
  // generic canonical reason phrase.
  let message = body::to_bytes(response.into_body(), 16 * 1024)
    .await
    .ok()
    .and_then(|b| String::from_utf8(b.to_vec()).ok())
    .filter(|s| !s.is_empty())
    .unwrap_or_else(|| {
      status
        .canonical_reason()
        .unwrap_or("Request failed")
        .to_string()
    });

  (
    status,
    Json(ApiResponse::<()>::error(
      error_code_for_status(status).to_string(),
      message,
    )),
  )
    .into_response()
}

/// Maps a raw HTTP status code to a canonical error code string.
/// Only called for framework-level rejections — application errors use
/// `ApiError::error_code()` directly.
fn error_code_for_status(status: StatusCode) -> &'static str {
  match status {
    StatusCode::BAD_REQUEST | StatusCode::UNPROCESSABLE_ENTITY => "VALIDATION_ERROR",
    StatusCode::NOT_FOUND => "NOT_FOUND",
    StatusCode::UNAUTHORIZED => "UNAUTHORIZED",
    StatusCode::FORBIDDEN => "FORBIDDEN",
    StatusCode::METHOD_NOT_ALLOWED => "METHOD_NOT_ALLOWED",
    StatusCode::CONFLICT => "CONFLICT",
    _ if status.is_server_error() => "INTERNAL_ERROR",
    _ => "BAD_REQUEST",
  }
}
