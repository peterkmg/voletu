use std::sync::Arc;

use sea_orm::DatabaseConnection;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};

use crate::{
  api::ApiError,
  dtos::auth::{ChangePasswordRequest, LoginRequest, LoginResponse},
  entities::{role, user},
  services::token::TokenService,
  utils::password::{hash_password, verify_password},
};

pub struct AuthService {
  db: Arc<DatabaseConnection>,
  jwt_service: Arc<TokenService>,
}

impl AuthService {
  pub fn new(db: Arc<DatabaseConnection>, jwt_service: Arc<TokenService>) -> Self {
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
      .create_access(user.id, &user.username, role.common_name.as_str())
      .await?;

    Ok(LoginResponse {
      access_token: token,
      refresh_token: "".to_string(),
      user: user.to_user_response()?,
    })
  }

  pub async fn change_password(&self, dto: &ChangePasswordRequest) -> Result<(), ApiError> {
    let user = user::Entity::find()
      .filter(user::Column::Username.eq(&dto.username))
      .one(&*self.db)
      .await?
      .ok_or(ApiError::Unauthorized("Invalid credentials".to_string()))?;

    let is_valid = verify_password(&dto.current_password, &user.password_hash)
      .await
      .map_err(|_| ApiError::Unauthorized("Invalid credentials".to_string()))?;

    if !is_valid {
      return Err(ApiError::Unauthorized("Invalid credentials".to_string()));
    }

    let mut user_model: user::ActiveModel = user.into();
    user_model.password_hash = Set(
      hash_password(&dto.new_password)
        .await
        .map_err(ApiError::Internal)?,
    );
    user_model.update(&*self.db).await?;

    Ok(())
  }
}
