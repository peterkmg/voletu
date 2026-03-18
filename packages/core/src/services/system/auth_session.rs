use anyhow::anyhow;
use chrono::Duration;
use sea_orm::{
  entity::prelude::ChronoUtc,
  ActiveModelTrait,
  ActiveValue::Set,
  ColumnTrait,
  Condition,
  EntityLoaderTrait,
  EntityTrait,
  QueryFilter,
  QueryOrder,
  TransactionTrait,
};
use uuid::Uuid;

use super::SystemService;
use crate::{
  api::ApiError,
  dtos::{LoginRequest, LoginResponse, RefreshTokenResponse, UserResponse},
  entities::{refresh_token, role, user},
  utils::{
    jwt::parse_refresh_token,
    password::{hash_password, verify_password},
  },
};

impl SystemService {
  pub async fn authenticate(&self, dto: &LoginRequest) -> Result<LoginResponse, ApiError> {
    let local_db_id = self.local_db_id().await?;
    let user = user::Entity::load()
      .filter(user::Column::Username.eq(&dto.username))
      .filter(user::Column::OriginDbId.eq(local_db_id))
      .filter(user::Column::DeletedAt.is_null())
      .with(role::Entity)
      .one(self.db.as_ref())
      .await?
      .ok_or(ApiError::Unauthorized("Invalid credentials".to_string()))?;

    let is_valid = verify_password(&dto.password, &user.password_hash)
      .await
      .map_err(|_| ApiError::Unauthorized("Invalid credentials".to_string()))?;
    if !is_valid {
      return Err(ApiError::Unauthorized("Invalid credentials".to_string()));
    }

    let role = user
      .role
      .as_ref()
      .ok_or(ApiError::Unauthorized("User role not found".to_string()))?;
    let role_name = role.common_name.to_string();

    let access_token = self
      .access_create(user.id, &user.username, &role_name)
      .await?;
    let refresh_token = self
      .issue_refresh_token(self.db.as_ref(), user.id, None)
      .await?;

    Ok(LoginResponse {
      access_token,
      refresh_token,
      user: UserResponse::try_from(&user).map_err(ApiError::Internal)?,
    })
  }

  pub async fn refresh_access_token(&self, token: &str) -> Result<LoginResponse, ApiError> {
    let local_db_id = self.local_db_id().await?;
    let (refresh_id, refresh_secret) = parse_refresh_token(token)?;
    let txn = self.db.begin().await?;

    let stored = refresh_token::Entity::find_by_id(refresh_id)
      .one(&txn)
      .await?
      .ok_or(ApiError::Unauthorized("Invalid refresh token".to_string()))?;

    if stored.is_revoked || stored.expires_at <= ChronoUtc::now() {
      return Err(ApiError::Unauthorized(
        "Refresh token expired or revoked".to_string(),
      ));
    }

    let is_valid = verify_password(&refresh_secret, &stored.token_hash)
      .await
      .map_err(|_| ApiError::Unauthorized("Invalid refresh token".to_string()))?;
    if !is_valid {
      return Err(ApiError::Unauthorized("Invalid refresh token".to_string()));
    }

    let mut refresh_model: refresh_token::ActiveModel = stored.clone().into();
    refresh_model.is_revoked = Set(true);
    refresh_model.update(&txn).await?;

    let user = user::Entity::load()
      .filter_by_id(stored.user_id)
      .filter(user::Column::OriginDbId.eq(local_db_id))
      .filter(user::Column::DeletedAt.is_null())
      .with(role::Entity)
      .one(&txn)
      .await?
      .ok_or(ApiError::Unauthorized(
        "User not found or inactive".to_string(),
      ))?;

    let role = user
      .role
      .as_ref()
      .ok_or(ApiError::Unauthorized("User role not found".to_string()))?;
    let role_name = role.common_name.to_string();

    let access_token = self
      .access_create(user.id, &user.username, &role_name)
      .await?;
    let refresh_token = self
      .issue_refresh_token(&txn, user.id, stored.device_info.clone())
      .await?;

    txn.commit().await?;

    Ok(LoginResponse {
      access_token,
      refresh_token,
      user: UserResponse::try_from(&user).map_err(ApiError::Internal)?,
    })
  }

  pub(super) async fn issue_refresh_token<C: sea_orm::ConnectionTrait>(
    &self,
    conn: &C,
    user_id: Uuid,
    device_info: Option<String>,
  ) -> Result<String, ApiError> {
    let refresh_secret = self.create_refresh_secret();
    let token_hash = hash_password(&refresh_secret)
      .await
      .map_err(ApiError::Internal)?;
    let expires_at = ChronoUtc::now()
      .checked_add_signed(
        Duration::try_seconds(self.refresh_expiration_seconds())
          .ok_or_else(|| ApiError::Internal(anyhow!("Invalid refresh token duration")))?,
      )
      .ok_or_else(|| ApiError::Internal(anyhow!("Invalid refresh token expiration")))?;

    let saved = refresh_token::ActiveModel {
      user_id: Set(user_id),
      token_hash: Set(token_hash),
      expires_at: Set(expires_at),
      is_revoked: Set(false),
      device_info: Set(device_info),
      ..Default::default()
    }
    .insert(conn)
    .await?;

    Ok(format!("{}.{}", saved.id, refresh_secret))
  }

  pub(super) async fn revoke_user_refresh_tokens(&self, user_id: Uuid) -> Result<(), ApiError> {
    let rows = refresh_token::Entity::find()
      .filter(refresh_token::Column::UserId.eq(user_id))
      .filter(refresh_token::Column::IsRevoked.eq(false))
      .all(self.db.as_ref())
      .await?;

    for row in rows {
      let mut model: refresh_token::ActiveModel = row.into();
      model.is_revoked = Set(true);
      model.update(self.db.as_ref()).await?;
    }

    Ok(())
  }

  pub async fn refresh_token_list(&self) -> Result<Vec<RefreshTokenResponse>, ApiError> {
    let rows = refresh_token::Entity::find()
      .order_by_desc(refresh_token::Column::CreatedAt)
      .all(self.db.as_ref())
      .await
      .map_err(ApiError::Database)?;

    Ok(rows.iter().map(RefreshTokenResponse::from).collect())
  }

  pub async fn refresh_token_get(&self, id: Uuid) -> Result<RefreshTokenResponse, ApiError> {
    let row = refresh_token::Entity::find_by_id(id)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Refresh token '{}' not found", id)))?;

    Ok((&row).into())
  }

  pub async fn refresh_token_query(
    &self,
    user_id: Option<Uuid>,
    is_revoked: Option<bool>,
  ) -> Result<Vec<RefreshTokenResponse>, ApiError> {
    let mut condition = Condition::all();

    if let Some(user_id) = user_id {
      condition = condition.add(refresh_token::Column::UserId.eq(user_id));
    }

    if let Some(is_revoked) = is_revoked {
      condition = condition.add(refresh_token::Column::IsRevoked.eq(is_revoked));
    }

    let rows = refresh_token::Entity::find()
      .filter(condition)
      .order_by_desc(refresh_token::Column::CreatedAt)
      .all(self.db.as_ref())
      .await
      .map_err(ApiError::Database)?;

    Ok(rows.iter().map(RefreshTokenResponse::from).collect())
  }
}
