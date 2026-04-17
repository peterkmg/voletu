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

use super::types::AcceptanceFlatRowRef;
use crate::{
  api::ApiError,
  dtos::{self, request::query::NullableFilter, response::document::AcceptanceFlatRow},
  entities::{
    acceptance_document,
    acceptance_item,
    company,
    dispatch_document,
    product,
    rail_waybill,
    storage,
    truck_waybill,
  },
  services::document::{
    specs::{AcceptanceDocumentQuerySpec, AcceptanceFlatQuerySpec},
    DocumentService,
  },
};

impl DocumentService {
  pub(super) async fn acceptance_document_model(
    &self,
    id: Uuid,
  ) -> Result<acceptance_document::ModelEx, ApiError> {
    acceptance_document::Entity::load()
      .filter_by_id(id)
      .filter(acceptance_document::Column::DeletedAt.is_null())
      .with(company::Entity)
      .with(truck_waybill::Entity)
      .with(rail_waybill::Entity)
      .with(dispatch_document::Entity)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Acceptance document '{}' not found", id)))
  }

  pub(super) async fn acceptance_document_query_models(
    &self,
    query: &AcceptanceDocumentQuerySpec,
  ) -> Result<Vec<acceptance_document::ModelEx>, ApiError> {
    let (page, per_page) =
      crate::services::common::normalize_pagination(query.page, query.per_page)?;

    let mut condition = Condition::all();
    condition = condition.add(acceptance_document::Column::DeletedAt.is_null());

    if let Some(document_number) = query.document_number.as_deref() {
      condition =
        condition.add(acceptance_document::Column::DocumentNumber.contains(document_number));
    }

    if let Some(status) = query.status {
      condition = condition.add(acceptance_document::Column::Status.eq(status));
    }

    if let Some(filter) = query.truck_waybill_id {
      match filter {
        NullableFilter::IsNull => {
          condition = condition.add(acceptance_document::Column::TruckWaybillId.is_null());
        }
        NullableFilter::IsNotNull => {
          condition = condition.add(acceptance_document::Column::TruckWaybillId.is_not_null());
        }
      }
    }

    if let Some(filter) = query.rail_waybill_id {
      match filter {
        NullableFilter::IsNull => {
          condition = condition.add(acceptance_document::Column::RailWaybillId.is_null());
        }
        NullableFilter::IsNotNull => {
          condition = condition.add(acceptance_document::Column::RailWaybillId.is_not_null());
        }
      }
    }

    if let Some(filter) = query.transit_dispatch_id {
      match filter {
        NullableFilter::IsNull => {
          condition = condition.add(acceptance_document::Column::TransitDispatchId.is_null());
        }
        NullableFilter::IsNotNull => {
          condition = condition.add(acceptance_document::Column::TransitDispatchId.is_not_null());
        }
      }
    }

    Ok(
      acceptance_document::Entity::load()
        .filter(condition)
        .with(company::Entity)
        .with(truck_waybill::Entity)
        .with(rail_waybill::Entity)
        .with(dispatch_document::Entity)
        .order_by_desc(acceptance_document::Column::DateAccepted)
        .paginate(self.db.as_ref(), per_page)
        .fetch_page(page - 1)
        .await?,
    )
  }

  pub(super) async fn acceptance_composite_model(
    &self,
    id: Uuid,
  ) -> Result<acceptance_document::ModelEx, ApiError> {
    acceptance_document::Entity::load()
      .filter_by_id(id)
      .filter(acceptance_document::Column::DeletedAt.is_null())
      .with(company::Entity)
      .with(truck_waybill::Entity)
      .with(rail_waybill::Entity)
      .with(dispatch_document::Entity)
      .with((acceptance_item::Entity, product::Entity))
      .with((acceptance_item::Entity, storage::Entity))
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Acceptance document '{}' not found", id)))
  }

  pub async fn acceptance_composite_create(
    &self,
    req: &dtos::CreateAcceptanceCompositeRequest,
  ) -> Result<dtos::AcceptanceCompositeResponse, ApiError> {
    let txn = self.db.begin().await?;

    let res = self.acceptance_composite_create_no_tx(&txn, req).await?;

    txn.commit().await?;

    Ok(res)
  }

