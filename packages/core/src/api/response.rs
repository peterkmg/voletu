use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ErrorData {
  pub code: String,
  pub message: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
  pub success: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<T>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub error: Option<ErrorData>,
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
  fn into_response(self) -> Response {
    // Handlers return ApiResponse only on the success path; errors go through
    // ApiError::into_response().  Calling this with success=false means the
    // call site bypassed ApiError entirely, which is a bug.
    debug_assert!(
      self.success,
      "ApiResponse::into_response called with success=false; \
       error responses must be produced via ApiError"
    );
    (StatusCode::OK, Json(self)).into_response()
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

impl<T> ApiResponse<T> {
  pub fn into_anyhow_data(self, method: &str, url: &str, status: StatusCode) -> anyhow::Result<T> {
    if self.success {
      return self.data.ok_or_else(|| {
        anyhow::anyhow!(
          "{} {} succeeded but no data payload was returned",
          method,
          url
        )
      });
    }

    match self.error {
      Some(error) => anyhow::bail!(
        "{} {} failed (status {}): {} ({})",
        method,
        url,
        status,
        error.message,
        error.code
      ),
      None => anyhow::bail!(
        "{} {} failed (status {}): no error payload",
        method,
        url,
        status
      ),
    }
  }
}
