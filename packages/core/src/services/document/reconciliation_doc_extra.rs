use sea_orm::{
  ColumnTrait, Condition, EntityLoaderTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos::{response::pipeline::ReconciliationFlatRow, InventoryReconciliationResponse},
  entities::{company, inventory_adjustment, inventory_reconciliation, product, storage, warehouse},
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

  /// Returns one row per reconciliation adjustment with document fields repeated.
  /// Used by the grouped-row list table on the frontend.
  pub async fn reconciliation_flat_query(
    &self,
    status: Option<DocumentStatus>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<ReconciliationFlatRow>, ApiError> {
    let (page, per_page) = crate::services::common::normalize_pagination(page, per_page)?;
    let db = self.db.as_ref();

    let mut cond = Condition::all().add(inventory_reconciliation::Column::DeletedAt.is_null());
    if let Some(s) = status {
      cond = cond.add(inventory_reconciliation::Column::Status.eq(s));
    }

    let docs: Vec<inventory_reconciliation::ModelEx> = inventory_reconciliation::Entity::load()
      .filter(cond)
      .with(company::Entity) // doc-level contractor
      .with(warehouse::Entity) // doc-level warehouse
      .with((inventory_adjustment::Entity, product::Entity))
      .with((inventory_adjustment::Entity, storage::Entity))
      .with((inventory_adjustment::Entity, company::Entity)) // adjustment-level contractor
      .order_by_desc(inventory_reconciliation::Column::Date)
      .paginate(db, per_page)
      .fetch_page(page - 1)
      .await?;

    let dash = "\u{2014}".to_string();

    let mut rows = Vec::new();
    for doc in &docs {
      let contractor_name = doc
        .contractor
        .as_ref()
        .map(|c| c.common_name.clone())
        .unwrap_or(dash.clone());
      let warehouse_name = doc
        .warehouse
        .as_ref()
        .map(|w| w.common_name.clone())
        .unwrap_or(dash.clone());

      if doc.adjustments.is_empty() {
        rows.push(ReconciliationFlatRow {
          id: doc.id,
          document_id: doc.id,
          document_number: doc.document_number.clone(),
          date: doc.date.to_string(),
          status: doc.status,
          contractor_id_name: contractor_name.clone(),
          warehouse_id_name: warehouse_name.clone(),
          item_id: doc.id,
          product_id_name: dash.clone(),
          storage_id_name: dash.clone(),
          adjustment_type: crate::enums::AdjustmentType::Surplus,
          amount: Default::default(),
          reason: None,
        });
      }
      for adj in &doc.adjustments {
        rows.push(ReconciliationFlatRow {
          id: doc.id,
          document_id: doc.id,
          document_number: doc.document_number.clone(),
          date: doc.date.to_string(),
          status: doc.status,
          contractor_id_name: contractor_name.clone(),
          warehouse_id_name: warehouse_name.clone(),
          item_id: adj.id,
          product_id_name: adj
            .product
            .as_ref()
            .map(|p| p.common_name.clone())
            .unwrap_or_default(),
          storage_id_name: adj
            .storage
            .as_ref()
            .map(|s| s.common_name.clone())
            .unwrap_or_default(),
          adjustment_type: adj.adjustment_type,
          amount: adj.amount,
          reason: adj.reason.clone(),
        });
      }
    }

    Ok(rows)
  }
}