  pub async fn acceptance_composite_create_and_execute(
    &self,
    req: &dtos::CreateAcceptanceCompositeRequest,
    actor_id: Uuid,
  ) -> Result<dtos::AcceptanceCompositeResponse, ApiError> {
    let txn = self.db.begin().await?;

    let mut response = self.acceptance_composite_create_no_tx(&txn, req).await?;

    self
      .acceptance_document_execute_no_tx(&txn, response.document.id, actor_id)
      .await?;

    response.document.status = crate::enums::DocumentStatus::Executed;
    txn.commit().await?;

    Ok(response)
  }

  pub(crate) async fn acceptance_composite_create_no_tx(
    &self,
    conn: &sea_orm::DatabaseTransaction,
    req: &dtos::CreateAcceptanceCompositeRequest,
  ) -> Result<dtos::AcceptanceCompositeResponse, ApiError> {
    let saved = acceptance_document::ActiveModelEx::from(req)
      .save(conn)
      .await?;
    let document_id = match saved.id {
      sea_orm::ActiveValue::Set(id) | sea_orm::ActiveValue::Unchanged(id) => id,
      sea_orm::ActiveValue::NotSet => {
        return Err(ApiError::Internal(anyhow::anyhow!(
          "acceptance graph save returned no id"
        )));
      }
    };

    self
      .audit
      .backfill_document_routing::<acceptance_document::Entity>(conn, document_id)
      .await?;

    dtos::AcceptanceCompositeResponse::try_from(
      acceptance_document::Entity::load()
        .filter_by_id(document_id)
        .filter(acceptance_document::Column::DeletedAt.is_null())
        .with(company::Entity)
        .with(truck_waybill::Entity)
        .with(rail_waybill::Entity)
        .with(dispatch_document::Entity)
        .with((acceptance_item::Entity, product::Entity))
        .with((acceptance_item::Entity, storage::Entity))
        .one(conn)
        .await?
        .ok_or_else(|| {
          ApiError::NotFound(format!("Acceptance document '{}' not found", document_id))
        })?,
    )
  }

  /// Composite update: applies a header partial update plus a full diff on the items list.
  /// Items with `id: Some(uuid)` matching an existing row are updated.
  /// Items with `id: None` are inserted.
  /// Existing items not present in the request are hard-deleted.
  pub async fn acceptance_composite_update(
    &self,
    acceptance_id: Uuid,
    req: &dtos::UpdateAcceptanceCompositeRequest,
  ) -> Result<dtos::AcceptanceCompositeResponse, ApiError> {
    let txn = self.db.begin().await?;
    let res = self
      .acceptance_composite_update_no_tx(&txn, acceptance_id, req)
      .await?;
    txn.commit().await?;
    Ok(res)
  }

