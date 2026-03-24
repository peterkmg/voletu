use anyhow::anyhow;
use sea_orm::{
  ActiveModelTrait,
  ActiveValue::Set,
  ColumnTrait,
  EntityTrait,
  QueryFilter,
  TransactionTrait,
};

use super::SystemService;
use crate::{
  api::ApiError,
  constants::{DEFAULT_ADMIN_PASSWORD, DEFAULT_ADMIN_USERNAME},
  context::audit::current_actor_id,
  db::ops::load_local_bootstrap,
  dtos::CompleteInitializationRequest,
  entities::{database_instance, local, user},
  enums::RoleType,
  utils::password::{hash_password, verify_password},
};

impl SystemService {
  pub async fn complete_initialization(
    &self,
    dto: &CompleteInitializationRequest,
  ) -> Result<(), ApiError> {
    let actor_id = current_actor_id()
      .ok_or_else(|| ApiError::Unauthorized("Missing authenticated actor context".to_string()))?;

    let local = load_local_bootstrap(self.db.as_ref()).await?;

    if local.is_initialized {
      return Err(ApiError::Conflict(
        "Database initialization is already completed".to_string(),
      ));
    }

    let txn = self.db.begin().await?;

    let bootstrap_admin = user::Entity::find()
      .filter(user::Column::Username.eq(DEFAULT_ADMIN_USERNAME))
      .filter(user::Column::OriginDbId.eq(local.local_db_id))
      .filter(user::Column::RoleId.eq(RoleType::Admin.uuid()))
      .filter(user::Column::DeletedAt.is_null())
      .one(&txn)
      .await?;

    if let Some(bootstrap_admin) = bootstrap_admin {
      let has_default_password =
        verify_password(DEFAULT_ADMIN_PASSWORD, &bootstrap_admin.password_hash)
          .await
          .map_err(ApiError::Internal)?;

      if has_default_password {
        let duplicate_user = user::Entity::find()
          .filter(user::Column::Username.eq(&dto.new_username))
          .filter(user::Column::OriginDbId.eq(local.local_db_id))
          .filter(user::Column::DeletedAt.is_null())
          .one(&txn)
          .await?;

        if let Some(duplicate_user) = duplicate_user {
          if duplicate_user.id != bootstrap_admin.id {
            return Err(ApiError::Conflict(format!(
              "Username '{}' is already taken",
              dto.new_username
            )));
          }
        }

        let mut bootstrap_admin_model: user::ActiveModel = bootstrap_admin.into();
        bootstrap_admin_model.username = Set(dto.new_username.clone());
        bootstrap_admin_model.password_hash = Set(
          hash_password(&dto.new_password)
            .await
            .map_err(ApiError::Internal)?,
        );
        bootstrap_admin_model.fullname = Set(dto.fullname.clone());
        bootstrap_admin_model.updated_by = Set(actor_id);
        bootstrap_admin_model.update(&txn).await?;
      }
    }

    let requested_node_type = dto.node_type;

    let local_db_id = local.local_db_id;
    let mut local_model: local::ActiveModel = local.into();

    let needs_instance_update = requested_node_type.is_some() || dto.node_name.is_some();
    if needs_instance_update {
      let mut instance_model: database_instance::ActiveModel =
        database_instance::Entity::find_by_id(local_db_id)
          .one(&txn)
          .await?
          .ok_or_else(|| ApiError::Internal(anyhow!("Database instance row is missing")))?
          .into();
      if let Some(db_node_type) = &requested_node_type {
        instance_model.node_type = Set(*db_node_type);
      }
      if let Some(node_name) = &dto.node_name {
        instance_model.common_name = Set(node_name.clone());
      }
      instance_model.update(&txn).await?;
    }

    if let Some(central_api_url) = &dto.central_api_url {
      local_model.central_api_url = Set(Some(central_api_url.clone()));
    }

    local_model.is_initialized = Set(true);
    local_model.update(&txn).await?;

    txn.commit().await?;

    Ok(())
  }
}
