use sea_orm::{
  entity::prelude::*,
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
  dtos::{self, response::pipeline::TruckReceiptPipelineResponse},
  entities::{
    acceptance_document,
    acceptance_item,
    company,
    product,
    truck_waybill,
    truck_waybill_item,
  },
  enums::PipelineStatus,
  services::DocumentService,
};

impl DocumentService {
  pub async fn truck_waybill_query(
    &self,
    document_number: Option<&str>,
    sender_id: Option<Uuid>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<dtos::TruckWaybillResponse>, ApiError> {
    let (page, per_page) = crate::services::common::normalize_pagination(page, per_page)?;

    let mut condition = Condition::all();
    condition = condition.add(truck_waybill::Column::DeletedAt.is_null());

    if let Some(document_number) = document_number {
      condition = condition.add(truck_waybill::Column::DocumentNumber.contains(document_number));
    }

    if let Some(sender_id) = sender_id {
      condition = condition.add(truck_waybill::Column::SenderId.eq(sender_id));
    }

    let docs = truck_waybill::Entity::find()
      .filter(condition)
      .paginate(self.db.as_ref(), per_page)
      .fetch_page(page - 1)
      .await?;

    Ok(
      docs
        .into_iter()
        .map(dtos::TruckWaybillResponse::from)
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
    let waybill = self
      .truck_waybill_create_no_tx(conn, &dtos::CreateTruckWaybillRequest::from_composite(req))
      .await?;

    let mut items: Option<Vec<dtos::TruckWaybillItemResponse>> = None;
    let mut weight_docs: Option<Vec<dtos::TruckWeightDocResponse>> = None;

    if let Some(item_reqs) = &req.items {
      for item_req in item_reqs {
        let item = self
          .truck_waybill_item_create_no_tx(
            conn,
            &dtos::CreateTruckWaybillItemRequest::from_composite(waybill.id, item_req),
          )
          .await?;

        items.get_or_insert_with(Vec::new).push(item);
      }
    }

    if let Some(weight_doc_reqs) = &req.weight_docs {
      for weight_doc_req in weight_doc_reqs {
        let weight_doc = self
          .truck_weight_doc_create_no_tx(
            conn,
            &dtos::CreateTruckWeightDocRequest::from_composite(waybill.id, weight_doc_req),
          )
          .await?;

        weight_docs.get_or_insert_with(Vec::new).push(weight_doc);
      }
    }

    Ok(dtos::TruckWaybillCompositeResponse {
      waybill,
      items,
      weight_docs,
    })
  }

  pub async fn truck_receipt_pipeline_query(
    &self,
    pipeline_status: Option<PipelineStatus>,
    contractor_id: Option<Uuid>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<TruckReceiptPipelineResponse>, ApiError> {
    let (page, per_page) = crate::services::common::normalize_pagination(page, per_page)?;
    let db = self.db.as_ref();

    let mut cond = Condition::all().add(truck_waybill::Column::DeletedAt.is_null());
    if let Some(cid) = contractor_id {
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

      if pipeline_status.is_some() && pipeline_status != Some(status) {
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
