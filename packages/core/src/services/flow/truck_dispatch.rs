use sea_orm::{
  entity::prelude::Decimal, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter,
  QueryOrder,
};
use uuid::Uuid;

use super::FlowService;
use crate::{
  api::ApiError,
  dtos::response::flow::TruckDispatchFlowRow,
  entities::{company, dispatch_document, dispatch_item, product},
  enums::{DispatchMethod, DocumentStatus, PipelineStatus},
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
    if let Some(ps) = pipeline_status {
      match ps {
        PipelineStatus::Pending => return Ok(vec![]),
        PipelineStatus::Draft => cond = cond.add(dispatch_document::Column::Status.eq(DocumentStatus::Draft)),
        PipelineStatus::Executed => cond = cond.add(dispatch_document::Column::Status.eq(DocumentStatus::Posted)),
      }
    }

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
    let items: Vec<dispatch_item::Model> = dispatch_item::Entity::find()
      .filter(
        Condition::all()
          .add(dispatch_item::Column::DispatchDocId.is_in(doc_ids))
          .add(dispatch_item::Column::DeletedAt.is_null()),
      )
      .all(db)
      .await?;

    let mut first_item: std::collections::HashMap<Uuid, &dispatch_item::Model> = Default::default();
    let mut sums: std::collections::HashMap<Uuid, Decimal> = Default::default();
    for item in &items {
      first_item.entry(item.dispatch_doc_id).or_insert(item);
      *sums.entry(item.dispatch_doc_id).or_insert(Decimal::ZERO) += item.dispatched_amount;
    }

    let cids: Vec<Uuid> = dispatches.iter().map(|d| d.contractor_id).collect();
    let companies: std::collections::HashMap<Uuid, String> = company::Entity::find()
      .filter(company::Column::Id.is_in(cids))
      .all(db)
      .await?
      .into_iter()
      .map(|c| (c.id, c.common_name))
      .collect();

    let pids: Vec<Uuid> = first_item.values().map(|i| i.product_id).collect();
    let products: std::collections::HashMap<Uuid, String> = if pids.is_empty() {
      Default::default()
    } else {
      product::Entity::find()
        .filter(product::Column::Id.is_in(pids))
        .all(db)
        .await?
        .into_iter()
        .map(|p| (p.id, p.common_name))
        .collect()
    };

    let mut rows = Vec::with_capacity(dispatches.len());
    for dd in &dispatches {
      let fi = first_item.get(&dd.id);
      rows.push(TruckDispatchFlowRow {
        dispatch_id: dd.id,
        document_number: dd.document_number.clone(),
        date: dd.date.to_string(),
        contractor_id: dd.contractor_id,
        contractor_name: companies.get(&dd.contractor_id).cloned().unwrap_or_default(),
        product_name: fi.and_then(|i| products.get(&i.product_id).cloned()),
        dispatched_quantity: sums.get(&dd.id).copied(),
        pipeline_status: PipelineStatus::from_doc_status(Some(&dd.status)),
      });
    }

    Ok(rows)
  }
}
