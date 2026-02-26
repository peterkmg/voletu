use std::future::Future;

use sea_orm::DbErr;
use uuid::Uuid;

#[derive(Clone, Copy, Debug)]
pub struct AuditContext {
  pub actor_id: Uuid,
  pub origin_db_id: Uuid,
}

tokio::task_local! {
  static AUDIT_CONTEXT: AuditContext;
}

pub async fn with_audit_context<F, Fut, T>(actor_id: Uuid, origin_db_id: Uuid, f: F) -> T
where
  F: FnOnce() -> Fut,
  Fut: Future<Output = T>,
{
  AUDIT_CONTEXT
    .scope(
      AuditContext {
        actor_id,
        origin_db_id,
      },
      f(),
    )
    .await
}

pub fn current_actor_id() -> Option<Uuid> {
  AUDIT_CONTEXT.try_with(|ctx| ctx.actor_id).ok()
}

pub fn current_origin_db_id() -> Option<Uuid> {
  AUDIT_CONTEXT.try_with(|ctx| ctx.origin_db_id).ok()
}

pub fn current_actor_id_or_err() -> Result<Uuid, DbErr> {
  AUDIT_CONTEXT
    .try_with(|ctx| ctx.actor_id)
    .map_err(|_| DbErr::Custom("Missing audit actor context".to_string()))
}

pub fn current_origin_db_id_or_err() -> Result<Uuid, DbErr> {
  AUDIT_CONTEXT
    .try_with(|ctx| ctx.origin_db_id)
    .map_err(|_| DbErr::Custom("Missing audit origin_db_id context".to_string()))
}
