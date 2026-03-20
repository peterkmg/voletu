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
    CreatePhysicalTransferItemRequest,
    CreatePhysicalTransferRequest,
    PhysicalTransferResponse,
  },
  entities::{physical_storage_transfer, physical_transfer_item},
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

    response.status = crate::enums::DocumentStatus::Posted;

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
}
