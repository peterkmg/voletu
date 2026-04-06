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
    response::pipeline::OwnershipTransferFlatRow,
    CreateOwnershipTransferItemRequest,
    CreateOwnershipTransferRequest,
    OwnershipTransferResponse,
  },
  entities::{company, ownership_transfer, ownership_transfer_item, product, storage},
  enums,
  services::document::DocumentService,
};

impl DocumentService {
  pub async fn ownership_transfer_composite_create(
    &self,
    req: &CreateOwnershipTransferRequest,
  ) -> Result<OwnershipTransferResponse, ApiError> {
    tracing::info!("Creating ownership transfer");
    let txn = self.db.begin().await?;
    let response = self
      .ownership_transfer_composite_create_no_tx(&txn, req)
      .await?;

    txn.commit().await?;

    Ok(response)
  }

  pub async fn ownership_transfer_composite_create_and_execute(
    &self,
    req: &CreateOwnershipTransferRequest,
    actor_id: Uuid,
  ) -> Result<OwnershipTransferResponse, ApiError> {
    let txn = self.db.begin().await?;

    let mut response = self
      .ownership_transfer_composite_create_no_tx(&txn, req)
      .await?;

    self
      .ownership_transfer_execute_no_tx(&txn, response.id, actor_id)
      .await?;

    response.status = crate::enums::DocumentStatus::Executed;

    txn.commit().await?;

    Ok(response)
  }

  pub(crate) async fn ownership_transfer_composite_create_no_tx(
    &self,
    conn: &sea_orm::DatabaseTransaction,
    req: &CreateOwnershipTransferRequest,
  ) -> Result<OwnershipTransferResponse, ApiError> {
    let mut response = self.ownership_transfer_create_no_tx(conn, req).await?;

    for item_req in &req.items {
      response.items.push(
        self
          .ownership_item_create_no_tx(
            conn,
            &CreateOwnershipTransferItemRequest::from_composite(response.id, item_req),
          )
          .await?,
      );
    }

    Ok(response)
  }

  pub async fn ownership_transfer_composite_list(
    &self,
  ) -> Result<Vec<OwnershipTransferResponse>, ApiError> {
    let docs = ownership_transfer::Entity::load()
      .filter(ownership_transfer::Column::DeletedAt.is_null())
      .with(ownership_transfer_item::Entity)
      .all(self.db.as_ref())
      .await?;

    docs
      .into_iter()
      .map(OwnershipTransferResponse::try_from)
      .collect()
  }

  pub async fn ownership_transfer_composite_get(
    &self,
    id: Uuid,
  ) -> Result<OwnershipTransferResponse, ApiError> {
    let doc = ownership_transfer::Entity::load()
      .filter_by_id(id)
      .filter(ownership_transfer::Column::DeletedAt.is_null())
      .with(ownership_transfer_item::Entity)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Ownership transfer '{}' not found", id)))?;

    OwnershipTransferResponse::try_from(doc)
  }

  pub async fn ownership_transfer_composite_query(
    &self,
    status: Option<enums::DocumentStatus>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<OwnershipTransferResponse>, ApiError> {
    let (page, per_page) = crate::services::common::normalize_pagination(page, per_page)?;

    let mut condition = Condition::all();
    condition = condition.add(ownership_transfer::Column::DeletedAt.is_null());

    if let Some(status) = status {
      condition = condition.add(ownership_transfer::Column::Status.eq(status));
    }

    let docs = ownership_transfer::Entity::find()
      .filter(condition)
      .paginate(self.db.as_ref(), per_page)
      .fetch_page(page - 1)
      .await?;

    let mut out = Vec::with_capacity(docs.len());
    for doc in docs {
      let loaded = ownership_transfer::Entity::load()
        .filter_by_id(doc.id)
        .filter(ownership_transfer::Column::DeletedAt.is_null())
        .with(ownership_transfer_item::Entity)
        .one(self.db.as_ref())
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Ownership transfer '{}' not found", doc.id)))?;
      out.push(OwnershipTransferResponse::try_from(loaded)?);
    }

    Ok(out)
  }

  /// Returns one row per ownership transfer item with document fields repeated.
  /// Used by the grouped-row list table on the frontend.
  pub async fn ownership_transfer_flat_query(
    &self,
    status: Option<enums::DocumentStatus>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<OwnershipTransferFlatRow>, ApiError> {
    let (page, per_page) = crate::services::common::normalize_pagination(page, per_page)?;
    let db = self.db.as_ref();

    let mut cond = Condition::all().add(ownership_transfer::Column::DeletedAt.is_null());
    if let Some(s) = status {
      cond = cond.add(ownership_transfer::Column::Status.eq(s));
    }

    let docs: Vec<ownership_transfer::ModelEx> = ownership_transfer::Entity::load()
      .filter(cond)
      .with((ownership_transfer_item::Entity, product::Entity))
      .with((ownership_transfer_item::Entity, storage::Entity))
      .order_by_desc(ownership_transfer::Column::Date)
      .paginate(db, per_page)
      .fetch_page(page - 1)
      .await?;

    // Collect all unique contractor IDs from items to batch-fetch names
    let mut contractor_ids = std::collections::HashSet::new();
    for doc in &docs {
      for item in &doc.items {
        contractor_ids.insert(item.from_contractor_id);
        contractor_ids.insert(item.to_contractor_id);
      }
    }

    let contractor_map: HashMap<Uuid, String> = if contractor_ids.is_empty() {
      HashMap::new()
    } else {
      let companies = company::Entity::find()
        .filter(company::Column::Id.is_in(contractor_ids))
        .all(db)
        .await?;
      companies
        .into_iter()
        .map(|c| (c.id, c.common_name))
        .collect()
    };

    let dash = "\u{2014}".to_string();

    let mut rows = Vec::new();
    for doc in &docs {
      if doc.items.is_empty() {
        rows.push(OwnershipTransferFlatRow {
          id: doc.id,
          document_id: doc.id,
          date: doc.date.to_string(),
          status: doc.status,
          item_id: doc.id,
          product_id_name: dash.clone(),
          storage_id_name: dash.clone(),
          from_contractor_id_name: dash.clone(),
          to_contractor_id_name: dash.clone(),
          amount: Default::default(),
        });
      }
      for item in &doc.items {
        rows.push(OwnershipTransferFlatRow {
          id: doc.id,
          document_id: doc.id,
          date: doc.date.to_string(),
          status: doc.status,
          item_id: item.id,
          product_id_name: item
            .product
            .as_ref()
            .map(|p| p.common_name.clone())
            .unwrap_or_default(),
          storage_id_name: item
            .storage
            .as_ref()
            .map(|s| s.common_name.clone())
            .unwrap_or_default(),
          from_contractor_id_name: contractor_map
            .get(&item.from_contractor_id)
            .cloned()
            .unwrap_or(dash.clone()),
          to_contractor_id_name: contractor_map
            .get(&item.to_contractor_id)
            .cloned()
            .unwrap_or(dash.clone()),
          amount: item.amount,
        });
      }
    }

    Ok(rows)
  }
}
