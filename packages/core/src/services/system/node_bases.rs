use sea_orm::{ColumnTrait, ConnectionTrait, DbErr, QueryFilter};
use uuid::Uuid;

use crate::entities::node_base_assignment;

pub async fn load_node_base_assignments<C: ConnectionTrait>(
  conn: &C,
  node_id: Uuid,
) -> Result<Vec<node_base_assignment::ModelEx>, DbErr> {
  node_base_assignment::Entity::load()
    .filter(node_base_assignment::Column::NodeId.eq(node_id))
    .all(conn)
    .await
}

pub async fn load_node_base_assignment<C: ConnectionTrait>(
  conn: &C,
  node_id: Uuid,
  base_id: Uuid,
) -> Result<Option<node_base_assignment::ModelEx>, DbErr> {
  node_base_assignment::Entity::load()
    .filter(node_base_assignment::Column::NodeId.eq(node_id))
    .filter(node_base_assignment::Column::BaseId.eq(base_id))
    .one(conn)
    .await
}

pub async fn load_node_base_ids<C: ConnectionTrait>(
  conn: &C,
  node_id: Uuid,
) -> Result<Vec<Uuid>, DbErr> {
  Ok(
    load_node_base_assignments(conn, node_id)
      .await?
      .into_iter()
      .map(|row| row.base_id)
      .collect(),
  )
}
