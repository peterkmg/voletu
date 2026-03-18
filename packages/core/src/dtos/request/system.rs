use voletu_core_macros::request_dto;

use crate::enums::{InitializeAdminAction, NodeType};

#[request_dto]
pub struct LoginRequest {
  #[validate(length(min = 3))]
  pub username: String,
  #[validate(length(min = 5))]
  pub password: String,
}

#[request_dto]
pub struct ChangePasswordRequest {
  #[validate(length(min = 3))]
  pub username: String,
  #[validate(length(min = 5))]
  pub current_password: String,
  #[validate(length(min = 8))]
  pub new_password: String,
}

#[request_dto]
#[validate(schema(function = "crate::dtos::validators::validate_complete_initialization_request"))]
pub struct CompleteInitializationRequest {
  pub action: InitializeAdminAction,
  pub node_type: Option<NodeType>,
  #[validate(length(min = 3, max = 50))]
  pub new_username: Option<String>,
  #[validate(length(min = 8))]
  pub new_password: Option<String>,
  #[validate(length(min = 2, max = 100))]
  pub fullname: Option<String>,
}

#[request_dto]
pub struct RefreshTokenRequest {
  #[validate(length(min = 20))]
  pub refresh_token: String,
}

#[request_dto]
pub struct CreateUserRequest {
  #[validate(length(min = 3, max = 50))]
  #[schema(example = "johndoe")]
  pub username: String,
  #[validate(length(min = 6))]
  #[schema(example = "s3cr3t!")]
  pub password: String,
  #[validate(length(min = 2, max = 100))]
  #[schema(example = "John Doe")]
  pub fullname: Option<String>,
  #[validate(length(min = 2, max = 50))]
  #[schema(example = "ADMIN")]
  pub role_name: String,
}

#[request_dto]
pub struct UpdateUserRequest {
  #[validate(length(min = 3, max = 50))]
  #[schema(example = "johndoe")]
  pub username: Option<String>,
  #[validate(length(min = 6))]
  #[schema(example = "s3cr3t!")]
  pub password: Option<String>,
  #[validate(length(min = 2, max = 100))]
  #[schema(example = "John Doe")]
  pub fullname: Option<String>,
  #[validate(length(min = 2, max = 50))]
  #[schema(example = "ADMIN")]
  pub role_name: Option<String>,
}
