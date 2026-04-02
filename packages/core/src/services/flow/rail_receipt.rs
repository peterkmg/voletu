use sea_orm::{
  entity::prelude::Decimal, ColumnTrait, Condition, EntityTrait, LoaderTrait, PaginatorTrait,
  QueryFilter, QueryOrder,
};
use uuid::Uuid;

use super::FlowService;
use crate::{
  api::ApiError,
  dtos::response::flow::RailReceiptFlowRow,
  entities::{acceptance_document, acceptance_item, company, product, rail_wagon_manifest, rail_waybill},
  enums::PipelineStatus,
  services::common::normalize_pagination,
};

impl FlowService {
  pub async fn rail_receipt_query(
    &self,
    pipeline_status: Option<PipelineStatus>,
    contractor_id: Option<Uuid>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<RailReceiptFlowRow>, ApiError> {
    let (page, per_page) = normalize_pagination(page, per_page)?;
    let db = self.db.as_ref();

    let mut cond = Condition::all().add(rail_waybill::Column::DeletedAt.is_null());
    if let Some(cid) = contractor_id {
      cond = cond.add(rail_waybill::Column::SenderId.eq(cid));
    }

    let waybills = rail_waybill::Entity::find()
      .filter(cond)
      .order_by_desc(rail_waybill::Column::Date)
      .paginate(db, per_page)
      .fetch_page(page - 1)
      .await?;

    if waybills.is_empty() {
      return Ok(vec![]);
    }

    let acceptances_per_wb = waybills
      .load_many(
        acceptance_document::Entity::find()
          .filter(acceptance_document::Column::DeletedAt.is_null()),
        db,
      )
      .await?;

    let manifests_per_wb = waybills
      .load_many(
        rail_wagon_manifest::Entity::find()
          .filter(rail_wagon_manifest::Column::DeletedAt.is_null()),
        db,
      )
      .await?;

    let sender_ids: Vec<Uuid> = waybills.iter().map(|w| w.sender_id).collect();
    let companies: std::collections::HashMap<Uuid, String> = company::Entity::find()
      .filter(company::Column::Id.is_in(sender_ids))
      .all(db)
      .await?
      .into_iter()
      .map(|c| (c.id, c.common_name))
      .collect();

    let product_ids: Vec<Uuid> = manifests_per_wb.iter()
      .filter_map(|ms| ms.first().map(|m| m.product_id))
      .collect();
    let products: std::collections::HashMap<Uuid, String> = if product_ids.is_empty() {
      Default::default()
    } else {
      product::Entity::find()
        .filter(product::Column::Id.is_in(product_ids))
        .all(db)
        .await?
        .into_iter()
        .map(|p| (p.id, p.common_name))
        .collect()
    };

    let acc_ids: Vec<Uuid> = acceptances_per_wb.iter()
      .filter_map(|accs| accs.first().map(|a| a.id))
      .collect();
    let acc_items: Vec<acceptance_item::Model> = if acc_ids.is_empty() {
      vec![]
    } else {
      acceptance_item::Entity::find()
        .filter(
          Condition::all()
            .add(acceptance_item::Column::AcceptanceDocId.is_in(acc_ids))
            .add(acceptance_item::Column::DeletedAt.is_null()),
        )
        .all(db)
        .await?
    };
    let mut acc_sums: std::collections::HashMap<Uuid, Decimal> = Default::default();
    for ai in &acc_items {
      *acc_sums.entry(ai.acceptance_doc_id).or_insert(Decimal::ZERO) += ai.accepted_amount;
    }

    let mut rows = Vec::with_capacity(waybills.len());
    for ((wb, accs), manifests) in waybills.iter().zip(&acceptances_per_wb).zip(&manifests_per_wb) {
      let acc = accs.first();
      let status = PipelineStatus::from_doc_status(acc.map(|a| &a.status));

      if pipeline_status.is_some() && pipeline_status != Some(status) {
        continue;
      }

      let manifest = manifests.first();
      rows.push(RailReceiptFlowRow {
        basis_id: wb.id,
        basis_document_number: wb.document_number.clone(),
        basis_date: wb.date.to_string(),
        contractor_id: wb.sender_id,
        contractor_name: companies.get(&wb.sender_id).cloned().unwrap_or_default(),
        product_name: manifest.and_then(|m| products.get(&m.product_id).cloned()),
        expected_quantity: manifest.map(|m| m.declared_mass),
        action_id: acc.map(|a| a.id),
        action_document_number: acc.map(|a| a.document_number.clone()),
        actual_quantity: acc.and_then(|a| acc_sums.get(&a.id).copied()),
        pipeline_status: status,
      });
    }

    Ok(rows)
  }
}