  pub(crate) async fn acceptance_composite_update_no_tx(
    &self,
    conn: &sea_orm::DatabaseTransaction,
    acceptance_id: Uuid,
    req: &dtos::UpdateAcceptanceCompositeRequest,
  ) -> Result<dtos::AcceptanceCompositeResponse, ApiError> {
    // 1. Header update via the macro-generated per-row updater.
    //    This enforces draft-only mutation, applies set_if_some semantics,
    //    and registers an audit log row.
    self
      .acceptance_document_update_no_tx(conn, acceptance_id, &req.acceptance)
      .await?;

    // 2. Reject duplicate `Some(id)` entries in the payload before touching the
    //    database. The HashSet doubles as the dedup guard: if the same id
    //    appears twice we bail out before any items are persisted, so the
    //    transaction stays clean.
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

    // 3. Persist the items as a graph save on the parent `ActiveModelEx`. The
    //    parent itself carries only `id: Unchanged(_)` and no scalar mutations
    //    (the header was already updated in step 1), so the macro-generated
    //    `save` skips the parent UPDATE because `is_changed()` is false and
    //    proceeds straight to the children.
    //
    //    Each request item is mapped to an `acceptance_item::ActiveModelEx`:
    //      * `id: Some(uuid)` -> primary key as `Unchanged(uuid)` -> UPDATE,
    //      * `id: None`       -> primary key left `NotSet`         -> INSERT.
    //    `acceptance_doc_id` is set automatically by SeaORM via
    //    `set_parent_key` during the save action.
    //
    //    Wrapping the items in `HasManyModel::Replace(_)` (instead of the
    //    `Vec::into()` shorthand which produces `Append`) tells SeaORM to
    //    delete every existing related row that is not present in the new
    //    set — this is the diff semantic we need.
    let items: Vec<acceptance_item::ActiveModelEx> = req
      .items
      .iter()
      .map(|item| acceptance_item::ActiveModelEx {
        id: match item.id {
          Some(id) => sea_orm::ActiveValue::Unchanged(id),
          None => sea_orm::ActiveValue::NotSet,
        },
        product_id: sea_orm::ActiveValue::Set(item.product_id),
        storage_id: sea_orm::ActiveValue::Set(item.storage_id),
        accepted_amount: sea_orm::ActiveValue::Set(item.accepted_amount),
        ..Default::default()
      })
      .collect();

    acceptance_document::ActiveModelEx {
      id: sea_orm::ActiveValue::Unchanged(acceptance_id),
      items: sea_orm::HasManyModel::Replace(items),
      ..Default::default()
    }
    .save(conn)
    .await?;

    // 4. Re-derive document routing tags via the existing utility.
    self
      .audit
      .backfill_document_routing::<acceptance_document::Entity>(conn, acceptance_id)
      .await?;

    // 5. Reload the full composite using the same eager-loading shape as on create.
    dtos::AcceptanceCompositeResponse::try_from(
      acceptance_document::Entity::load()
        .filter_by_id(acceptance_id)
        .filter(acceptance_document::Column::DeletedAt.is_null())
        .with(company::Entity)
        .with(truck_waybill::Entity)
        .with(rail_waybill::Entity)
        .with(dispatch_document::Entity)
        .with((acceptance_item::Entity, product::Entity))
        .with((acceptance_item::Entity, storage::Entity))
        .one(conn)
        .await?
        .ok_or_else(|| {
          ApiError::NotFound(format!("Acceptance document '{}' not found", acceptance_id))
        })?,
    )
  }

  pub async fn acceptance_document_query(
    &self,
    query: AcceptanceDocumentQuerySpec,
  ) -> Result<Vec<dtos::AcceptanceResponse>, ApiError> {
    Ok(
      self
        .acceptance_document_query_models(&query)
        .await?
        .into_iter()
        .map(|doc| dtos::AcceptanceResponse::from(acceptance_document::Model::from(doc)))
        .collect(),
    )
  }

  pub async fn acceptance_composite_get(
    &self,
    id: Uuid,
  ) -> Result<dtos::AcceptanceCompositeResponse, ApiError> {
    let doc = self.acceptance_composite_model(id).await?;

    dtos::AcceptanceCompositeResponse::try_from(doc)
  }

  /// Returns one row per acceptance item with document fields repeated.
  /// Used by the grouped-row list table on the frontend.
  pub async fn acceptance_flat_query(
    &self,
    query: AcceptanceFlatQuerySpec,
  ) -> Result<Vec<AcceptanceFlatRow>, ApiError> {
    let (page, per_page) =
      crate::services::common::normalize_pagination(query.page, query.per_page)?;
    let db = self.db.as_ref();

    let mut cond = Condition::all().add(acceptance_document::Column::DeletedAt.is_null());
    if let Some(s) = query.status {
      cond = cond.add(acceptance_document::Column::Status.eq(s));
    }

    let docs: Vec<acceptance_document::ModelEx> = acceptance_document::Entity::load()
      .filter(cond)
      .with(company::Entity) // doc-level contractor
      .with((acceptance_item::Entity, product::Entity))
      .with((acceptance_item::Entity, storage::Entity))
      .order_by_desc(acceptance_document::Column::DateAccepted)
      .paginate(db, per_page)
      .fetch_page(page - 1)
      .await?;

    let mut rows = Vec::new();
    for doc in &docs {
      if doc.items.is_empty() {
        rows.push(AcceptanceFlatRow::from(AcceptanceFlatRowRef {
          document: doc,
          item: None,
        }));
      }
      for item in &doc.items {
        rows.push(AcceptanceFlatRow::from(AcceptanceFlatRowRef {
          document: doc,
          item: Some(item),
        }));
      }
    }

    Ok(rows)
  }
}
