use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos::InventoryReconciliationResponse,
  entities::inventory_reconciliation,
  enums::DocumentStatus,
  services::DocumentService,
};

impl DocumentService {
  pub async fn reconciliation_query(
    &self,
    document_number: Option<&str>,
    status: Option<DocumentStatus>,
    warehouse_id: Option<Uuid>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<InventoryReconciliationResponse>, ApiError> {
    let (page, per_page) = crate::services::common::normalize_pagination(page, per_page)?;

    let mut condition = Condition::all();
    condition = condition.add(inventory_reconciliation::Column::DeletedAt.is_null());

    if let Some(document_number) = document_number {
      condition =
        condition.add(inventory_reconciliation::Column::DocumentNumber.contains(document_number));
    }

    if let Some(status) = status {
      condition = condition.add(inventory_reconciliation::Column::Status.eq(status));
    }

    if let Some(warehouse_id) = warehouse_id {
      condition = condition.add(inventory_reconciliation::Column::WarehouseId.eq(warehouse_id));
    }

    let docs = inventory_reconciliation::Entity::find()
      .filter(condition)
      .paginate(self.db.as_ref(), per_page)
      .fetch_page(page - 1)
      .await?;

    Ok(
      docs
        .into_iter()
        .map(InventoryReconciliationResponse::from)
        .collect(),
    )
  }
}
