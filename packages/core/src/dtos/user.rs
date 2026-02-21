use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
  #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
  pub id: Uuid,
  #[schema(example = "johndoe")]
  pub username: String,
  #[schema(example = "John Doe")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub fullname: Option<String>,
  #[schema(example = "admin")]
  pub role: String,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateUserRequest {
  #[validate(length(min = 3, max = 50))]
  #[schema(example = "johndoe")]
  pub username: String,
  #[validate(length(min = 6))]
  #[schema(example = "s3cr3t!")]
  pub password: String,
  #[schema(example = "John Doe")]
  pub fullname: Option<String>,
  #[schema(example = "admin")]
  pub role_name: String,
}
