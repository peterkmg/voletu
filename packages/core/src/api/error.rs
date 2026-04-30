use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;
use validator::{ValidationErrors, ValidationErrorsKind};

use super::response::ApiResponse;

#[derive(Clone)]
pub(crate) struct HandledByApiError;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ValidationIssue {
  pub field: String,
  pub code: String,
  pub message: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
  #[error("Not found: {0}")]
  NotFound(String),
  #[error("Bad request: {0}")]
  BadRequest(String),
  #[error("Validation error: {0}")]
  Validation(String),
  #[error("Validation error")]
  ValidationFields(Vec<ValidationIssue>),
  #[error("Conflict: {0}")]
  Conflict(String),
  #[error("Unauthorized: {0}")]
  Unauthorized(String),
  #[error("Forbidden: {0}")]
  Forbidden(String),
  #[error("Too many requests: {0}")]
  TooManyRequests(String),
  #[error("Node not initialized")]
  NodeNotInitialized,
  #[error("Internal error")]
  Internal(#[from] anyhow::Error),
  #[error("Database error")]
  Database(#[from] sea_orm::DbErr),
}

impl ApiError {
  pub fn status_code(&self) -> StatusCode {
    match self {
      Self::NotFound(_) => StatusCode::NOT_FOUND,
      Self::BadRequest(_) | Self::Validation(_) | Self::ValidationFields(_) => {
        StatusCode::BAD_REQUEST
      }
      Self::Conflict(_) => StatusCode::CONFLICT,
      Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
      Self::Forbidden(_) => StatusCode::FORBIDDEN,
      Self::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
      Self::NodeNotInitialized => StatusCode::FORBIDDEN,
      Self::Internal(_) | Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }

  pub fn error_code(&self) -> &'static str {
    match self {
      Self::NotFound(_) => "NOT_FOUND",
      Self::BadRequest(_) => "BAD_REQUEST",
      Self::Validation(_) | Self::ValidationFields(_) => "VALIDATION_ERROR",
      Self::Conflict(_) => "CONFLICT",
      Self::Unauthorized(_) => "UNAUTHORIZED",
      Self::Forbidden(_) => "FORBIDDEN",
      Self::TooManyRequests(_) => "TOO_MANY_REQUESTS",
      Self::NodeNotInitialized => "NODE_NOT_INITIALIZED",
      Self::Internal(_) => "INTERNAL_ERROR",
      Self::Database(_) => "DATABASE_ERROR",
    }
  }
}

impl IntoResponse for ApiError {
  fn into_response(self) -> Response {
    match &self {
      Self::Internal(err) => tracing::error!("Internal error: {err:#}"),
      Self::Database(err) => tracing::error!("Database error: {err}"),
      Self::NotFound(msg) => tracing::warn!("Not found: {msg}"),
      Self::BadRequest(msg) => tracing::warn!("Bad request: {msg}"),
      Self::Validation(msg) => tracing::warn!("Validation error: {msg}"),
      Self::ValidationFields(issues) => {
        tracing::warn!("Validation error ({} issues): {:?}", issues.len(), issues)
      }
      Self::Conflict(msg) => tracing::warn!("Conflict: {msg}"),
      Self::Unauthorized(msg) => tracing::warn!("Unauthorized: {msg}"),
      Self::Forbidden(msg) => tracing::warn!("Forbidden: {msg}"),
      Self::TooManyRequests(msg) => tracing::warn!("Too many requests: {msg}"),
      Self::NodeNotInitialized => tracing::warn!("Node not initialized"),
    }

    let status = self.status_code();

    let mut response = match self {
      Self::ValidationFields(issues) => {
        let body = json!({
          "success": false,
          "error": {
            "code": "VALIDATION_ERROR",
            "message": "Validation failed",
            "issues": issues,
          }
        });
        (status, Json(body)).into_response()
      }
      other => {
        let body = Json(ApiResponse::<()>::error(
          other.error_code().to_string(),
          other.to_string(),
        ));
        (status, body).into_response()
      }
    };

    response.extensions_mut().insert(HandledByApiError);
    response
  }
}

impl From<serde_json::Error> for ApiError {
  fn from(err: serde_json::Error) -> Self {
    Self::Validation(err.to_string())
  }
}

impl From<validator::ValidationErrors> for ApiError {
  fn from(err: validator::ValidationErrors) -> Self {
    Self::ValidationFields(validation_errors_to_issues(&err))
  }
}

pub fn validation_errors_to_issues(err: &ValidationErrors) -> Vec<ValidationIssue> {
  let mut out = Vec::new();
  collect_issues(err, "", &mut out);
  out
}

fn collect_issues(err: &ValidationErrors, prefix: &str, out: &mut Vec<ValidationIssue>) {
  for (field, kind) in err.errors() {
    let path = join_path(prefix, field);

    match kind {
      ValidationErrorsKind::Field(errors) => {
        for e in errors {
          let code = map_validator_code(&e.code, &e.params);
          let message = e
            .message
            .as_ref()
            .map(|m| m.to_string())
            .unwrap_or_else(|| format!("validation failed: {}", e.code));
          out.push(ValidationIssue {
            field: path.clone(),
            code,
            message,
          });
        }
      }
      ValidationErrorsKind::Struct(nested) => {
        collect_issues(nested, &path, out);
      }
      ValidationErrorsKind::List(items) => {
        for (idx, nested) in items {
          let item_path = format!("{}.{}", path, idx);
          collect_issues(nested, &item_path, out);
        }
      }
    }
  }
}

fn join_path(prefix: &str, field: &str) -> String {
  if prefix.is_empty() {
    field.to_string()
  } else {
    format!("{}.{}", prefix, field)
  }
}

fn map_validator_code(
  code: &str,
  params: &std::collections::HashMap<std::borrow::Cow<'static, str>, serde_json::Value>,
) -> String {
  match code {
    "length" => {
      let has_min = params.contains_key("min");
      let has_max = params.contains_key("max");
      match (has_min, has_max) {
        (true, true) => "length".to_string(),
        (true, false) => "min_length".to_string(),
        (false, true) => "max_length".to_string(),
        (false, false) => "length".to_string(),
      }
    }
    "range" => {
      let has_min = params.contains_key("min");
      let has_max = params.contains_key("max");
      match (has_min, has_max) {
        (true, false) => "too_small".to_string(),
        (false, true) => "too_big".to_string(),
        _ => "range".to_string(),
      }
    }
    "email" => "email".to_string(),
    "url" => "url".to_string(),
    "uuid" => "uuid".to_string(),
    "required" => "required".to_string(),
    "must_match" => "must_match".to_string(),
    "regex" => "regex".to_string(),
    other => other.to_string(),
  }
}
