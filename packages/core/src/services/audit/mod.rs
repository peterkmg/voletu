mod register;

use std::sync::Arc;

use crate::config::ApiConfig;

pub struct AuditService {
  cfg: Arc<ApiConfig>,
}

impl AuditService {
  pub fn new(cfg: Arc<ApiConfig>) -> Self {
    Self { cfg }
  }
}
