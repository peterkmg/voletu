use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct LoggingConfig {
  pub filter: String,
  pub log_file: PathBuf,
  pub enabled: bool,
}

impl LoggingConfig {
  pub fn new(filter: String, log_file: PathBuf) -> Self {
    Self {
      filter,
      log_file,
      enabled: true,
    }
  }

  pub fn disabled() -> Self {
    Self {
      filter: "off".to_string(),
      log_file: PathBuf::new(),
      enabled: false,
    }
  }
}
