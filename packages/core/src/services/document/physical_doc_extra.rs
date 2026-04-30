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

use super::types::PhysicalTransferFlatRowRef;
use crate::{
  api::ApiError,
  dtos::{
    self,
    response::document::PhysicalTransferFlatRow,
    CreatePhysicalTransferRequest,
    PhysicalTransferResponse,
  },
  entities::{company, physical_storage_transfer, physical_transfer_item, product, storage},
  enums::DocumentStatus,
  services::{
    common::normalize_pagination,
    document::{
      specs::{PhysicalTransferFlatQuerySpec, PhysicalTransferQuerySpec},
      DocumentService,
    },
  },
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

    response.status = DocumentStatus::Executed;

    txn.commit().await?;

    Ok(response)
  }

  pub(crate) async fn physical_transfer_composite_create_no_tx(
    &self,
    conn: &sea_orm::DatabaseTransaction,
    req: &CreatePhysicalTransferRequest,
  ) -> Result<PhysicalTransferResponse, ApiError> {
    let saved = physical_storage_transfer::ActiveModelEx::from(req)
      .save(conn)
      .await?;

    let transfer_id = match saved.id {
      sea_orm::ActiveValue::Set(id) | sea_orm::ActiveValue::Unchanged(id) => id,
      sea_orm::ActiveValue::NotSet => {
        return Err(ApiError::Internal(anyhow::anyhow!(
          "physical transfer graph save returned no id"
        )));
      }
    };

    self
      .audit
      .backfill_document_routing::<physical_storage_transfer::Entity>(conn, transfer_id)
      .await?;

    PhysicalTransferResponse::try_from(
      physical_storage_transfer::Entity::load()
        .filter_by_id(transfer_id)
        .filter(physical_storage_transfer::Column::DeletedAt.is_null())
        .with(company::Entity)
        .with((physical_transfer_item::Entity, product::Entity))
        .with((physical_transfer_item::Entity, storage::Entity))
        .one(conn)
        .await?
        .ok_or_else(|| {
          ApiError::NotFound(format!("Physical transfer '{}' not found", transfer_id))
        })?,
    )
  }

  pub async fn physical_transfer_composite_update(
    &self,
    physical_transfer_id: Uuid,
    req: &dtos::UpdatePhysicalTransferCompositeRequest,
  ) -> Result<PhysicalTransferResponse, ApiError> {
    let txn = self.db.begin().await?;

    let res = self
      .physical_transfer_composite_update_no_tx(&txn, physical_transfer_id, req)
      .await?;

    txn.commit().await?;

    Ok(res)
  }

  pub(crate) async fn physical_transfer_composite_update_no_tx(
    &self,
    conn: &sea_orm::DatabaseTransaction,
    physical_transfer_id: Uuid,
    req: &dtos::UpdatePhysicalTransferCompositeRequest,
  ) -> Result<PhysicalTransferResponse, ApiError> {
    self
      .physical_transfer_update_no_tx(conn, physical_transfer_id, &req.physical_transfer)
      .await?;

    let mut kept_ids: HashSet<Uuid> = HashSet::new();
    for item in &req.items {
      if let Some(item_id) = item.id {
        if !kept_ids.insert(item_id) {
          return Err(ApiError::BadRequest(format!(
            "duplicate item id in request: {}",
            item_id
          )));
        }
      }
    }

    let items: Vec<physical_transfer_item::ActiveModelEx> = req
      .items
      .iter()
      .map(|item| physical_transfer_item::ActiveModelEx {
        id: match item.id {
          Some(id) => sea_orm::ActiveValue::Unchanged(id),
          None => sea_orm::ActiveValue::NotSet,
        },
        product_id: sea_orm::ActiveValue::Set(item.product_id),
        from_storage_id: sea_orm::ActiveValue::Set(item.from_storage_id),
        to_storage_id: sea_orm::ActiveValue::Set(item.to_storage_id),
        amount: sea_orm::ActiveValue::Set(item.amount),
        ..Default::default()
      })
      .collect();

    physical_storage_transfer::ActiveModelEx {
      id: sea_orm::ActiveValue::Unchanged(physical_transfer_id),
      items: sea_orm::HasManyModel::Replace(items),
      ..Default::default()
    }
    .save(conn)
    .await?;

    self
      .audit
      .backfill_document_routing::<physical_storage_transfer::Entity>(conn, physical_transfer_id)
      .await?;

    PhysicalTransferResponse::try_from(
      physical_storage_transfer::Entity::load()
        .filter_by_id(physical_transfer_id)
        .filter(physical_storage_transfer::Column::DeletedAt.is_null())
        .with(company::Entity)
        .with((physical_transfer_item::Entity, product::Entity))
        .with((physical_transfer_item::Entity, storage::Entity))
        .one(conn)
        .await?
        .ok_or_else(|| {
          ApiError::NotFound(format!(
            "Physical transfer '{}' not found",
            physical_transfer_id
          ))
        })?,
    )
  }

  pub(super) async fn physical_transfer_model(
    &self,
    id: Uuid,
  ) -> Result<physical_storage_transfer::ModelEx, ApiError> {
    physical_storage_transfer::Entity::load()
      .filter_by_id(id)
      .filter(physical_storage_transfer::Column::DeletedAt.is_null())
      .with(company::Entity)
      .with((physical_transfer_item::Entity, product::Entity))
      .with((physical_transfer_item::Entity, storage::Entity))
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Physical transfer '{}' not found", id)))
  }

  pub(super) async fn physical_transfer_query_models(
    &self,
    query: &PhysicalTransferQuerySpec,
  ) -> Result<Vec<physical_storage_transfer::ModelEx>, ApiError> {
    let (page, per_page) = normalize_pagination(query.page, query.per_page)?;

    let mut condition = Condition::all();
    condition = condition.add(physical_storage_transfer::Column::DeletedAt.is_null());

    if let Some(document_number) = query.document_number.as_deref() {
      condition =
        condition.add(physical_storage_transfer::Column::DocumentNumber.contains(document_number));
    }

    if let Some(status) = query.status {
      condition = condition.add(physical_storage_transfer::Column::Status.eq(status));
    }

    physical_storage_transfer::Entity::load()
      .filter(condition)
      .with(company::Entity)
      .with((physical_transfer_item::Entity, product::Entity))
      .with((physical_transfer_item::Entity, storage::Entity))
      .order_by_desc(physical_storage_transfer::Column::Date)
      .paginate(self.db.as_ref(), per_page)
      .fetch_page(page - 1)
      .await
      .map_err(Into::into)
  }

  pub async fn physical_transfer_composite_list(
    &self,
  ) -> Result<Vec<PhysicalTransferResponse>, ApiError> {
    let docs = self
      .physical_transfer_query_models(&PhysicalTransferQuerySpec::default())
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
    let doc = self.physical_transfer_model(id).await?;

    PhysicalTransferResponse::try_from(doc)
  }

  pub async fn physical_transfer_composite_query(
    &self,
    query: PhysicalTransferQuerySpec,
  ) -> Result<Vec<PhysicalTransferResponse>, ApiError> {
    self
      .physical_transfer_query_models(&query)
      .await?
      .into_iter()
      .map(PhysicalTransferResponse::try_from)
      .collect()
  }

  pub async fn physical_transfer_flat_query(
    &self,
    query: PhysicalTransferFlatQuerySpec,
  ) -> Result<Vec<PhysicalTransferFlatRow>, ApiError> {
    let (page, per_page) = normalize_pagination(query.page, query.per_page)?;

    let docs = self
      .physical_transfer_query_models(&PhysicalTransferQuerySpec {
        status: query.status,
        page: Some(page),
        per_page: Some(per_page),
        ..PhysicalTransferQuerySpec::default()
      })
      .await?;

    let to_storage_map = self
      .storage_name_map(
        docs
          .iter()
          .flat_map(|d| d.items.iter().map(|i| i.to_storage_id))
          .collect::<Vec<_>>(),
      )
      .await?;

    let mut rows = Vec::new();
    for doc in &docs {
      let contractor_id_name = doc
        .contractor
        .as_ref()
        .map(|contractor| contractor.common_name.as_str())
        .unwrap_or("\u{2014}");

      if doc.items.is_empty() {
        rows.push(PhysicalTransferFlatRow::from(PhysicalTransferFlatRowRef {
          document: doc,
          item: None,
          contractor_id_name,
          to_storage_id_name: "\u{2014}",
        }));
      }

      for item in &doc.items {
        let to_storage_id_name = to_storage_map
          .get(&item.to_storage_id)
          .map(String::as_str)
          .unwrap_or_default();

        rows.push(PhysicalTransferFlatRow::from(PhysicalTransferFlatRowRef {
          document: doc,
          item: Some(item),
          contractor_id_name,
          to_storage_id_name,
        }));
      }
    }

    Ok(rows)
  }
}
