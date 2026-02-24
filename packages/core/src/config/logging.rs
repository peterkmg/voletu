use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct LoggingConfig {
  pub filter: String,
  pub log_file: PathBuf,
}

impl LoggingConfig {
  pub fn new(filter: String, log_file: PathBuf) -> Self {
    Self { filter, log_file }
  }
}
