use sea_orm::{
  entity::prelude::*,
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
  dtos::{self, response::flow::TruckReceiptPipelineResponse},
  entities::{
    acceptance_document,
    acceptance_item,
    company,
    product,
    truck_waybill,
    truck_waybill_item,
    truck_weight_doc,
  },
  enums::PipelineStatus,
  services::{
    document::specs::{TruckReceiptPipelineQuerySpec, TruckWaybillQuerySpec},
    DocumentService,
  },
};

impl DocumentService {
  pub(super) async fn truck_waybill_composite_model(
    &self,
    id: Uuid,
  ) -> Result<truck_waybill::ModelEx, ApiError> {
    truck_waybill::Entity::load()
      .filter_by_id(id)
      .filter(truck_waybill::Column::DeletedAt.is_null())
      .with(company::Entity)
      .with((truck_waybill_item::Entity, product::Entity))
      .with(truck_weight_doc::Entity)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Truck waybill '{}' not found", id)))
  }

  pub(super) async fn truck_waybill_query_models(
    &self,
    query: &TruckWaybillQuerySpec,
  ) -> Result<Vec<truck_waybill::ModelEx>, ApiError> {
    let (page, per_page) =
      crate::services::common::normalize_pagination(query.page, query.per_page)?;

    let mut condition = Condition::all();
    condition = condition.add(truck_waybill::Column::DeletedAt.is_null());

    if let Some(document_number) = query.document_number.as_deref() {
      condition = condition.add(truck_waybill::Column::DocumentNumber.contains(document_number));
    }

    if let Some(sender_id) = query.sender_id {
      condition = condition.add(truck_waybill::Column::SenderId.eq(sender_id));
    }

    Ok(
      truck_waybill::Entity::load()
        .filter(condition)
        .with(company::Entity)
        .paginate(self.db.as_ref(), per_page)
        .fetch_page(page - 1)
        .await?,
    )
  }

  pub async fn truck_waybill_query(
    &self,
    query: TruckWaybillQuerySpec,
  ) -> Result<Vec<dtos::TruckWaybillResponse>, ApiError> {
    Ok(
      self
        .truck_waybill_query_models(&query)
        .await?
        .into_iter()
        .map(|doc| dtos::TruckWaybillResponse::from(truck_waybill::Model::from(doc)))
        .collect(),
    )
  }

  pub async fn truck_waybill_composite_create(
    &self,
    req: &dtos::TruckWaybillCompositeRequest,
  ) -> Result<dtos::TruckWaybillCompositeResponse, ApiError> {
    let txn = self.db.begin().await?;
    let response = self.truck_waybill_composite_create_no_tx(&txn, req).await?;
    txn.commit().await?;
    Ok(response)
  }

  pub(crate) async fn truck_waybill_composite_create_no_tx(
    &self,
    conn: &sea_orm::DatabaseTransaction,
    req: &dtos::TruckWaybillCompositeRequest,
  ) -> Result<dtos::TruckWaybillCompositeResponse, ApiError> {
    let saved = truck_waybill::ActiveModelEx::from(req).save(conn).await?;
    let waybill_id = match saved.id {
      sea_orm::ActiveValue::Set(id) | sea_orm::ActiveValue::Unchanged(id) => id,
      sea_orm::ActiveValue::NotSet => {
        return Err(ApiError::Internal(anyhow::anyhow!(
          "truck waybill graph save returned no id"
        )));
      }
    };
    let doc = truck_waybill::Entity::load()
      .filter_by_id(waybill_id)
      .filter(truck_waybill::Column::DeletedAt.is_null())
      .with(company::Entity)
      .with((truck_waybill_item::Entity, product::Entity))
      .with(truck_weight_doc::Entity)
      .one(conn)
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Truck waybill '{}' not found", waybill_id)))?;

    let items: Vec<dtos::TruckWaybillItemResponse> = doc
      .items
      .iter()
      .map(|item| {
        dtos::TruckWaybillItemResponse::from(truck_waybill_item::Model::from(item.clone()))
      })
      .collect();
    let weight_docs: Vec<dtos::TruckWeightDocResponse> = doc
      .weight_docs
      .iter()
      .cloned()
      .map(dtos::TruckWeightDocResponse::from)
      .collect();

    let waybill = dtos::TruckWaybillResponse::from(truck_waybill::Model::from(doc));

    Ok(dtos::TruckWaybillCompositeResponse {
      waybill,
      items: if items.is_empty() { None } else { Some(items) },
      weight_docs: if weight_docs.is_empty() {
        None
      } else {
        Some(weight_docs)
      },
    })
  }

