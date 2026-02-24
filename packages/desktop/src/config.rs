use anyhow::Context;
use serde::{Deserialize, Serialize};
use voletu_core::{DbParams, JwtConfig};

use crate::constants::{CONFY_APP_NAME, CONFY_CONFIG_NAME};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AppMode {
  Remote,
  Local,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
  pub mode: Option<AppMode>,
  pub remote_api_url: Option<String>,
  pub db_params: Option<DbParams>,
  pub jwt_config: Option<JwtConfig>,
}

impl AppConfig {
  pub fn load() -> anyhow::Result<Self> {
    confy::load(CONFY_APP_NAME, CONFY_CONFIG_NAME).context("failed to load confy config")
  }

  pub fn save(&self) -> anyhow::Result<()> {
    confy::store(CONFY_APP_NAME, CONFY_CONFIG_NAME, self).context("failed to store confy config")
  }

  pub fn resolved_mode(&self, has_password: bool) -> Option<AppMode> {
    match self.mode {
      Some(AppMode::Remote) => self
        .remote_api_url
        .as_ref()
        .filter(|url| !url.trim().is_empty())
        .map(|_| AppMode::Remote),
      Some(AppMode::Local)
        if self.db_params.is_some() && self.jwt_config.is_some() && has_password =>
      {
        Some(AppMode::Local)
      }
      _ => None,
    }
  }

  pub fn reset(&mut self) {
    self.mode = None;
    self.remote_api_url = None;
    self.db_params = None;
    self.jwt_config = None;
  }
}
