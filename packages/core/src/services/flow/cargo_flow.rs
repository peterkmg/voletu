use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use super::FlowService;
use crate::{
  api::ApiError,
  dtos::response::flow::CargoFlowRow,
  entities::{
    acceptance_document, acceptance_item, blending_document, dispatch_document, dispatch_item,
    inventory_adjustment, inventory_reconciliation, ownership_transfer, ownership_transfer_item,
    physical_storage_transfer, physical_transfer_item, rail_waybill, truck_waybill,
  },
  enums::{FlowOperation, FlowType, PipelineStatus},
  services::common::normalize_pagination,
};

impl FlowService {
  pub async fn cargo_flow_query(
    &self,
    flow_type: Option<FlowType>,
    operation: Option<FlowOperation>,
    status: Option<PipelineStatus>,
    contractor_id: Option<Uuid>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<CargoFlowRow>, ApiError> {
    let (page, per_page) = normalize_pagination(page, per_page)?;
    let mut rows = Vec::new();
    let ft = flow_type;
    let op = operation;

    if matches_any(ft, op, &[FlowOperation::TruckDispatch, FlowOperation::DirectDispatch, FlowOperation::Bunkering, FlowOperation::InternalDispatch]) {
      self.collect_dispatches(&mut rows, op, status, contractor_id).await?;
    }

    if matches_any(ft, op, &[FlowOperation::TruckReceipt, FlowOperation::RailReceipt, FlowOperation::ExternalAcceptance, FlowOperation::TransitReceipt]) {
      if status != Some(PipelineStatus::Pending) {
        self.collect_acceptances(&mut rows, op, status, contractor_id).await?;
      }
      if status.is_none() || status == Some(PipelineStatus::Pending) {
        self.collect_pending_waybills(&mut rows, op, contractor_id).await?;
      }
    }

    if FlowOperation::Blending.matches_filter(ft, op.as_ref()) {
      self.collect_blending(&mut rows, status, contractor_id).await?;
    }
    if FlowOperation::PhysicalTransfer.matches_filter(ft, op.as_ref()) {
      self.collect_physical_transfers(&mut rows, status, contractor_id).await?;
    }
    if FlowOperation::OwnershipTransfer.matches_filter(ft, op.as_ref()) {
      self.collect_ownership_transfers(&mut rows, status, contractor_id).await?;
    }
    if FlowOperation::InventoryReconciliation.matches_filter(ft, op.as_ref()) {
      self.collect_reconciliations(&mut rows, status, contractor_id).await?;
    }

    rows.sort_by(|a, b| b.date.cmp(&a.date));
    Ok(paginate(rows, page, per_page))
  }

  async fn collect_dispatches(
    &self,
    rows: &mut Vec<CargoFlowRow>,
    filter_op: Option<FlowOperation>,
    status: Option<PipelineStatus>,
    contractor_id: Option<Uuid>,
  ) -> Result<(), ApiError> {
    let base = Condition::all().add(dispatch_document::Column::DeletedAt.is_null());
    let Some(mut cond) = Self::add_status_filter(base, status, dispatch_document::Column::Status)
    else {
      return Ok(());
    };
    if let Some(cid) = contractor_id {
      cond = cond.add(dispatch_document::Column::ContractorId.eq(cid));
    }

    let docs = dispatch_document::Entity::find()
      .filter(cond)
      .order_by_desc(dispatch_document::Column::Date)
      .all(self.db.as_ref())
      .await?;

    let company_map = self.resolve_companies(&ids(&docs, |d| d.contractor_id)).await?;
    let items = self.load_children::<dispatch_item::Entity>(
      dispatch_item::Column::DispatchDocId, &ids(&docs, |d| d.id),
    ).await?;
    let first = Self::first_per_parent(&items, |i| i.dispatch_doc_id);
    let product_map = self.resolve_products(&ids_from_map(&first, |i| i.product_id)).await?;

    for d in &docs {
      let op = FlowOperation::from_dispatch(d.dispatch_method, d.dispatch_purpose);
      if !op.matches_filter(None, filter_op.as_ref()) { continue; }

      let item = first.get(&d.id);
      rows.push(make_row(
        d.id, d.document_number.clone(), d.date.to_string(), op,
        Some(d.contractor_id), Some(Self::company_name(&company_map, d.contractor_id)),
        item.and_then(|i| product_map.get(&i.product_id).cloned()),
        item.map(|i| i.dispatched_amount),
        PipelineStatus::from_doc_status(Some(&d.status)),
      ));
    }
    Ok(())
  }

  async fn collect_acceptances(
    &self,
    rows: &mut Vec<CargoFlowRow>,
    filter_op: Option<FlowOperation>,
    status: Option<PipelineStatus>,
    contractor_id: Option<Uuid>,
  ) -> Result<(), ApiError> {
    let base = Condition::all().add(acceptance_document::Column::DeletedAt.is_null());
    let Some(cond) = Self::add_status_filter(base, status, acceptance_document::Column::Status)
    else {
      return Ok(());
    };

    let docs = acceptance_document::Entity::find()
      .filter(cond)
      .order_by_desc(acceptance_document::Column::DateAccepted)
      .all(self.db.as_ref())
      .await?;

    let twb_ids: Vec<Uuid> = docs.iter().filter_map(|d| d.truck_waybill_id).collect();
    let twb_map: std::collections::HashMap<Uuid, truck_waybill::Model> = if twb_ids.is_empty() {
      Default::default()
    } else {
      truck_waybill::Entity::find()
        .filter(truck_waybill::Column::Id.is_in(twb_ids))
        .all(self.db.as_ref()).await?
        .into_iter().map(|w| (w.id, w)).collect()
    };
    let rwb_ids: Vec<Uuid> = docs.iter().filter_map(|d| d.rail_waybill_id).collect();
    let rwb_map: std::collections::HashMap<Uuid, rail_waybill::Model> = if rwb_ids.is_empty() {
      Default::default()
    } else {
      rail_waybill::Entity::find()
        .filter(rail_waybill::Column::Id.is_in(rwb_ids))
        .all(self.db.as_ref()).await?
        .into_iter().map(|w| (w.id, w)).collect()
    };

    let all_cids: Vec<Uuid> = docs.iter()
      .filter_map(|d| contractor_from_waybill(d, &twb_map, &rwb_map))
      .collect();
    let company_map = self.resolve_companies(&all_cids).await?;

    let items = self.load_children::<acceptance_item::Entity>(
      acceptance_item::Column::AcceptanceDocId, &ids(&docs, |d| d.id),
    ).await?;
    let first = Self::first_per_parent(&items, |i| i.acceptance_doc_id);
    let product_map = self.resolve_products(&ids_from_map(&first, |i| i.product_id)).await?;

    for d in &docs {
      let (op, cid) = classify_acceptance(d, &twb_map, &rwb_map);
      if !op.matches_filter(None, filter_op.as_ref()) { continue; }
      if contractor_id.is_some() && cid != contractor_id { continue; }

      let item = first.get(&d.id);
      rows.push(make_row(
        d.id, d.document_number.clone(), d.date_accepted.to_string(), op,
        cid, cid.and_then(|c| company_map.get(&c).cloned()),
        item.and_then(|i| product_map.get(&i.product_id).cloned()),
        item.map(|i| i.accepted_amount),
        PipelineStatus::from_doc_status(Some(&d.status)),
      ));
    }
    Ok(())
  }

  async fn collect_pending_waybills(
    &self,
    rows: &mut Vec<CargoFlowRow>,
    filter_op: Option<FlowOperation>,
    contractor_id: Option<Uuid>,
  ) -> Result<(), ApiError> {
    if FlowOperation::TruckReceipt.matches_filter(None, filter_op.as_ref()) {
      self.collect_pending_truck(rows, contractor_id).await?;
    }
    if FlowOperation::RailReceipt.matches_filter(None, filter_op.as_ref()) {
      self.collect_pending_rail(rows, contractor_id).await?;
    }
    Ok(())
  }

  async fn collect_pending_truck(&self, rows: &mut Vec<CargoFlowRow>, contractor_id: Option<Uuid>) -> Result<(), ApiError> {
    let mut cond = Condition::all().add(truck_waybill::Column::DeletedAt.is_null());
    if let Some(cid) = contractor_id {
      cond = cond.add(truck_waybill::Column::SenderId.eq(cid));
    }
    let waybills = truck_waybill::Entity::find().filter(cond).all(self.db.as_ref()).await?;
    let wb_ids = ids(&waybills, |w| w.id);

    let linked = self.linked_waybill_ids(acceptance_document::Column::TruckWaybillId, &wb_ids).await?;
    let company_map = self.resolve_companies(&ids(&waybills, |w| w.sender_id)).await?;

    for w in &waybills {
      if linked.contains(&w.id) { continue; }
      rows.push(make_row(
        w.id, w.document_number.clone(), w.date.to_string(), FlowOperation::TruckReceipt,
        Some(w.sender_id), Some(Self::company_name(&company_map, w.sender_id)),
        None, None, PipelineStatus::Pending,
      ));
    }
    Ok(())
  }

  async fn collect_pending_rail(&self, rows: &mut Vec<CargoFlowRow>, contractor_id: Option<Uuid>) -> Result<(), ApiError> {
    let mut cond = Condition::all().add(rail_waybill::Column::DeletedAt.is_null());
    if let Some(cid) = contractor_id {
      cond = cond.add(rail_waybill::Column::SenderId.eq(cid));
    }
    let waybills = rail_waybill::Entity::find().filter(cond).all(self.db.as_ref()).await?;
    let wb_ids = ids(&waybills, |w| w.id);

    let linked = self.linked_waybill_ids(acceptance_document::Column::RailWaybillId, &wb_ids).await?;
    let company_map = self.resolve_companies(&ids(&waybills, |w| w.sender_id)).await?;

    for w in &waybills {
      if linked.contains(&w.id) { continue; }
      rows.push(make_row(
        w.id, w.document_number.clone(), w.date.to_string(), FlowOperation::RailReceipt,
        Some(w.sender_id), Some(Self::company_name(&company_map, w.sender_id)),
        None, None, PipelineStatus::Pending,
      ));
    }
    Ok(())
  }

  async fn collect_blending(&self, rows: &mut Vec<CargoFlowRow>, status: Option<PipelineStatus>, contractor_id: Option<Uuid>) -> Result<(), ApiError> {
    let base = Condition::all().add(blending_document::Column::DeletedAt.is_null());
    let Some(mut cond) = Self::add_status_filter(base, status, blending_document::Column::Status) else { return Ok(()); };
    if let Some(cid) = contractor_id {
      cond = cond.add(blending_document::Column::ContractorId.eq(cid));
    }

    let docs = blending_document::Entity::find().filter(cond).all(self.db.as_ref()).await?;
    let company_map = self.resolve_companies(&ids(&docs, |d| d.contractor_id)).await?;
    let product_map = self.resolve_products(&ids(&docs, |d| d.target_product_id)).await?;

    for d in &docs {
      rows.push(make_row(
        d.id, d.document_number.clone(), d.date.to_string(), FlowOperation::Blending,
        Some(d.contractor_id), Some(Self::company_name(&company_map, d.contractor_id)),
        product_map.get(&d.target_product_id).cloned(), None,
        PipelineStatus::from_doc_status(Some(&d.status)),
      ));
    }
    Ok(())
  }

  async fn collect_physical_transfers(&self, rows: &mut Vec<CargoFlowRow>, status: Option<PipelineStatus>, contractor_id: Option<Uuid>) -> Result<(), ApiError> {
    let base = Condition::all().add(physical_storage_transfer::Column::DeletedAt.is_null());
    let Some(cond) = Self::add_status_filter(base, status, physical_storage_transfer::Column::Status) else { return Ok(()); };

    let docs = physical_storage_transfer::Entity::find().filter(cond).all(self.db.as_ref()).await?;
    let items = self.load_children::<physical_transfer_item::Entity>(
      physical_transfer_item::Column::PhysicalTransferId, &ids(&docs, |d| d.id),
    ).await?;
    let first = Self::first_per_parent(&items, |i| i.physical_transfer_id);

    let company_map = self.resolve_companies(&ids_from_map(&first, |i| i.contractor_id)).await?;
    let product_map = self.resolve_products(&ids_from_map(&first, |i| i.product_id)).await?;

    for d in &docs {
      let item = first.get(&d.id);
      let cid = item.map(|i| i.contractor_id);
      if contractor_id.is_some() && cid != contractor_id { continue; }

      rows.push(make_row(
        d.id, d.document_number.clone(), d.date.to_string(), FlowOperation::PhysicalTransfer,
        cid, cid.map(|c| Self::company_name(&company_map, c)),
        item.and_then(|i| product_map.get(&i.product_id).cloned()),
        item.map(|i| i.amount),
        PipelineStatus::from_doc_status(Some(&d.status)),
      ));
    }
    Ok(())
  }

  async fn collect_ownership_transfers(&self, rows: &mut Vec<CargoFlowRow>, status: Option<PipelineStatus>, contractor_id: Option<Uuid>) -> Result<(), ApiError> {
    let base = Condition::all().add(ownership_transfer::Column::DeletedAt.is_null());
    let Some(cond) = Self::add_status_filter(base, status, ownership_transfer::Column::Status) else { return Ok(()); };

    let docs = ownership_transfer::Entity::find().filter(cond).all(self.db.as_ref()).await?;
    let items = self.load_children::<ownership_transfer_item::Entity>(
      ownership_transfer_item::Column::OwnershipTransferId, &ids(&docs, |d| d.id),
    ).await?;
    let first = Self::first_per_parent(&items, |i| i.ownership_transfer_id);

    let company_map = self.resolve_companies(&ids_from_map(&first, |i| i.from_contractor_id)).await?;
    let product_map = self.resolve_products(&ids_from_map(&first, |i| i.product_id)).await?;

    for d in &docs {
      let item = first.get(&d.id);
      let cid = item.map(|i| i.from_contractor_id);
      if contractor_id.is_some() && cid != contractor_id { continue; }

      rows.push(make_row(
        d.id, d.id.to_string(), d.date.to_string(), FlowOperation::OwnershipTransfer,
        cid, cid.map(|c| Self::company_name(&company_map, c)),
        item.and_then(|i| product_map.get(&i.product_id).cloned()),
        item.map(|i| i.amount),
        PipelineStatus::from_doc_status(Some(&d.status)),
      ));
    }
    Ok(())
  }

  async fn collect_reconciliations(&self, rows: &mut Vec<CargoFlowRow>, status: Option<PipelineStatus>, contractor_id: Option<Uuid>) -> Result<(), ApiError> {
    let base = Condition::all().add(inventory_reconciliation::Column::DeletedAt.is_null());
    let Some(cond) = Self::add_status_filter(base, status, inventory_reconciliation::Column::Status) else { return Ok(()); };

    let docs = inventory_reconciliation::Entity::find().filter(cond).all(self.db.as_ref()).await?;
    let items = self.load_children::<inventory_adjustment::Entity>(
      inventory_adjustment::Column::ReconciliationId, &ids(&docs, |d| d.id),
    ).await?;
    let first = Self::first_per_parent(&items, |i| i.reconciliation_id);

    let company_map = self.resolve_companies(&ids_from_map(&first, |i| i.contractor_id)).await?;
    let product_map = self.resolve_products(&ids_from_map(&first, |i| i.product_id)).await?;

    for d in &docs {
      let item = first.get(&d.id);
      let cid = item.map(|i| i.contractor_id);
      if contractor_id.is_some() && cid != contractor_id { continue; }

      rows.push(make_row(
        d.id, d.document_number.clone(), d.date.to_string(), FlowOperation::InventoryReconciliation,
        cid, cid.map(|c| Self::company_name(&company_map, c)),
        item.and_then(|i| product_map.get(&i.product_id).cloned()),
        None,
        PipelineStatus::from_doc_status(Some(&d.status)),
      ));
    }
    Ok(())
  }

  async fn load_children<E: EntityTrait>(
    &self, parent_col: impl ColumnTrait, parent_ids: &[Uuid],
  ) -> Result<Vec<E::Model>, ApiError>
  where E::Model: Send {
    if parent_ids.is_empty() { return Ok(vec![]); }
    Ok(E::find()
      .filter(parent_col.is_in(parent_ids.to_vec()))
      .all(self.db.as_ref())
      .await?)
  }

  async fn linked_waybill_ids(
    &self, link_col: impl ColumnTrait, wb_ids: &[Uuid],
  ) -> Result<std::collections::HashSet<Uuid>, ApiError> {
    if wb_ids.is_empty() { return Ok(Default::default()); }
    Ok(acceptance_document::Entity::find()
      .filter(Condition::all()
        .add(link_col.is_in(wb_ids.to_vec()))
        .add(acceptance_document::Column::DeletedAt.is_null()))
      .all(self.db.as_ref()).await?
      .iter()
      .filter_map(|a| a.truck_waybill_id.or(a.rail_waybill_id))
      .collect())
  }
}

fn matches_any(ft: Option<FlowType>, op: Option<FlowOperation>, ops: &[FlowOperation]) -> bool {
  ops.iter().any(|o| o.matches_filter(ft, op.as_ref()))
}

fn classify_acceptance(
  doc: &acceptance_document::Model,
  twb: &std::collections::HashMap<Uuid, truck_waybill::Model>,
  rwb: &std::collections::HashMap<Uuid, rail_waybill::Model>,
) -> (FlowOperation, Option<Uuid>) {
  if let Some(id) = doc.truck_waybill_id {
    (FlowOperation::TruckReceipt, twb.get(&id).map(|w| w.sender_id))
  } else if let Some(id) = doc.rail_waybill_id {
    (FlowOperation::RailReceipt, rwb.get(&id).map(|w| w.sender_id))
  } else if doc.transit_dispatch_id.is_some() {
    (FlowOperation::TransitReceipt, None)
  } else {
    (FlowOperation::ExternalAcceptance, None)
  }
}

fn contractor_from_waybill(
  doc: &acceptance_document::Model,
  twb: &std::collections::HashMap<Uuid, truck_waybill::Model>,
  rwb: &std::collections::HashMap<Uuid, rail_waybill::Model>,
) -> Option<Uuid> {
  doc.truck_waybill_id.and_then(|id| twb.get(&id).map(|w| w.sender_id))
    .or_else(|| doc.rail_waybill_id.and_then(|id| rwb.get(&id).map(|w| w.sender_id)))
}

fn ids<T>(items: &[T], f: impl Fn(&T) -> Uuid) -> Vec<Uuid> {
  items.iter().map(f).collect()
}

fn ids_from_map<T>(map: &std::collections::HashMap<Uuid, &T>, f: impl Fn(&T) -> Uuid) -> Vec<Uuid> {
  map.values().map(|v| f(v)).collect()
}

fn make_row(
  id: Uuid, document_number: String, date: String, operation: FlowOperation,
  contractor_id: Option<Uuid>, contractor_name: Option<String>,
  product_name: Option<String>, quantity: Option<sea_orm::entity::prelude::Decimal>,
  status: PipelineStatus,
) -> CargoFlowRow {
  CargoFlowRow {
    id, document_number, date,
    flow_type: operation.flow_type(),
    operation,
    contractor_id, contractor_name, product_name, quantity, status,
    entity_type: operation.entity_type(),
  }
}

fn paginate(rows: Vec<CargoFlowRow>, page: u64, per_page: u64) -> Vec<CargoFlowRow> {
  let start = ((page - 1) * per_page) as usize;
  if start >= rows.len() { return vec![]; }
  let end = (start + per_page as usize).min(rows.len());
  rows[start..end].to_vec()
}
