use sea_orm::{
  ColumnTrait,
  Condition,
  EntityLoaderTrait,
  EntityTrait,
  PaginatorTrait,
  QueryFilter,
  TransactionTrait,
};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos::{
    CreateOwnershipTransferItemRequest,
    CreateOwnershipTransferRequest,
    OwnershipTransferResponse,
  },
  entities::{ownership_transfer, ownership_transfer_item},
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

    response.status = crate::enums::DocumentStatus::Posted;

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
}
