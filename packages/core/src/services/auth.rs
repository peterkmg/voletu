use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::{
  api::ApiError,
  dtos::auth::{LoginRequest, LoginResponse},
  entities::{role, user},
  services::jwt::JwtService,
  utils::hasher::verify_password,
};

pub struct AuthService {
  db: Arc<DatabaseConnection>,
  jwt_service: Arc<JwtService>,
}

impl AuthService {
  pub fn new(db: Arc<DatabaseConnection>, jwt_service: Arc<JwtService>) -> Self {
    Self { db, jwt_service }
  }

  pub async fn authenticate(&self, dto: &LoginRequest) -> Result<LoginResponse, ApiError> {
    let user = user::Entity::load()
      .filter_by_username(&dto.username)
      .with(role::Entity)
      .one(&*self.db)
      .await?
      .ok_or(ApiError::Unauthorized("Invalid credentials".to_string()))?;

    verify_password(&dto.password, &user.password_hash)
      .await
      .map_err(|_| ApiError::Unauthorized("Invalid credentials".to_string()))?;

    let role = user
      .role
      .as_ref()
      .ok_or(ApiError::Unauthorized("User role not found".to_string()))?;

    let token = self
      .jwt_service
      .create_jwt(user.id, &user.username, &role.name)
      .await?;

    Ok(LoginResponse {
      access_token: token,
      refresh_token: "".to_string(),
      user: user.to_user_response()?,
    })
  }
}
