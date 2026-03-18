mod auth;
mod crud;
mod delete;
mod documents;
mod fields;
mod storage;

pub use auth::{ensure_senior_supervisor_or_higher, ensure_supervisor_or_higher};
pub use crud::{
  get_active_by_id,
  get_soft_delete_target_by_id,
  insert_with_audit,
  list_active,
  update_with_audit,
};
pub use delete::{hard_delete_with_audit, map_hard_delete_db_error};
pub use documents::ensure_doc_mod_allowed;
pub use fields::{set_if_some, set_if_some_mapped, set_soft_deleted_fields};
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
