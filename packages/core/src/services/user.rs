use std::sync::Arc;

use sea_orm::{
  ActiveModelBehavior, ActiveModelTrait, ActiveValue::Set, DatabaseConnection, DeleteResult,
  EntityLoaderTrait, EntityTrait,
};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos::user::{CreateUserRequest, UserResponse},
  entities::{
    role::{self, RoleType},
    user,
  },
  utils::password::hash_password,
};

pub struct UserService {
  db: Arc<DatabaseConnection>,
}

impl UserService {
  pub fn new(db: Arc<DatabaseConnection>) -> Self {
    Self { db }
  }

  /// Returns all users with their roles loaded.
  pub async fn list(&self) -> Result<Vec<UserResponse>, ApiError> {
    tracing::debug!("Listing all users");

    let users: Vec<user::ModelEx> = user::Entity::load()
      .with(role::Entity)
      .all(&*self.db)
      .await?;

    tracing::debug!("Found {} user(s)", users.len());

    users
      .iter()
      .map(|u| u.to_user_response())
      .collect::<anyhow::Result<Vec<_>>>()
      .map_err(ApiError::Internal)
  }

  /// Creates a new user, hashing their password and resolving the role by name.
  pub async fn create(&self, dto: &CreateUserRequest) -> Result<UserResponse, ApiError> {
    tracing::info!(username = %dto.username, "Creating user");

    // Guard against duplicate usernames
    let existing: Option<user::ModelEx> = user::Entity::load()
      .filter_by_username(&dto.username)
      .one(&*self.db)
      .await?;

    if existing.is_some() {
      tracing::warn!(username = %dto.username, "Username already taken");
      return Err(ApiError::Conflict(format!(
        "Username '{}' is already taken",
        dto.username
      )));
    }

    let req_role = RoleType::from_str(&dto.role_name).map_err(|_| {
      tracing::warn!(role_name = %dto.role_name, "Invalid role name provided");
      ApiError::BadRequest(format!("Invalid role name '{}'", dto.role_name))
    })?;

    // Resolve role
    let role: role::ModelEx = role::Entity::load()
      .filter_by_common_name(req_role)
      .one(&*self.db)
      .await?
      .ok_or_else(|| {
        tracing::warn!(role_name = %dto.role_name, "Role not found");
        ApiError::NotFound(format!("Role '{}' not found", dto.role_name))
      })?;

    // Hash password
    let password_hash = hash_password(&dto.password)
      .await
      .map_err(ApiError::Internal)?;

    // Build and insert the active model
    let mut am = user::ActiveModel::new();
    am.username = Set(dto.username.clone());
    am.fullname = Set(dto.fullname.clone());
    am.password_hash = Set(password_hash);
    am.role_id = Set(role.id);

    let inserted = am.insert(&*self.db).await?;

    tracing::info!(id = %inserted.id, username = %inserted.username, "User created");

    // Reload with role for the response
    let user_ex = user::Entity::load()
      .filter_by_id(inserted.id)
      .with(role::Entity)
      .one(&*self.db)
      .await?
      .ok_or_else(|| ApiError::Internal(anyhow::anyhow!("User not found after creation")))?;

    user_ex.to_user_response().map_err(ApiError::Internal)
  }

  /// Deletes a user by ID. Returns `NotFound` if the user does not exist.
  pub async fn delete(&self, id: Uuid) -> Result<(), ApiError> {
    tracing::info!(id = %id, "Deleting user");

    let DeleteResult { rows_affected } = user::Entity::delete_by_id(id).exec(&*self.db).await?;

    if rows_affected == 0 {
      tracing::warn!(id = %id, "User not found for deletion");
      return Err(ApiError::NotFound(format!("User '{}' not found", id)));
    }

    tracing::info!(id = %id, "User deleted");
    Ok(())
  }
}
