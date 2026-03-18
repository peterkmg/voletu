use sea_orm::{ActiveModelTrait, ActiveValue::Set, ConnectionTrait, ModelTrait};
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

use super::AuditService;
use crate::{
  api::ApiError,
  context::audit::{current_actor_id, current_origin_db_id},
  db::ops::model_table_name,
  entities::audit_log,
  enums::AuditAction,
};

fn serialize_snapshot<M: Serialize>(model: &M) -> Result<Value, ApiError> {
  serde_json::to_value(model)
    .map_err(|error| ApiError::BadRequest(format!("Failed to serialize audit snapshot: {error}")))
}

impl AuditService {
  pub async fn register_insert<C: ConnectionTrait, M: ModelTrait + serde::Serialize>(
    &self,
    conn: &C,
    record_id: Uuid,
    new_model: &M,
  ) -> Result<(), ApiError> {
    let table_name = model_table_name::<M>();

    let new_val = serialize_snapshot(new_model)?;

    self
      .register_action(
        conn,
        &table_name,
        record_id,
        AuditAction::Insert,
        None,
        Some(new_val),
      )
      .await
  }

  pub async fn register_update<C: ConnectionTrait, M: ModelTrait + serde::Serialize>(
    &self,
    conn: &C,
    record_id: Uuid,
    old_model: &M,
    new_model: &M,
  ) -> Result<(), ApiError> {
    let table_name = model_table_name::<M>();

    let old_val = serialize_snapshot(old_model)?;
    let new_val = serialize_snapshot(new_model)?;

    if old_val == new_val {
      return Ok(());
    }

    self
      .register_action(
        conn,
        &table_name,
        record_id,
        AuditAction::Update,
        Some(old_val),
        Some(new_val),
      )
      .await
  }

  pub async fn register_delete<C: ConnectionTrait, M: ModelTrait + serde::Serialize>(
    &self,
    conn: &C,
    record_id: Uuid,
    old_model: &M,
  ) -> Result<(), ApiError> {
    let table_name = model_table_name::<M>();

    let old_val = serialize_snapshot(old_model)?;

    self
      .register_action(
        conn,
        &table_name,
        record_id,
        AuditAction::HardDelete,
        Some(old_val),
        None,
      )
      .await
  }

  async fn register_action<C: ConnectionTrait>(
    &self,
    conn: &C,
    table_name: &str,
    record_id: Uuid,
    action: AuditAction,
    old_values: Option<Value>,
    new_values: Option<Value>,
  ) -> Result<(), ApiError> {
    let actor_id = current_actor_id()
      .ok_or_else(|| ApiError::Unauthorized("Missing authenticated actor context".to_string()))?;
    let origin_db_id = current_origin_db_id().unwrap_or(self.cfg.node.db_id);

    audit_log::ActiveModel {
      table_name: Set(table_name.to_string()),
      record_id: Set(record_id),
      action: Set(action),
      old_values: Set(old_values),
      new_values: Set(new_values),
      target_base_ids: Set(String::new()),
      user_role_weight: Set(0),
      user_id: Set(actor_id),
      origin_db_id: Set(origin_db_id),
      ..Default::default()
    }
    .insert(conn)
    .await?;

    Ok(())
  }
}
