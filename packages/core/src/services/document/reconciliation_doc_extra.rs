use std::collections::HashSet;

use sea_orm::{
  ColumnTrait,
  Condition,
  EntityLoaderTrait,
  QueryFilter,
  QueryOrder,
  TransactionTrait,
};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos::{self, response::document::ReconciliationFlatRow, InventoryReconciliationResponse},
  entities::{
    company,
    inventory_adjustment,
    inventory_reconciliation,
    product,
    storage,
    warehouse,
  },
  enums::AdjustmentType,
  services::{
    common::normalize_pagination,
    document::specs::{ReconciliationFlatQuerySpec, ReconciliationQuerySpec},
    DocumentService,
  },
};

impl DocumentService {
  pub(super) async fn reconciliation_model(
    &self,
    id: Uuid,
  ) -> Result<inventory_reconciliation::ModelEx, ApiError> {
    inventory_reconciliation::Entity::load()
      .filter_by_id(id)
      .filter(inventory_reconciliation::Column::DeletedAt.is_null())
      .with(company::Entity)
      .with(warehouse::Entity)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Reconciliation '{}' not found", id)))
  }

  pub(super) async fn reconciliation_query_models(
    &self,
    query: &ReconciliationQuerySpec,
  ) -> Result<Vec<inventory_reconciliation::ModelEx>, ApiError> {
    let (page, per_page) = normalize_pagination(query.page, query.per_page)?;

    let mut condition = Condition::all();
    condition = condition.add(inventory_reconciliation::Column::DeletedAt.is_null());

    if let Some(document_number) = query.document_number.as_deref() {
      condition =
        condition.add(inventory_reconciliation::Column::DocumentNumber.contains(document_number));
    }

    if let Some(status) = query.status {
      condition = condition.add(inventory_reconciliation::Column::Status.eq(status));
    }

    if let Some(warehouse_id) = query.warehouse_id {
      condition = condition.add(inventory_reconciliation::Column::WarehouseId.eq(warehouse_id));
    }

    inventory_reconciliation::Entity::load()
      .filter(condition)
      .with(company::Entity)
      .with(warehouse::Entity)
      .order_by_desc(inventory_reconciliation::Column::Date)
      .paginate(self.db.as_ref(), per_page)
      .fetch_page(page - 1)
      .await
      .map_err(Into::into)
  }

  pub(super) async fn reconciliation_composite_model(
    &self,
    id: Uuid,
  ) -> Result<inventory_reconciliation::ModelEx, ApiError> {
    inventory_reconciliation::Entity::load()
      .filter_by_id(id)
      .filter(inventory_reconciliation::Column::DeletedAt.is_null())
      .with(company::Entity)
      .with(warehouse::Entity)
      .with((inventory_adjustment::Entity, product::Entity))
      .with((inventory_adjustment::Entity, storage::Entity))
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Reconciliation '{}' not found", id)))
  }

  pub async fn inventory_reconciliation_composite_create(
    &self,
    req: &dtos::CreateInventoryReconciliationCompositeRequest,
  ) -> Result<dtos::InventoryReconciliationCompositeResponse, ApiError> {
    let txn = self.db.begin().await?;

    let res = self
      .inventory_reconciliation_composite_create_no_tx(&txn, req)
      .await?;

    txn.commit().await?;

    Ok(res)
  }

  pub(crate) async fn inventory_reconciliation_composite_create_no_tx(
    &self,
    conn: &sea_orm::DatabaseTransaction,
    req: &dtos::CreateInventoryReconciliationCompositeRequest,
  ) -> Result<dtos::InventoryReconciliationCompositeResponse, ApiError> {
    let saved = inventory_reconciliation::ActiveModelEx::from(req)
      .save(conn)
      .await?;

    let document_id = match saved.id {
      sea_orm::ActiveValue::Set(id) | sea_orm::ActiveValue::Unchanged(id) => id,
      sea_orm::ActiveValue::NotSet => {
        return Err(ApiError::Internal(anyhow::anyhow!(
          "reconciliation graph save returned no id"
        )));
      }
    };

    self
      .audit
      .backfill_document_routing::<inventory_reconciliation::Entity>(conn, document_id)
      .await?;

    dtos::InventoryReconciliationCompositeResponse::try_from(
      inventory_reconciliation::Entity::load()
        .filter_by_id(document_id)
        .filter(inventory_reconciliation::Column::DeletedAt.is_null())
        .with(company::Entity)
        .with(warehouse::Entity)
        .with((inventory_adjustment::Entity, product::Entity))
        .with((inventory_adjustment::Entity, storage::Entity))
        .one(conn)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Reconciliation '{}' not found", document_id)))?,
    )
  }

