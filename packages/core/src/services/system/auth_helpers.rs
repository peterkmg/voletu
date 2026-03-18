use uuid::Uuid;

use super::SystemService;
use crate::api::ApiError;

impl SystemService {
  pub(super) async fn local_db_id(&self) -> Result<Uuid, ApiError> {
    Ok(self.cfg.node.db_id)
  }
}
