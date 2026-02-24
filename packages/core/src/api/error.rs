use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};

use super::response::ApiResponse;

/// All errors that can be returned from API handlers.
///
/// Variants map to HTTP status codes and are automatically logged at the
/// appropriate level when converted to an HTTP response:
/// - 4xx client errors → `WARN`
/// - 5xx server errors → `ERROR`
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
  #[error("Not found: {0}")]
  NotFound(String),
  #[error("Bad request: {0}")]
  BadRequest(String),
  #[error("Validation error: {0}")]
  Validation(String),
  #[error("Conflict: {0}")]
  Conflict(String),
  #[error("Unauthorized: {0}")]
  Unauthorized(String),
  #[error("Forbidden: {0}")]
  Forbidden(String),
  #[error("Internal error")]
  Internal(#[from] anyhow::Error),
  #[error("Database error")]
  Database(#[from] sea_orm::DbErr),
}

impl ApiError {
  pub fn status_code(&self) -> StatusCode {
    match self {
      Self::NotFound(_) => StatusCode::NOT_FOUND,
      Self::BadRequest(_) | Self::Validation(_) => StatusCode::BAD_REQUEST,
      Self::Conflict(_) => StatusCode::CONFLICT,
      Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
      Self::Forbidden(_) => StatusCode::FORBIDDEN,
      Self::Internal(_) | Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }

  pub fn error_code(&self) -> &'static str {
    match self {
      Self::NotFound(_) => "NOT_FOUND",
      Self::BadRequest(_) => "BAD_REQUEST",
      Self::Validation(_) => "VALIDATION_ERROR",
      Self::Conflict(_) => "CONFLICT",
      Self::Unauthorized(_) => "UNAUTHORIZED",
      Self::Forbidden(_) => "FORBIDDEN",
      Self::Internal(_) => "INTERNAL_ERROR",
      Self::Database(_) => "DATABASE_ERROR",
    }
  }
}

impl IntoResponse for ApiError {
  fn into_response(self) -> Response {
    // Log at the appropriate level so callers don't have to.
    match &self {
      Self::Internal(err) => tracing::error!("Internal error: {err:#}"),
      Self::Database(err) => tracing::error!("Database error: {err}"),
      Self::NotFound(msg) => tracing::warn!("Not found: {msg}"),
      Self::BadRequest(msg) => tracing::warn!("Bad request: {msg}"),
      Self::Validation(msg) => tracing::warn!("Validation error: {msg}"),
      Self::Conflict(msg) => tracing::warn!("Conflict: {msg}"),
      Self::Unauthorized(msg) => tracing::warn!("Unauthorized: {msg}"),
      Self::Forbidden(msg) => tracing::warn!("Forbidden: {msg}"),
    }

    let status = self.status_code();
    let body = Json(ApiResponse::<()>::error(
      self.error_code().to_string(),
      self.to_string(),
    ));
    (status, body).into_response()
  }
}

impl From<serde_json::Error> for ApiError {
  fn from(err: serde_json::Error) -> Self {
    Self::Validation(err.to_string())
  }
}
