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
