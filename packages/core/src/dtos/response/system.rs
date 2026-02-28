use uuid::Uuid;
use voletu_core_macros::response_dto;

#[response_dto]
pub struct UserResponse {
  #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
  pub id: Uuid,
  #[schema(example = "johndoe")]
  pub username: String,
  #[schema(example = "John Doe")]
  pub fullname: Option<String>,
  #[schema(example = "admin")]
  pub role: String,
}

#[response_dto]
pub struct LoginResponse {
  pub access_token: String,
  pub refresh_token: String,
  pub user: UserResponse,
}