  pub async fn truck_waybill_composite_get(
    &self,
    id: Uuid,
  ) -> Result<dtos::TruckWaybillCompositeResponse, ApiError> {
    let doc = self.truck_waybill_composite_model(id).await?;

    let items: Vec<dtos::TruckWaybillItemResponse> = doc
      .items
      .iter()
      .map(|item| {
        dtos::TruckWaybillItemResponse::from(truck_waybill_item::Model::from(item.clone()))
      })
      .collect();
    let weight_docs: Vec<dtos::TruckWeightDocResponse> = doc
      .weight_docs
      .iter()
      .cloned()
      .map(dtos::TruckWeightDocResponse::from)
      .collect();

    let waybill = dtos::TruckWaybillResponse::from(truck_waybill::Model::from(doc));

    Ok(dtos::TruckWaybillCompositeResponse {
      waybill,
      items: if items.is_empty() { None } else { Some(items) },
      weight_docs: if weight_docs.is_empty() {
        None
      } else {
        Some(weight_docs)
      },
    })
  }

  pub async fn truck_receipt_pipeline_query(
    &self,
    query: TruckReceiptPipelineQuerySpec,
  ) -> Result<Vec<TruckReceiptPipelineResponse>, ApiError> {
    let (page, per_page) =
      crate::services::common::normalize_pagination(query.page, query.per_page)?;
    let db = self.db.as_ref();

    let mut cond = Condition::all().add(truck_waybill::Column::DeletedAt.is_null());
    if let Some(cid) = query.contractor_id {
      cond = cond.add(truck_waybill::Column::SenderId.eq(cid));
    }

    let waybills: Vec<truck_waybill::ModelEx> = truck_waybill::Entity::load()
      .filter(cond)
      .with(company::Entity)
      .with((truck_waybill_item::Entity, product::Entity))
      .with((acceptance_document::Entity, acceptance_item::Entity))
      .order_by_desc(truck_waybill::Column::Date)
      .paginate(db, per_page)
      .fetch_page(page - 1)
      .await?;

    let mut rows = Vec::with_capacity(waybills.len());
    for wb in &waybills {
      let acc = wb.acceptances.get(0);
      let status = PipelineStatus::from_doc_status(acc.map(|a| &a.status));

      if query.pipeline_status.is_some() && query.pipeline_status != Some(status) {
        continue;
      }

      let first_item = wb.items.get(0);
      let actual_quantity: Decimal = acc
        .map(|a| a.items.iter().map(|i| i.accepted_amount).sum())
        .unwrap_or(Decimal::ZERO);

      rows.push(TruckReceiptPipelineResponse {
        id: wb.id,
        basis_document_number: wb.document_number.clone(),
        basis_date: wb.date.to_string(),
        contractor_id: wb.sender_id,
        contractor_name: wb
          .sender
          .as_ref()
          .map(|s| s.common_name.clone())
          .unwrap_or_default(),
        product_name: first_item.and_then(|i| i.product.as_ref().map(|p| p.common_name.clone())),
        expected_quantity: first_item.map(|i| i.declared_amount),
        action_id: acc.map(|a| a.id),
        action_document_number: acc.map(|a| a.document_number.clone()),
        actual_quantity: if actual_quantity > Decimal::ZERO {
          Some(actual_quantity)
        } else {
          None
        },
        pipeline_status: status,
      });
    }

    Ok(rows)
  }
}
