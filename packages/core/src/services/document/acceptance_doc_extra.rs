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
  dtos::{self, response::pipeline::AcceptanceFlatRow},
  endpoints::query::NullableFilter,
  entities::{acceptance_document, acceptance_item, company, product, storage},
  enums,
  services::document::DocumentService,
};

impl DocumentService {
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
    let document = self
      .acceptance_document_create_no_tx(conn, &req.acceptance)
      .await?;

    let mut items = Vec::new();

    for item_req in &req.items {
      items.push(
        self
          .acceptance_item_create_no_tx(
            conn,
            &dtos::CreateAcceptanceItemRequest::from_composite(document.id, item_req),
          )
          .await?,
      );
    }

    Ok(dtos::AcceptanceCompositeResponse { document, items })
  }

  #[allow(clippy::too_many_arguments)]
  pub async fn acceptance_document_query(
    &self,
    document_number: Option<&str>,
    status: Option<enums::DocumentStatus>,
    truck_waybill_id: Option<NullableFilter>,
    rail_waybill_id: Option<NullableFilter>,
    transit_dispatch_id: Option<NullableFilter>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<dtos::AcceptanceResponse>, ApiError> {
    let (page, per_page) = crate::services::common::normalize_pagination(page, per_page)?;

    let mut condition = Condition::all();
    condition = condition.add(acceptance_document::Column::DeletedAt.is_null());

    if let Some(document_number) = document_number {
      condition =
        condition.add(acceptance_document::Column::DocumentNumber.contains(document_number));
    }

    if let Some(status) = status {
      condition = condition.add(acceptance_document::Column::Status.eq(status));
    }

    if let Some(filter) = truck_waybill_id {
      match filter {
        NullableFilter::IsNull => {
          condition = condition.add(acceptance_document::Column::TruckWaybillId.is_null());
        }
        NullableFilter::IsNotNull => {
          condition = condition.add(acceptance_document::Column::TruckWaybillId.is_not_null());
        }
      }
    }

    if let Some(filter) = rail_waybill_id {
      match filter {
        NullableFilter::IsNull => {
          condition = condition.add(acceptance_document::Column::RailWaybillId.is_null());
        }
        NullableFilter::IsNotNull => {
          condition = condition.add(acceptance_document::Column::RailWaybillId.is_not_null());
        }
      }
    }

    if let Some(filter) = transit_dispatch_id {
      match filter {
        NullableFilter::IsNull => {
          condition = condition.add(acceptance_document::Column::TransitDispatchId.is_null());
        }
        NullableFilter::IsNotNull => {
          condition = condition.add(acceptance_document::Column::TransitDispatchId.is_not_null());
        }
      }
    }

    let docs = acceptance_document::Entity::find()
      .filter(condition)
      .paginate(self.db.as_ref(), per_page)
      .fetch_page(page - 1)
      .await?;

    Ok(
      docs
        .into_iter()
        .map(dtos::AcceptanceResponse::from)
        .collect(),
    )
  }

  pub async fn acceptance_composite_get(
    &self,
    id: Uuid,
  ) -> Result<dtos::AcceptanceCompositeResponse, ApiError> {
    let doc = acceptance_document::Entity::load()
      .filter_by_id(id)
      .filter(acceptance_document::Column::DeletedAt.is_null())
      .with(acceptance_item::Entity)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Acceptance document '{}' not found", id)))?;

    dtos::AcceptanceCompositeResponse::try_from(doc)
  }

  /// Returns one row per acceptance item with document fields repeated.
  /// Used by the grouped-row list table on the frontend.
  pub async fn acceptance_flat_query(
    &self,
    status: Option<enums::DocumentStatus>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<AcceptanceFlatRow>, ApiError> {
    let (page, per_page) = crate::services::common::normalize_pagination(page, per_page)?;
    let db = self.db.as_ref();

    let mut cond = Condition::all().add(acceptance_document::Column::DeletedAt.is_null());
    if let Some(s) = status {
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
      let contractor_name = doc
        .contractor
        .as_ref()
        .map(|c| c.common_name.clone())
        .unwrap_or("\u{2014}".to_string());

      if doc.items.is_empty() {
        rows.push(AcceptanceFlatRow {
          id: doc.id,
          document_id: doc.id,
          document_number: doc.document_number.clone(),
          date_accepted: doc.date_accepted.to_string(),
          status: doc.status,
          source_entity: doc.source_entity.clone(),
          item_id: doc.id,
          product_id_name: "\u{2014}".to_string(),
          storage_id_name: "\u{2014}".to_string(),
          contractor_id_name: contractor_name.clone(),
          accepted_amount: Default::default(),
        });
      }
      for item in &doc.items {
        rows.push(AcceptanceFlatRow {
          id: doc.id,
          document_id: doc.id,
          document_number: doc.document_number.clone(),
          date_accepted: doc.date_accepted.to_string(),
          status: doc.status,
          source_entity: doc.source_entity.clone(),
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
          contractor_id_name: contractor_name.clone(),
          accepted_amount: item.accepted_amount,
        });
      }
    }

    Ok(rows)
  }
}
