pub mod cargo_flow;
pub mod rail_receipt;
pub mod truck_dispatch;
pub mod truck_receipt;

use std::{collections::HashMap, sync::Arc};

use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{
  api::ApiError,
  entities::{company, product},
  enums::{DocumentStatus, PipelineStatus},
};

pub struct FlowService {
  pub(crate) db: Arc<DatabaseConnection>,
}

impl FlowService {
  pub fn new(db: Arc<DatabaseConnection>) -> Self {
    Self { db }
  }

  pub(crate) async fn resolve_companies(&self, ids: &[Uuid]) -> Result<HashMap<Uuid, String>, ApiError> {
    if ids.is_empty() {
      return Ok(HashMap::new());
    }
    let mut unique = ids.to_vec();
    unique.sort();
    unique.dedup();
    Ok(
      company::Entity::find()
        .filter(company::Column::Id.is_in(unique))
        .all(self.db.as_ref())
        .await?
        .into_iter()
        .map(|c| (c.id, c.common_name))
        .collect(),
    )
  }

  pub(crate) async fn resolve_products(&self, ids: &[Uuid]) -> Result<HashMap<Uuid, String>, ApiError> {
    if ids.is_empty() {
      return Ok(HashMap::new());
    }
    let mut unique = ids.to_vec();
    unique.sort();
    unique.dedup();
    Ok(
      product::Entity::find()
        .filter(product::Column::Id.is_in(unique))
        .all(self.db.as_ref())
        .await?
        .into_iter()
        .map(|p| (p.id, p.common_name))
        .collect(),
    )
  }

  pub(crate) fn company_name(map: &HashMap<Uuid, String>, id: Uuid) -> String {
    map.get(&id).cloned().unwrap_or_else(|| "Unknown".into())
  }

  pub(crate) fn first_per_parent<'a, T>(items: &'a [T], parent_id: impl Fn(&T) -> Uuid) -> HashMap<Uuid, &'a T> {
    let mut map = HashMap::new();
    for item in items {
      map.entry(parent_id(item)).or_insert(item);
    }
    map
  }

  pub(crate) fn add_status_filter(
    cond: Condition,
    status: Option<PipelineStatus>,
    col: impl ColumnTrait,
  ) -> Option<Condition> {
    match status {
      None => Some(cond),
      Some(PipelineStatus::Pending) => None,
      Some(PipelineStatus::Draft) => Some(cond.add(col.eq(DocumentStatus::Draft))),
      Some(PipelineStatus::Executed) => Some(cond.add(col.eq(DocumentStatus::Posted))),
    }
  }
}
