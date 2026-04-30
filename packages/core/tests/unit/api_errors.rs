use std::borrow::Cow;

use anyhow::anyhow;
use axum::{
  body::to_bytes,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use serde_json::Value;
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};
use voletu_core::api::{validation_errors_to_issues, ApiError, ValidationIssue};

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
  match &api_from_validator {
    ApiError::ValidationFields(issues) => {
      assert_eq!(issues.len(), 1, "expected one issue, got {issues:?}");
      assert_eq!(issues[0].field, "username");

      assert_eq!(issues[0].code, "length");
    }
    other => panic!("expected ApiError::ValidationFields, got {other:?}"),
  }

  assert_eq!(api_from_validator.error_code(), "VALIDATION_ERROR");
}

fn field_kind(errors: Vec<ValidationError>) -> ValidationErrorsKind {
  ValidationErrorsKind::Field(errors)
}

#[test]
fn validation_errors_convert_to_dot_index_paths() {
  let mut inner = ValidationErrors::new();
  let mut e = ValidationError::new("positive");
  e.message = Some(Cow::from("must be greater than 0"));
  inner
    .errors_mut()
    .insert(Cow::Borrowed("accepted_amount"), field_kind(vec![e]));

  let mut list_map = std::collections::BTreeMap::new();
  list_map.insert(2usize, Box::new(inner));

  let mut top = ValidationErrors::new();
  top
    .errors_mut()
    .insert(Cow::Borrowed("items"), ValidationErrorsKind::List(list_map));

  let issues = validation_errors_to_issues(&top);
  assert_eq!(issues.len(), 1);
  assert_eq!(issues[0].field, "items.2.accepted_amount");
  assert_eq!(issues[0].code, "positive");
  assert_eq!(issues[0].message, "must be greater than 0");
}

#[test]
fn validation_errors_map_length_and_range_codes() {
  let mut top = ValidationErrors::new();

  let mut min_len_err = ValidationError::new("length");
  min_len_err.add_param(Cow::from("min"), &1u64);
  top.errors_mut().insert(
    Cow::Borrowed("document_number"),
    field_kind(vec![min_len_err]),
  );

  let mut too_big_err = ValidationError::new("range");
  too_big_err.add_param(Cow::from("max"), &100u64);
  top
    .errors_mut()
    .insert(Cow::Borrowed("count"), field_kind(vec![too_big_err]));

  let issues = validation_errors_to_issues(&top);
  let by_field: std::collections::HashMap<_, _> =
    issues.iter().map(|i| (i.field.as_str(), i)).collect();

  assert_eq!(by_field["document_number"].code, "min_length");
  assert_eq!(by_field["count"].code, "too_big");
}

#[tokio::test]
async fn validation_fields_response_includes_issues_array() {
  let issues = vec![
    ValidationIssue {
      field: "items.2.accepted_amount".to_string(),
      code: "too_small".to_string(),
      message: "must be greater than 0".to_string(),
    },
    ValidationIssue {
      field: "document_number".to_string(),
      code: "min_length".to_string(),
      message: "must be at least 1 character".to_string(),
    },
  ];
  let err = ApiError::ValidationFields(issues);

  let response = err.into_response();
  assert_eq!(
    response.status(),
    StatusCode::BAD_REQUEST,
    "ValidationFields must return HTTP 400"
  );

  let body = response_json(response).await;
  assert_json_diff::assert_json_eq!(
    body,
    serde_json::json!({
      "success": false,
      "error": {
        "code": "VALIDATION_ERROR",
        "message": "Validation failed",
        "issues": [
          {
            "field": "items.2.accepted_amount",
            "code": "too_small",
            "message": "must be greater than 0"
          },
          {
            "field": "document_number",
            "code": "min_length",
            "message": "must be at least 1 character"
          }
        ]
      }
    })
  );
}

#[tokio::test]
async fn non_validation_error_response_keeps_legacy_envelope() {
  let err = ApiError::NotFound("widget 42".to_string());
  let response = err.into_response();
  assert_eq!(
    response.status(),
    StatusCode::NOT_FOUND,
    "NotFound must return HTTP 404"
  );

  let body = response_json(response).await;

  assert_eq!(
    body.get("success").and_then(|v| v.as_bool()),
    Some(false),
    "expected success=false, body was {body}"
  );
  let error_obj = body
    .get("error")
    .and_then(|v| v.as_object())
    .expect("expected error object on legacy envelope");
  assert_eq!(
    error_obj.get("code").and_then(|v| v.as_str()),
    Some("NOT_FOUND")
  );
  assert_eq!(
    error_obj.get("message").and_then(|v| v.as_str()),
    Some("Not found: widget 42")
  );
  assert!(
    !error_obj.contains_key("issues"),
    "non-validation error must not include `issues`, got {error_obj:?}"
  );
}
