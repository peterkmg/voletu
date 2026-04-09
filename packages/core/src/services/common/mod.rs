mod auth;
mod documents;
mod fields;
mod storage;

pub use auth::{ensure_senior_supervisor_or_higher, ensure_supervisor_or_higher};
pub use documents::ensure_doc_mod_allowed;
pub use fields::{set_if_some, set_if_some_mapped};
pub use storage::ensure_storage_accepts_product;

use crate::api::ApiError;

pub type SvcResult<T> = Result<T, ApiError>;

pub fn normalize_pagination(page: Option<u64>, per_page: Option<u64>) -> SvcResult<(u64, u64)> {
  let page = page.unwrap_or(1);
  let per_page = per_page.unwrap_or(50);

  if page == 0 || per_page == 0 {
    return Err(ApiError::Validation(
      "page and per_page must be >= 1".to_string(),
    ));
  }

  Ok((page, per_page))
}
