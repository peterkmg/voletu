use sea_orm::{ColumnTrait, ConnectionTrait, EntityLoaderTrait, QueryFilter};
use uuid::Uuid;

use super::super::SystemService;
use crate::{api::ApiError, entities::user};

impl SystemService {
  pub(super) async fn user_local_db_id(&self) -> Result<Uuid, ApiError> {
    Ok(self.cfg.node.db_id)
  }
}

pub(in crate::services::system) async fn load_local_user_by_id<C: ConnectionTrait>(
  conn: &C,
  local_db_id: Uuid,
  id: Uuid,
) -> Result<Option<user::ModelEx>, ApiError> {
  user::Entity::load()
    .filter_by_id(id)
    .filter(user::Column::OriginDbId.eq(local_db_id))
    .one(conn)
    .await
    .map_err(Into::into)
}

pub(in crate::services::system) async fn load_local_active_user_by_id<C: ConnectionTrait>(
  conn: &C,
  local_db_id: Uuid,
  id: Uuid,
) -> Result<Option<user::ModelEx>, ApiError> {
  user::Entity::load()
    .filter_by_id(id)
    .filter(user::Column::OriginDbId.eq(local_db_id))
    .filter(user::Column::DeletedAt.is_null())
    .one(conn)
    .await
    .map_err(Into::into)
}

pub(in crate::services::system) async fn load_local_active_user_by_username<C: ConnectionTrait>(
  conn: &C,
  local_db_id: Uuid,
  username: &str,
) -> Result<Option<user::ModelEx>, ApiError> {
  user::Entity::load()
    .filter(user::Column::Username.eq(username))
    .filter(user::Column::OriginDbId.eq(local_db_id))
    .filter(user::Column::DeletedAt.is_null())
    .one(conn)
    .await
    .map_err(Into::into)
}
