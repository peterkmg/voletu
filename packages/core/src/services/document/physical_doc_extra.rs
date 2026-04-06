use std::collections::HashMap;

use sea_orm::{
  ColumnTrait,
  Condition,
  EntityLoaderTrait,
  EntityTrait,
  PaginatorTrait,
  QueryFilter,
  QueryOrder,
  TransactionTrait,
};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos::{
    response::pipeline::PhysicalTransferFlatRow,
    CreatePhysicalTransferItemRequest,
    CreatePhysicalTransferRequest,
    PhysicalTransferResponse,
  },
  entities::{company, physical_storage_transfer, physical_transfer_item, product, storage},
  enums,
  services::document::DocumentService,
};

impl DocumentService {
  pub async fn physical_transfer_composite_create(
    &self,
    req: &CreatePhysicalTransferRequest,
  ) -> Result<PhysicalTransferResponse, ApiError> {
    tracing::info!(document_number = %req.document_number, "Creating physical storage transfer");
    let txn = self.db.begin().await?;
    let response = self
      .physical_transfer_composite_create_no_tx(&txn, req)
      .await?;

    txn.commit().await?;

    Ok(response)
  }

  pub async fn physical_transfer_composite_create_and_execute(
    &self,
    req: &CreatePhysicalTransferRequest,
    actor_id: Uuid,
  ) -> Result<PhysicalTransferResponse, ApiError> {
    let txn = self.db.begin().await?;

    let mut response = self
      .physical_transfer_composite_create_no_tx(&txn, req)
      .await?;

    self
      .physical_transfer_execute_no_tx(&txn, response.id, actor_id)
      .await?;

    response.status = crate::enums::DocumentStatus::Executed;

    txn.commit().await?;

    Ok(response)
  }

  pub(crate) async fn physical_transfer_composite_create_no_tx(
    &self,
    conn: &sea_orm::DatabaseTransaction,
    req: &CreatePhysicalTransferRequest,
  ) -> Result<PhysicalTransferResponse, ApiError> {
    let mut response = self.physical_transfer_create_no_tx(conn, req).await?;

    for item_req in &req.items {
      response.items.push(
        self
          .physical_item_create_no_tx(
            conn,
            &CreatePhysicalTransferItemRequest::from_composite(response.id, item_req),
          )
          .await?,
      );
    }

    Ok(response)
  }

  pub async fn physical_transfer_composite_list(
    &self,
  ) -> Result<Vec<PhysicalTransferResponse>, ApiError> {
    let docs = physical_storage_transfer::Entity::load()
      .filter(physical_storage_transfer::Column::DeletedAt.is_null())
      .with(physical_transfer_item::Entity)
      .all(self.db.as_ref())
      .await?;

    docs
      .into_iter()
      .map(PhysicalTransferResponse::try_from)
      .collect()
  }

  pub async fn physical_transfer_composite_get(
    &self,
    id: Uuid,
  ) -> Result<PhysicalTransferResponse, ApiError> {
    let doc = physical_storage_transfer::Entity::load()
      .filter_by_id(id)
      .filter(physical_storage_transfer::Column::DeletedAt.is_null())
      .with(physical_transfer_item::Entity)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Physical transfer '{}' not found", id)))?;

    PhysicalTransferResponse::try_from(doc)
  }