  pub async fn inventory_reconciliation_composite_update(
    &self,
    reconciliation_id: Uuid,
    req: &dtos::UpdateInventoryReconciliationCompositeRequest,
  ) -> Result<dtos::InventoryReconciliationCompositeResponse, ApiError> {
    let txn = self.db.begin().await?;

    let res = self
      .inventory_reconciliation_composite_update_no_tx(&txn, reconciliation_id, req)
      .await?;

    txn.commit().await?;

    Ok(res)
  }

  pub(crate) async fn inventory_reconciliation_composite_update_no_tx(
    &self,
    conn: &sea_orm::DatabaseTransaction,
    reconciliation_id: Uuid,
    req: &dtos::UpdateInventoryReconciliationCompositeRequest,
  ) -> Result<dtos::InventoryReconciliationCompositeResponse, ApiError> {
    self
      .reconciliation_update_no_tx(conn, reconciliation_id, &req.reconciliation)
      .await?;

    let mut kept_ids: HashSet<Uuid> = HashSet::new();
    for adjustment in &req.adjustments {
      if let Some(adjustment_id) = adjustment.id {
        if !kept_ids.insert(adjustment_id) {
          return Err(ApiError::BadRequest(format!(
            "duplicate adjustment id in request: {}",
            adjustment_id
          )));
        }
      }
    }

    let adjustments: Vec<inventory_adjustment::ActiveModelEx> = req
      .adjustments
      .iter()
      .map(|adjustment| inventory_adjustment::ActiveModelEx {
        id: match adjustment.id {
          Some(id) => sea_orm::ActiveValue::Unchanged(id),
          None => sea_orm::ActiveValue::NotSet,
        },
        storage_id: sea_orm::ActiveValue::Set(adjustment.storage_id),
        product_id: sea_orm::ActiveValue::Set(adjustment.product_id),
        adjustment_type: sea_orm::ActiveValue::Set(adjustment.adjustment_type),
        amount: sea_orm::ActiveValue::Set(adjustment.amount),
        reason: sea_orm::ActiveValue::Set(adjustment.reason.clone()),
        ..Default::default()
      })
      .collect();

    inventory_reconciliation::ActiveModelEx {
      id: sea_orm::ActiveValue::Unchanged(reconciliation_id),
      adjustments: sea_orm::HasManyModel::Replace(adjustments),
      ..Default::default()
    }
    .save(conn)
    .await?;

    self
      .audit
      .backfill_document_routing::<inventory_reconciliation::Entity>(conn, reconciliation_id)
      .await?;

    dtos::InventoryReconciliationCompositeResponse::try_from(
      inventory_reconciliation::Entity::load()
        .filter_by_id(reconciliation_id)
        .filter(inventory_reconciliation::Column::DeletedAt.is_null())
        .with(company::Entity)
        .with(warehouse::Entity)
        .with((inventory_adjustment::Entity, product::Entity))
        .with((inventory_adjustment::Entity, storage::Entity))
        .one(conn)
        .await?
        .ok_or_else(|| {
          ApiError::NotFound(format!("Reconciliation '{}' not found", reconciliation_id))
        })?,
    )
  }

  pub async fn inventory_reconciliation_composite_get(
    &self,
    id: Uuid,
  ) -> Result<dtos::InventoryReconciliationCompositeResponse, ApiError> {
    let doc = self.reconciliation_composite_model(id).await?;
    dtos::InventoryReconciliationCompositeResponse::try_from(doc)
  }

  pub async fn reconciliation_query(
    &self,
    query: ReconciliationQuerySpec,
  ) -> Result<Vec<InventoryReconciliationResponse>, ApiError> {
    Ok(
      self
        .reconciliation_query_models(&query)
        .await?
        .into_iter()
        .map(|model| {
          InventoryReconciliationResponse::from(inventory_reconciliation::Model::from(model))
        })
        .collect(),
    )
  }

  pub async fn reconciliation_flat_query(
    &self,
    query: ReconciliationFlatQuerySpec,
  ) -> Result<Vec<ReconciliationFlatRow>, ApiError> {
    let (page, per_page) = normalize_pagination(query.page, query.per_page)?;

    let db = self.db.as_ref();

    let mut cond = Condition::all().add(inventory_reconciliation::Column::DeletedAt.is_null());
    if let Some(s) = query.status {
      cond = cond.add(inventory_reconciliation::Column::Status.eq(s));
    }

    let docs: Vec<inventory_reconciliation::ModelEx> = inventory_reconciliation::Entity::load()
      .filter(cond)
      .with(company::Entity) // doc-level contractor
      .with(warehouse::Entity) // doc-level warehouse
      .with((inventory_adjustment::Entity, product::Entity))
      .with((inventory_adjustment::Entity, storage::Entity))
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
          adjustment_type: AdjustmentType::Surplus,
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
