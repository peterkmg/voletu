use anyhow::anyhow;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, QueryFilter, TransactionTrait};

use super::SystemService;
use crate::{
  api::ApiError,
  constants::{DEFAULT_ADMIN_PASSWORD, DEFAULT_ADMIN_USERNAME},
  context::audit::current_actor_id,
  dtos::CompleteInitializationRequest,
  entities::{database_instance, local, user},
  enums::RoleType,
  services::system::{
    database_instance::load_active_database_instance, local::load_local_bootstrap,
    user::helpers::load_local_active_user_by_username,
  },
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

    let bootstrap_admin = user::Entity::load()
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
        let duplicate_user =
          load_local_active_user_by_username(&txn, local.local_db_id, &dto.new_username).await?;

        if let Some(duplicate_user) = duplicate_user {
          if duplicate_user.id != bootstrap_admin.id {
            return Err(ApiError::Conflict(format!(
              "Username '{}' is already taken",
              dto.new_username
            )));
          }
        }

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
        .update(&txn)
        .await?;
      }
    }

    let requested_node_type = dto.node_type;

    let local_db_id = local.local_db_id;
    let mut local_model: local::ActiveModel = local.into();

    let needs_instance_update = requested_node_type.is_some() || dto.node_name.is_some();
    if needs_instance_update {
      let instance = load_active_database_instance(&txn, local_db_id)
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
