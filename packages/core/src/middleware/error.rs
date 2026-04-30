use axum::{
  body,
  extract::Request,
  http::StatusCode,
  middleware::Next,
  response::{IntoResponse, Response},
  Json,
};

use crate::api::{error::HandledByApiError, response::ApiResponse};

pub async fn error_middleware(req: Request, next: Next) -> Response {
  let response = next.run(req).await;
  let status = response.status();

  if status.is_success() || response.extensions().get::<HandledByApiError>().is_some() {
    return response;
  }

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
