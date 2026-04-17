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

use super::types::OwnershipTransferFlatRowRef;
use crate::{
  api::ApiError,
  dtos::{
    self,
    response::document::OwnershipTransferFlatRow,
    CreateOwnershipTransferRequest,
    OwnershipTransferResponse,
  },
  entities::{ownership_transfer, ownership_transfer_item, product, storage},
  services::document::{
    specs::{OwnershipTransferFlatQuerySpec, OwnershipTransferQuerySpec},
    DocumentService,
  },
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
    let saved = ownership_transfer::ActiveModelEx::from(req)
      .save(conn)
      .await?;
    let transfer_id = match saved.id {
      sea_orm::ActiveValue::Set(id) | sea_orm::ActiveValue::Unchanged(id) => id,
      sea_orm::ActiveValue::NotSet => {
        return Err(ApiError::Internal(anyhow::anyhow!(
          "ownership transfer graph save returned no id"
        )));
      }
    };

    self
      .audit
      .backfill_document_routing::<ownership_transfer::Entity>(conn, transfer_id)
      .await?;

    OwnershipTransferResponse::try_from(
      ownership_transfer::Entity::load()
        .filter_by_id(transfer_id)
        .filter(ownership_transfer::Column::DeletedAt.is_null())
        .with((ownership_transfer_item::Entity, product::Entity))
        .with((ownership_transfer_item::Entity, storage::Entity))
        .one(conn)
        .await?
        .ok_or_else(|| {
          ApiError::NotFound(format!("Ownership transfer '{}' not found", transfer_id))
        })?,
    )
  }

  /// Composite update: applies a header partial update plus a full diff on the items list.
  /// Items with `id: Some(uuid)` matching an existing row are updated.
  /// Items with `id: None` are inserted.
  /// Existing items not present in the request are hard-deleted.
  pub async fn ownership_transfer_composite_update(
    &self,
    ownership_transfer_id: Uuid,
    req: &dtos::UpdateOwnershipTransferCompositeRequest,
  ) -> Result<OwnershipTransferResponse, ApiError> {
    let txn = self.db.begin().await?;
    let res = self
      .ownership_transfer_composite_update_no_tx(&txn, ownership_transfer_id, req)
      .await?;
    txn.commit().await?;
    Ok(res)
  }

  pub(crate) async fn ownership_transfer_composite_update_no_tx(
    &self,
    conn: &sea_orm::DatabaseTransaction,
    ownership_transfer_id: Uuid,
    req: &dtos::UpdateOwnershipTransferCompositeRequest,
  ) -> Result<OwnershipTransferResponse, ApiError> {
    // 1. Header update via the macro-generated per-row updater.
    //    This enforces draft-only mutation, applies set_if_some semantics,
    //    and registers an audit log row.
    self
      .ownership_transfer_update_no_tx(conn, ownership_transfer_id, &req.ownership_transfer)
      .await?;

    // 2. Reject duplicate `Some(id)` entries in the payload before touching the
    //    database. The HashSet doubles as the dedup guard.
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

    // 3. Persist the items as a graph save. `HasManyModel::Replace(_)` deletes
    //    every existing related row that is not present in the new set.
    let items: Vec<ownership_transfer_item::ActiveModelEx> = req
      .items
      .iter()
      .map(|item| ownership_transfer_item::ActiveModelEx {
        id: match item.id {
          Some(id) => sea_orm::ActiveValue::Unchanged(id),
          None => sea_orm::ActiveValue::NotSet,
        },
        storage_id: sea_orm::ActiveValue::Set(item.storage_id),
        product_id: sea_orm::ActiveValue::Set(item.product_id),
        from_contractor_id: sea_orm::ActiveValue::Set(item.from_contractor_id),
        to_contractor_id: sea_orm::ActiveValue::Set(item.to_contractor_id),
        amount: sea_orm::ActiveValue::Set(item.amount),
        ..Default::default()
      })
      .collect();

    ownership_transfer::ActiveModelEx {
      id: sea_orm::ActiveValue::Unchanged(ownership_transfer_id),
      items: sea_orm::HasManyModel::Replace(items),
      ..Default::default()
    }
    .save(conn)
    .await?;

    // 4. Re-derive document routing tags via the existing utility.
    self
      .audit
      .backfill_document_routing::<ownership_transfer::Entity>(conn, ownership_transfer_id)
      .await?;

    // 5. Reload the full composite using the same eager-loading shape as on create.
    OwnershipTransferResponse::try_from(
      ownership_transfer::Entity::load()
        .filter_by_id(ownership_transfer_id)
        .filter(ownership_transfer::Column::DeletedAt.is_null())
        .with((ownership_transfer_item::Entity, product::Entity))
        .with((ownership_transfer_item::Entity, storage::Entity))
        .one(conn)
        .await?
        .ok_or_else(|| {
          ApiError::NotFound(format!(
            "Ownership transfer '{}' not found",
            ownership_transfer_id
          ))
        })?,
    )
  }

  pub(super) async fn ownership_transfer_model(
    &self,
    id: Uuid,
  ) -> Result<ownership_transfer::ModelEx, ApiError> {
    ownership_transfer::Entity::load()
      .filter_by_id(id)
      .filter(ownership_transfer::Column::DeletedAt.is_null())
      .with((ownership_transfer_item::Entity, product::Entity))
      .with((ownership_transfer_item::Entity, storage::Entity))
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Ownership transfer '{}' not found", id)))
  }

  pub(super) async fn ownership_transfer_query_models(
    &self,
    query: &OwnershipTransferQuerySpec,
  ) -> Result<Vec<ownership_transfer::ModelEx>, ApiError> {
    let (page, per_page) =
      crate::services::common::normalize_pagination(query.page, query.per_page)?;

    let mut condition = Condition::all();
    condition = condition.add(ownership_transfer::Column::DeletedAt.is_null());

    if let Some(status) = query.status {
      condition = condition.add(ownership_transfer::Column::Status.eq(status));
    }

    ownership_transfer::Entity::load()
      .filter(condition)
      .with((ownership_transfer_item::Entity, product::Entity))
      .with((ownership_transfer_item::Entity, storage::Entity))
      .order_by_desc(ownership_transfer::Column::Date)
      .paginate(self.db.as_ref(), per_page)
      .fetch_page(page - 1)
      .await
      .map_err(Into::into)
  }

  pub async fn ownership_transfer_composite_list(
    &self,
  ) -> Result<Vec<OwnershipTransferResponse>, ApiError> {
    let docs = self
      .ownership_transfer_query_models(&OwnershipTransferQuerySpec::default())
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
    let doc = self.ownership_transfer_model(id).await?;

    OwnershipTransferResponse::try_from(doc)
  }

  pub async fn ownership_transfer_composite_query(
    &self,
    query: OwnershipTransferQuerySpec,
  ) -> Result<Vec<OwnershipTransferResponse>, ApiError> {
    self
      .ownership_transfer_query_models(&query)
      .await?
      .into_iter()
      .map(OwnershipTransferResponse::try_from)
      .collect()
  }

  /// Returns one row per ownership transfer item with document fields repeated.
  /// Used by the grouped-row list table on the frontend.
  pub async fn ownership_transfer_flat_query(
    &self,
    query: OwnershipTransferFlatQuerySpec,
  ) -> Result<Vec<OwnershipTransferFlatRow>, ApiError> {
    let (page, per_page) =
      crate::services::common::normalize_pagination(query.page, query.per_page)?;
    let docs = self
      .ownership_transfer_query_models(&OwnershipTransferQuerySpec {
        status: query.status,
        page: Some(page),
        per_page: Some(per_page),
      })
      .await?;

    let contractor_map = self
      .company_name_map(
        docs
          .iter()
          .flat_map(|doc| {
            doc
              .items
              .iter()
              .flat_map(|item| [item.from_contractor_id, item.to_contractor_id])
          })
          .collect::<Vec<_>>(),
      )
      .await?;

    let mut rows = Vec::new();
    for doc in &docs {
      if doc.items.is_empty() {
        rows.push(OwnershipTransferFlatRow::from(
          OwnershipTransferFlatRowRef {
            document: doc,
            item: None,
            from_contractor_id_name: "\u{2014}",
            to_contractor_id_name: "\u{2014}",
          },
        ));
      }
      for item in &doc.items {
        let from_contractor_id_name = contractor_map
          .get(&item.from_contractor_id)
          .map(String::as_str)
          .unwrap_or("\u{2014}");
        let to_contractor_id_name = contractor_map
          .get(&item.to_contractor_id)
          .map(String::as_str)
          .unwrap_or("\u{2014}");

        rows.push(OwnershipTransferFlatRow::from(
          OwnershipTransferFlatRowRef {
            document: doc,
            item: Some(item),
            from_contractor_id_name,
            to_contractor_id_name,
          },
        ));
      }
    }

    Ok(rows)
  }
}
