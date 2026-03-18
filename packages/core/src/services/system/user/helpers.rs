use uuid::Uuid;

use super::super::SystemService;
use crate::api::ApiError;

impl SystemService {
  pub(super) async fn user_local_db_id(&self) -> Result<Uuid, ApiError> {
    Ok(self.cfg.node.db_id)
  }
}
