pub mod cargo_flow;
pub mod rail_receipt;
pub mod truck_dispatch;
pub mod truck_receipt;

use std::{collections::HashMap, sync::Arc};

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{
  api::ApiError,
  entities::{company, product},
};

pub struct FlowService {
  pub(crate) db: Arc<DatabaseConnection>,
}

impl FlowService {
  pub fn new(db: Arc<DatabaseConnection>) -> Self {
    Self { db }
  }

  /// Batch-resolve company UUIDs to their common names.
  pub(crate) async fn resolve_companies(
    &self,
    ids: &[Uuid],
  ) -> Result<HashMap<Uuid, String>, ApiError> {
    if ids.is_empty() {
      return Ok(HashMap::new());
    }
    let mut unique: Vec<Uuid> = ids.to_vec();
    unique.sort();
    unique.dedup();
    let companies = company::Entity::find()
      .filter(company::Column::Id.is_in(unique))
      .all(self.db.as_ref())
      .await?;
    Ok(
      companies
        .into_iter()
        .map(|c| (c.id, c.common_name))
        .collect(),
    )
  }

  /// Batch-resolve product UUIDs to their common names.
  pub(crate) async fn resolve_products(
    &self,
    ids: &[Uuid],
  ) -> Result<HashMap<Uuid, String>, ApiError> {
    if ids.is_empty() {
      return Ok(HashMap::new());
    }
    let mut unique: Vec<Uuid> = ids.to_vec();
    unique.sort();
    unique.dedup();
    let products = product::Entity::find()
      .filter(product::Column::Id.is_in(unique))
      .all(self.db.as_ref())
      .await?;
    Ok(
      products
        .into_iter()
        .map(|p| (p.id, p.common_name))
        .collect(),
    )
  }
}
