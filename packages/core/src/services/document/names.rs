use std::collections::HashMap;

use sea_orm::{ColumnTrait, QueryFilter};
use uuid::Uuid;

use crate::{
  api::ApiError,
  entities::{company, storage},
  services::DocumentService,
};

fn unique_ids(ids: impl IntoIterator<Item = Uuid>) -> Vec<Uuid> {
  let mut ids = ids.into_iter().collect::<Vec<_>>();
  ids.sort_unstable();
  ids.dedup();

  ids
}

impl DocumentService {
  pub(super) async fn company_name_map(
    &self,
    company_ids: impl IntoIterator<Item = Uuid>,
  ) -> Result<HashMap<Uuid, String>, ApiError> {
    let company_ids = unique_ids(company_ids);
    if company_ids.is_empty() {
      return Ok(HashMap::new());
    }

    Ok(
      company::Entity::load()
        .filter(company::Column::Id.is_in(company_ids))
        .all(self.db.as_ref())
        .await?
        .into_iter()
        .map(|company| (company.id, company.common_name))
        .collect(),
    )
  }

  pub(super) async fn storage_name_map(
    &self,
    storage_ids: impl IntoIterator<Item = Uuid>,
  ) -> Result<HashMap<Uuid, String>, ApiError> {
    let storage_ids = unique_ids(storage_ids);
    if storage_ids.is_empty() {
      return Ok(HashMap::new());
    }

    Ok(
      storage::Entity::load()
        .filter(storage::Column::Id.is_in(storage_ids))
        .all(self.db.as_ref())
        .await?
        .into_iter()
        .map(|storage| (storage.id, storage.common_name))
        .collect(),
    )
  }
}
