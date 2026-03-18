use anyhow::anyhow;
use axum::{
  body::to_bytes,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use serde_json::Value;
use validator::{ValidationError, ValidationErrors};
use voletu_core::api::ApiError;

async fn response_json(response: Response) -> Value {
  let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
  serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn api_error_into_response_uses_expected_status_and_error_code_mapping() {
  let cases = [
    (
      ApiError::NotFound("missing".to_string()),
      StatusCode::NOT_FOUND,
      "NOT_FOUND",
    ),
    (
      ApiError::BadRequest("bad input".to_string()),
      StatusCode::BAD_REQUEST,
      "BAD_REQUEST",
    ),
    (
      ApiError::Validation("invalid".to_string()),
      StatusCode::BAD_REQUEST,
      "VALIDATION_ERROR",
    ),
    (
      ApiError::Conflict("taken".to_string()),
      StatusCode::CONFLICT,
      "CONFLICT",
    ),
    (
      ApiError::Unauthorized("nope".to_string()),
      StatusCode::UNAUTHORIZED,
      "UNAUTHORIZED",
    ),
    (
      ApiError::Forbidden("denied".to_string()),
      StatusCode::FORBIDDEN,
      "FORBIDDEN",
    ),
    (
      ApiError::Internal(anyhow!("boom")),
      StatusCode::INTERNAL_SERVER_ERROR,
      "INTERNAL_ERROR",
    ),
    (
      ApiError::Database(sea_orm::DbErr::Custom("db boom".to_string())),
      StatusCode::INTERNAL_SERVER_ERROR,
      "DATABASE_ERROR",
    ),
  ];

  for (err, expected_status, expected_code) in cases {
    let response = err.into_response();
    assert_eq!(response.status(), expected_status);
    let json = response_json(response).await;
    assert_eq!(json["success"], false);
    assert!(json["data"].is_null());
    assert_eq!(json["error"]["code"], expected_code);
    assert!(json["error"]["message"].is_string());
  }
}

#[tokio::test]
async fn api_error_from_serde_json_and_validator_errors_maps_to_validation_error_variant() {
  let serde_err = serde_json::from_str::<Value>("{not json").unwrap_err();
  let api_from_serde: ApiError = serde_err.into();
  assert!(matches!(api_from_serde, ApiError::Validation(_)));

  let mut validation_errors = ValidationErrors::new();
  validation_errors.add("username", ValidationError::new("length"));
  let api_from_validator: ApiError = validation_errors.into();
  assert!(matches!(api_from_validator, ApiError::Validation(_)));
}
