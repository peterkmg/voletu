use sea_orm::{
  entity::prelude::Decimal, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter,
  QueryOrder,
};
use uuid::Uuid;

use super::FlowService;
use crate::{
  api::ApiError,
  dtos::response::flow::TruckDispatchFlowRow,
  entities::{dispatch_document, dispatch_item},
  enums::{DispatchMethod, PipelineStatus},
  services::common::normalize_pagination,
};

impl FlowService {
  pub async fn truck_dispatch_query(
    &self,
    pipeline_status: Option<PipelineStatus>,
    contractor_id: Option<Uuid>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<TruckDispatchFlowRow>, ApiError> {
    let (page, per_page) = normalize_pagination(page, per_page)?;
    let db = self.db.as_ref();

    let mut cond = Condition::all()
      .add(dispatch_document::Column::DeletedAt.is_null())
      .add(dispatch_document::Column::DispatchMethod.eq(DispatchMethod::Truck));
    if let Some(cid) = contractor_id {
      cond = cond.add(dispatch_document::Column::ContractorId.eq(cid));
    }
    let Some(cond) = Self::add_status_filter(cond, pipeline_status, dispatch_document::Column::Status) else {
      return Ok(vec![]);
    };

    let dispatches = dispatch_document::Entity::find()
      .filter(cond)
      .order_by_desc(dispatch_document::Column::Date)
      .paginate(db, per_page)
      .fetch_page(page - 1)
      .await?;

    if dispatches.is_empty() {
      return Ok(vec![]);
    }

    let doc_ids: Vec<Uuid> = dispatches.iter().map(|d| d.id).collect();

    let items = dispatch_item::Entity::find()
      .filter(
        Condition::all()
          .add(dispatch_item::Column::DispatchDocId.is_in(doc_ids))
          .add(dispatch_item::Column::DeletedAt.is_null()),
      )
      .all(db)
      .await?;
    let first_item = Self::first_per_parent(&items, |i| i.dispatch_doc_id);

    let mut dispatched_sums: std::collections::HashMap<Uuid, Decimal> = std::collections::HashMap::new();
    for item in &items {
      *dispatched_sums.entry(item.dispatch_doc_id).or_insert(Decimal::ZERO) += item.dispatched_amount;
    }

    let company_map =
      self.resolve_companies(&dispatches.iter().map(|d| d.contractor_id).collect::<Vec<_>>()).await?;
    let product_map = self
      .resolve_products(&first_item.values().map(|i| i.product_id).collect::<Vec<_>>())
      .await?;

    let mut rows = Vec::with_capacity(dispatches.len());
    for dd in &dispatches {
      let fi = first_item.get(&dd.id);
      rows.push(TruckDispatchFlowRow {
        dispatch_id: dd.id,
        document_number: dd.document_number.clone(),
        date: dd.date.to_string(),
        contractor_id: dd.contractor_id,
        contractor_name: Self::company_name(&company_map, dd.contractor_id),
        product_name: fi.and_then(|i| product_map.get(&i.product_id).cloned()),
        dispatched_quantity: dispatched_sums.get(&dd.id).copied(),
        pipeline_status: PipelineStatus::from_doc_status(Some(&dd.status)),
      });
    }

    Ok(rows)
  }
}