  pub async fn physical_transfer_composite_query(
    &self,
    document_number: Option<&str>,
    status: Option<enums::DocumentStatus>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<PhysicalTransferResponse>, ApiError> {
    let (page, per_page) = crate::services::common::normalize_pagination(page, per_page)?;

    let mut condition = Condition::all();
    condition = condition.add(physical_storage_transfer::Column::DeletedAt.is_null());

    if let Some(document_number) = document_number {
      condition =
        condition.add(physical_storage_transfer::Column::DocumentNumber.contains(document_number));
    }

    if let Some(status) = status {
      condition = condition.add(physical_storage_transfer::Column::Status.eq(status));
    }

    let docs = physical_storage_transfer::Entity::find()
      .filter(condition)
      .paginate(self.db.as_ref(), per_page)
      .fetch_page(page - 1)
      .await?;

    let mut out = Vec::with_capacity(docs.len());
    for doc in docs {
      let loaded = physical_storage_transfer::Entity::load()
        .filter_by_id(doc.id)
        .filter(physical_storage_transfer::Column::DeletedAt.is_null())
        .with(physical_transfer_item::Entity)
        .one(self.db.as_ref())
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Physical transfer '{}' not found", doc.id)))?;
      out.push(PhysicalTransferResponse::try_from(loaded)?);
    }

    Ok(out)
  }

  /// Returns one row per physical transfer item with document fields repeated.
  /// Used by the grouped-row list table on the frontend.
  ///
  /// `from_storage` is resolved via the entity loader relationship.
  /// `to_storage` has no `belongs_to` on the entity, so we resolve it
  /// in-memory by batch-fetching storage records for all `to_storage_id` values.
  pub async fn physical_transfer_flat_query(
    &self,
    status: Option<enums::DocumentStatus>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<PhysicalTransferFlatRow>, ApiError> {
    let (page, per_page) = crate::services::common::normalize_pagination(page, per_page)?;
    let db = self.db.as_ref();

    let mut cond = Condition::all().add(physical_storage_transfer::Column::DeletedAt.is_null());
    if let Some(s) = status {
      cond = cond.add(physical_storage_transfer::Column::Status.eq(s));
    }

    let docs: Vec<physical_storage_transfer::ModelEx> = physical_storage_transfer::Entity::load()
      .filter(cond)
      .with(company::Entity)
      .with((physical_transfer_item::Entity, product::Entity))
      .with((physical_transfer_item::Entity, storage::Entity)) // from_storage
      .order_by_desc(physical_storage_transfer::Column::Date)
      .paginate(db, per_page)
      .fetch_page(page - 1)
      .await?;

    // Collect all to_storage_id values and batch-fetch storage names.
    let to_storage_ids: Vec<Uuid> = docs
      .iter()
      .flat_map(|d| d.items.iter().map(|i| i.to_storage_id))
      .collect();

    let to_storage_map: HashMap<Uuid, String> = if to_storage_ids.is_empty() {
      HashMap::new()
    } else {
      let storages = storage::Entity::find()
        .filter(storage::Column::Id.is_in(to_storage_ids))
        .all(db)
        .await?;
      storages
        .into_iter()
        .map(|s| (s.id, s.common_name))
        .collect()
    };

    let mut rows = Vec::new();
    for doc in &docs {
      let contractor_name = doc
        .contractor
        .as_ref()
        .map(|c| c.common_name.clone())
        .unwrap_or("\u{2014}".to_string());

      if doc.items.is_empty() {
        rows.push(PhysicalTransferFlatRow {
          id: doc.id,
          document_id: doc.id,
          document_number: doc.document_number.clone(),
          date: doc.date.to_string(),
          status: doc.status,
          contractor_id_name: contractor_name.clone(),
          item_id: doc.id,
          product_id_name: "\u{2014}".to_string(),
          from_storage_id_name: "\u{2014}".to_string(),
          to_storage_id_name: "\u{2014}".to_string(),
          amount: Default::default(),
        });
      }
      for item in &doc.items {
        rows.push(PhysicalTransferFlatRow {
          id: doc.id,
          document_id: doc.id,
          document_number: doc.document_number.clone(),
          date: doc.date.to_string(),
          status: doc.status,
          contractor_id_name: contractor_name.clone(),
          item_id: item.id,
          product_id_name: item
            .product
            .as_ref()
            .map(|p| p.common_name.clone())
            .unwrap_or_default(),
          from_storage_id_name: item
            .from_storage
            .as_ref()
            .map(|s| s.common_name.clone())
            .unwrap_or_default(),
          to_storage_id_name: to_storage_map
            .get(&item.to_storage_id)
            .cloned()
            .unwrap_or_default(),
          amount: item.amount,
        });
      }
    }

    Ok(rows)
  }
}
