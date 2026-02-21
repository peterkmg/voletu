use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorData {
  pub code: String,
  pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T: Serialize> {
  pub success: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<T>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub error: Option<ErrorData>,
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
  fn into_response(self) -> Response {
    let status = if self.success {
      StatusCode::OK
    } else {
      StatusCode::INTERNAL_SERVER_ERROR
    };
    (status, Json(self)).into_response()
  }
}

impl<T: Serialize> ApiResponse<T> {
  pub fn success(data: T) -> Self {
    Self {
      success: true,
      data: Some(data),
      error: None,
    }
  }

  pub fn error(code: String, message: String) -> Self {
    Self {
      success: false,
      data: None,
      error: Some(ErrorData { code, message }),
    }
  }
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
  #[error("Not found: {0}")]
  NotFound(String),
  #[error("Validation: {0}")]
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
      Self::Validation(_) => StatusCode::BAD_REQUEST,
      Self::Conflict(_) => StatusCode::CONFLICT,
      Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
      Self::Forbidden(_) => StatusCode::FORBIDDEN,
      Self::Internal(_) | Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }

  pub fn error_code(&self) -> &'static str {
    match self {
      Self::NotFound(_) => "NOT_FOUND",
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
    match &self {
      Self::Internal(err) => tracing::error!("Internal error: {err:#}"),
      Self::Database(err) => tracing::error!("Database error: {err}"),
      _ => {} // todo: handle other errors with appropriate logging levels and messages
    }

    let status = self.status_code();
    let body = Json(ApiResponse::<()>::error(
      self.error_code().to_string(),
      self.to_string(),
    ));
    (status, body).into_response()
  }
}

// impl From<DbErr> for ApiError {
//   fn from(db_error: DbErr) -> Self {
//     match &db_error {
//       DbErr::RecordNotFound(msg) => Self::NotFound(msg.clone()),
//       _ => Self::Database(db_error),
//     }
//   }
// }

// impl From<anyhow::Error> for ApiError {
//   fn from(err: anyhow::Error) -> Self {
//     Self::Internal(err)
//   }
// }

impl From<serde_json::Error> for ApiError {
  fn from(err: serde_json::Error) -> Self {
    Self::Validation(err.to_string())
  }
}

pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;
// impl IntoResponse for ApiResult<()> {
//   fn into_response(self) -> Response {
//     match self {
//       Ok(success) => (StatusCode::OK, Json(success)).into_response(),
//       Err(err) => err.into_response(),
//     }
//   }
// }
