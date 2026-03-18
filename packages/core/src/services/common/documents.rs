use crate::{api::ApiError, enums::DocumentStatus};

pub fn ensure_doc_mod_allowed(status: DocumentStatus) -> Result<(), ApiError> {
  if status != DocumentStatus::Draft {
    return Err(ApiError::Conflict(
      "Only draft documents can be modified".to_string(),
    ));
  }
  Ok(())
}
