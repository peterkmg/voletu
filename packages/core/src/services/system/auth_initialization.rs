use anyhow::anyhow;
use sea_orm::{
  ActiveModelTrait,
  ActiveValue::Set,
  ColumnTrait,
  DatabaseTransaction,
  QueryFilter,
  TransactionTrait,
};
use uuid::Uuid;

use super::SystemService;
use crate::{
  api::ApiError,
  constants::{DEFAULT_ADMIN_PASSWORD, DEFAULT_ADMIN_USERNAME},
  context::audit::current_actor_id,
  dtos::CompleteInitializationRequest,
  entities::{database_instance, local, user},
  enums::RoleType,
  services::system::{
    database_instance::load_active_database_instance,
    local::load_local_bootstrap,
    user::helpers::load_local_active_user_by_username,
  },
  utils::password::{hash_password, verify_password},
};

impl SystemService {
  pub async fn complete_initialization(
    &self,
    dto: &CompleteInitializationRequest,
  ) -> Result<(), ApiError> {
    let actor_id = initialization_actor_id()?;
    let local = self.load_pending_local_bootstrap().await?;

    let txn = self.db.begin().await?;

    self
      .rotate_bootstrap_admin_if_needed(&txn, local.local_db_id, dto, actor_id)
      .await?;
    self
      .update_database_instance_metadata_if_requested(&txn, local.local_db_id, dto, actor_id)
      .await?;
    self.mark_local_initialized(&txn, local, dto).await?;

    txn.commit().await?;

    Ok(())
  }

  async fn load_pending_local_bootstrap(&self) -> Result<local::Model, ApiError> {
    let local = load_local_bootstrap(self.db.as_ref()).await?;

    if local.is_initialized {
      return Err(ApiError::Conflict(
        "Database initialization is already completed".to_string(),
      ));
    }

    Ok(local)
  }

  async fn rotate_bootstrap_admin_if_needed(
    &self,
    txn: &DatabaseTransaction,
    local_db_id: Uuid,
    dto: &CompleteInitializationRequest,
    actor_id: Uuid,
  ) -> Result<(), ApiError> {
    let Some(bootstrap_admin) = self.load_bootstrap_admin(txn, local_db_id).await? else {
      return Ok(());
    };

    let has_default_password =
      verify_password(DEFAULT_ADMIN_PASSWORD, &bootstrap_admin.password_hash)
        .await
        .map_err(ApiError::Internal)?;

    if !has_default_password {
      return Ok(());
    }

    self
      .ensure_replacement_username_is_available(txn, local_db_id, dto, bootstrap_admin.id)
      .await?;

    user::ActiveModel {
      id: Set(bootstrap_admin.id),
      username: Set(dto.new_username.clone()),
      password_hash: Set(
        hash_password(&dto.new_password)
          .await
          .map_err(ApiError::Internal)?,
      ),
      fullname: Set(dto.fullname.clone()),
      updated_by: Set(actor_id),
      ..Default::default()
    }
    .update(txn)
    .await?;

    Ok(())
  }

  async fn load_bootstrap_admin(
    &self,
    txn: &DatabaseTransaction,
    local_db_id: Uuid,
  ) -> Result<Option<user::ModelEx>, ApiError> {
    user::Entity::load()
      .filter(user::Column::Username.eq(DEFAULT_ADMIN_USERNAME))
      .filter(user::Column::OriginDbId.eq(local_db_id))
      .filter(user::Column::RoleId.eq(RoleType::Admin.uuid()))
      .filter(user::Column::DeletedAt.is_null())
      .one(txn)
      .await
      .map_err(Into::into)
  }

  async fn ensure_replacement_username_is_available(
    &self,
    txn: &DatabaseTransaction,
    local_db_id: Uuid,
    dto: &CompleteInitializationRequest,
    bootstrap_admin_id: Uuid,
  ) -> Result<(), ApiError> {
    let duplicate_user =
      load_local_active_user_by_username(txn, local_db_id, &dto.new_username).await?;

    if let Some(duplicate_user) = duplicate_user {
      if duplicate_user.id != bootstrap_admin_id {
        return Err(ApiError::Conflict(format!(
          "Username '{}' is already taken",
          dto.new_username
        )));
      }
    }

    Ok(())
  }

  async fn update_database_instance_metadata_if_requested(
    &self,
    txn: &DatabaseTransaction,
    local_db_id: Uuid,
    dto: &CompleteInitializationRequest,
    actor_id: Uuid,
  ) -> Result<(), ApiError> {
    if dto.node_type.is_none() && dto.node_name.is_none() {
      return Ok(());
    }

    let instance = load_active_database_instance(txn, local_db_id)
      .await
      .map_err(|err| match err {
        ApiError::NotFound(_) => ApiError::Internal(anyhow!("Database instance row is missing")),
        other => other,
      })?;

    let mut instance_model = database_instance::ActiveModel {
      id: Set(instance.id),
      common_name: Set(instance.common_name),
      node_type: Set(instance.node_type),
      base_id: Set(instance.base_id),
      updated_by: Set(actor_id),
      ..Default::default()
    };

    if let Some(node_type) = dto.node_type {
      instance_model.node_type = Set(node_type);
    }
    if let Some(node_name) = &dto.node_name {
      instance_model.common_name = Set(node_name.clone());
    }

    instance_model.update(txn).await?;

    Ok(())
  }

  async fn mark_local_initialized(
    &self,
    txn: &DatabaseTransaction,
    local: local::Model,
    dto: &CompleteInitializationRequest,
  ) -> Result<(), ApiError> {
    let mut local_model: local::ActiveModel = local.into();

    if let Some(central_api_url) = &dto.central_api_url {
      local_model.central_api_url = Set(Some(central_api_url.clone()));
    }

    local_model.is_initialized = Set(true);
    local_model.update(txn).await?;

    Ok(())
  }
}

fn initialization_actor_id() -> Result<Uuid, ApiError> {
  current_actor_id()
    .ok_or_else(|| ApiError::Unauthorized("Missing authenticated actor context".to_string()))
}
