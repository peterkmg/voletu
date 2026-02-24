use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::dtos::user::UserResponse;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct LoginRequest {
  #[validate(length(min = 3))]
  pub username: String,
  #[validate(length(min = 6))]
  pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
  pub access_token: String,
  pub refresh_token: String,
  pub user: UserResponse,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct ChangePasswordRequest {
  #[validate(length(min = 3))]
  pub username: String,
  #[validate(length(min = 6))]
  pub current_password: String,
  #[validate(length(min = 8))]
  pub new_password: String,
}
