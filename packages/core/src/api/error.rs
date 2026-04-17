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

/// Zero-sized marker inserted into response extensions by `ApiError::into_response`.
/// [`error_envelope_middleware`] uses this to skip responses that are already
/// correctly enveloped — no body reading or JSON parsing required.
///
/// [`error_envelope_middleware`]: crate::middleware::error_envelope::error_envelope_middleware
#[derive(Clone)]
pub(crate) struct HandledByApiError;

/// A single field-level validation issue.
///
/// Returned inside the `issues` array of a structured validation error
/// response. Field paths use React Hook Form's dot+index syntax (e.g.
/// `items.2.accepted_amount`) so the frontend can map issues directly onto
/// form fields without translation.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ValidationIssue {
  /// Field path in RHF dot+index format, e.g. `items.2.accepted_amount`.
  pub field: String,
  /// Stable code for i18n key lookup, e.g. `positive`, `min_length`, `uuid`.
  pub code: String,
  /// Human-readable default message (English).
  pub message: String,
}

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
  /// Structured validation failure carrying per-field issues.
  ///
  /// Produced by `From<validator::ValidationErrors>`; serializes the same as
  /// [`ApiError::Validation`] (HTTP 400, code `VALIDATION_ERROR`) but the
  /// JSON body additionally includes an `issues` array so the frontend can
  /// surface errors on the originating form fields.
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

    // Structured validation gets a richer body with an `issues` array so the
    // frontend can attach errors to specific form fields. Every other variant
    // keeps the historical `ApiResponse<()>::error` shape exactly.
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

    // Tag so the normalisation middleware can skip body-sniffing.
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

/// Converts the validator crate's nested [`ValidationErrors`] tree into a flat
/// list of [`ValidationIssue`]s with React Hook Form-compatible field paths.
///
/// The validator crate's own `Display` impl produces square-bracket paths like
/// `items[2].accepted_amount`, but RHF uses dot+index syntax
/// (`items.2.accepted_amount`) for nested array fields. This conversion walks
/// the `Struct` / `List` / `Field` nodes and emits the latter form.
///
/// Validator codes are also normalised to a stable, frontend-friendly set
/// (e.g. `length` becomes `min_length` / `max_length` / `length` depending on
/// which params are present, `range` becomes `too_small` / `too_big`).
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

/// Maps a validator-crate error code to a stable, RHF-friendly identifier.
///
/// Compound codes (`length`, `range`) split into two depending on which bound
/// triggered the failure. The split is decided by inspecting the error's
/// `params` map (the validator crate populates `min` / `max` per the bounds
/// that were configured on the field).
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
        // Both bounds (or neither) — keep the generic `range` so callers can
        // disambiguate if needed.
        _ => "range".to_string(),
      }
    }
    "email" => "email".to_string(),
    "url" => "url".to_string(),
    "uuid" => "uuid".to_string(),
    "required" => "required".to_string(),
    "must_match" => "must_match".to_string(),
    "regex" => "regex".to_string(),
    // Custom validators already use stable identifiers — pass through verbatim.
    other => other.to_string(),
  }
}
