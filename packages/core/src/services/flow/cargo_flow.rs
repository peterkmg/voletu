use std::collections::HashMap;

use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use super::FlowService;
use crate::{
  api::ApiError,
  dtos::response::flow::CargoFlowRow,
  entities::{
    acceptance_document, acceptance_item, blending_document,
    company, dispatch_document, dispatch_item, inventory_reconciliation,
    inventory_adjustment, ownership_transfer, ownership_transfer_item,
    physical_storage_transfer, physical_transfer_item, product,
    rail_waybill, truck_waybill,
  },
  enums::DocumentStatus,
};

/// Internal helper to map DocumentStatus to cargo flow status string.
fn status_str(s: &DocumentStatus) -> &'static str {
  match s {
    DocumentStatus::Draft => "draft",
    DocumentStatus::Posted => "executed",
  }
}

impl FlowService {
  /// Query the unified cargo flow view across all document types.
  ///
  /// Returns a merged, date-sorted list of all inventory-impacting documents
  /// with common fields projected. Supports filtering by flow_type, operation,
  /// status, and contractor_id.
  #[allow(clippy::too_many_arguments)]
  pub async fn cargo_flow_query(
    &self,
    flow_type: Option<&str>,
    operation: Option<&str>,
    status: Option<&str>,
    contractor_id: Option<Uuid>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<CargoFlowRow>, ApiError> {
    let (page, per_page) = crate::services::common::normalize_pagination(page, per_page)?;

    let mut rows: Vec<CargoFlowRow> = Vec::new();

    // ── 1. Dispatch documents (Outgoing) ───────────────────────────────
    if flow_type.is_none() || flow_type == Some("Outgoing") {
      self.collect_dispatch_rows(&mut rows, operation, status, contractor_id).await?;
    }

    // ── 2. Acceptance documents (Incoming, only those with status = draft/posted) ──
    if flow_type.is_none() || flow_type == Some("Incoming") {
      self.collect_acceptance_rows(&mut rows, operation, status, contractor_id).await?;
    }

    // ── 3. Pending waybills (Incoming, no acceptance yet) ──────────────
    if flow_type.is_none() || flow_type == Some("Incoming") {
      if status.is_none() || status == Some("pending") {
        self.collect_pending_waybills(&mut rows, operation, contractor_id).await?;
      }
    }

    // ── 4. Blending documents (Internal) ───────────────────────────────
    if flow_type.is_none() || flow_type == Some("Internal") {
      if operation.is_none() || operation == Some("Blending") {
        self.collect_blending_rows(&mut rows, status, contractor_id).await?;
      }
    }

    // ── 5. Physical storage transfers (Internal) ───────────────────────
    if flow_type.is_none() || flow_type == Some("Internal") {
      if operation.is_none() || operation == Some("Physical Transfer") {
        self.collect_physical_transfer_rows(&mut rows, status, contractor_id).await?;
      }
    }

    // ── 6. Ownership transfers (Internal) ──────────────────────────────
    if flow_type.is_none() || flow_type == Some("Internal") {
      if operation.is_none() || operation == Some("Ownership Transfer") {
        self.collect_ownership_transfer_rows(&mut rows, status, contractor_id).await?;
      }
    }

    // ── 7. Inventory reconciliations (Internal) ────────────────────────
    if flow_type.is_none() || flow_type == Some("Internal") {
      if operation.is_none() || operation == Some("Inventory Reconciliation") {
        self.collect_reconciliation_rows(&mut rows, status, contractor_id).await?;
      }
    }

    // ── Sort by date descending, paginate ──────────────────────────────
    rows.sort_by(|a, b| b.date.cmp(&a.date));

    let start = ((page - 1) * per_page) as usize;
    let end = (start + per_page as usize).min(rows.len());
    if start >= rows.len() {
      return Ok(vec![]);
    }
    Ok(rows[start..end].to_vec())
  }

  // ── Dispatch documents ─────────────────────────────────────────────

  async fn collect_dispatch_rows(
    &self,
    rows: &mut Vec<CargoFlowRow>,
    operation: Option<&str>,
    status: Option<&str>,
    contractor_id: Option<Uuid>,
  ) -> Result<(), ApiError> {
    let mut cond = Condition::all().add(dispatch_document::Column::DeletedAt.is_null());
    if let Some(cid) = contractor_id {
      cond = cond.add(dispatch_document::Column::ContractorId.eq(cid));
    }
    if let Some(s) = status {
      match s {
        "draft" => { cond = cond.add(dispatch_document::Column::Status.eq(DocumentStatus::Draft)); }
        "executed" => { cond = cond.add(dispatch_document::Column::Status.eq(DocumentStatus::Posted)); }
        _ => {}
      }
    }

    let docs = dispatch_document::Entity::find()
      .filter(cond)
      .order_by_desc(dispatch_document::Column::Date)
      .all(self.db.as_ref())
      .await?;

    // Resolve contractor names
    let cids: Vec<Uuid> = docs.iter().map(|d| d.contractor_id).collect();
    let company_map = self.resolve_companies(&cids).await?;

    // Resolve first item product per dispatch
    let doc_ids: Vec<Uuid> = docs.iter().map(|d| d.id).collect();
    let items = if doc_ids.is_empty() { vec![] } else {
      dispatch_item::Entity::find()
        .filter(Condition::all()
          .add(dispatch_item::Column::DispatchDocId.is_in(doc_ids))
          .add(dispatch_item::Column::DeletedAt.is_null()))
        .all(self.db.as_ref()).await?
    };
    let mut first_item: HashMap<Uuid, &dispatch_item::Model> = HashMap::new();
    for i in &items { first_item.entry(i.dispatch_doc_id).or_insert(i); }

    let pids: Vec<Uuid> = first_item.values().map(|i| i.product_id).collect();
    let product_map = self.resolve_products(&pids).await?;

    for d in &docs {
      let op = match (d.dispatch_method, d.dispatch_purpose) {
        (crate::enums::DispatchMethod::Bunkering, _) => "Bunkering",
        (crate::enums::DispatchMethod::VesselTerminal, _) => "Direct Dispatch",
        (crate::enums::DispatchMethod::Truck, crate::enums::DispatchPurpose::Internal) => "Internal Dispatch",
        (crate::enums::DispatchMethod::Truck, crate::enums::DispatchPurpose::External) => "Truck Dispatch",
      };
      if let Some(filter_op) = operation {
        if filter_op != op { continue; }
      }

      let item = first_item.get(&d.id);
      rows.push(CargoFlowRow {
        id: d.id,
        document_number: d.document_number.clone(),
        date: d.date.to_string(),
        flow_type: "Outgoing".to_owned(),
        operation: op.to_owned(),
        contractor_id: Some(d.contractor_id),
        contractor_name: Some(company_map.get(&d.contractor_id).cloned().unwrap_or_else(|| "Unknown".to_owned())),
        product_name: item.and_then(|i| product_map.get(&i.product_id).map(|n| n.to_string())),
        quantity: item.map(|i| i.dispatched_amount),
        status: status_str(&d.status).to_owned(),
        entity_type: "dispatch".to_owned(),
        flow_route: match op {
          "Bunkering" => "/outgoing/bunkering",
          "Direct Dispatch" => "/outgoing/direct",
          "Internal Dispatch" => "/outgoing/direct",
          _ => "/outgoing/truck",
        }.to_owned(),
      });
    }
    Ok(())
  }

  // ── Acceptance documents ───────────────────────────────────────────

  async fn collect_acceptance_rows(
    &self,
    rows: &mut Vec<CargoFlowRow>,
    operation: Option<&str>,
    status: Option<&str>,
    contractor_id: Option<Uuid>,
  ) -> Result<(), ApiError> {
    let mut cond = Condition::all().add(acceptance_document::Column::DeletedAt.is_null());
    if let Some(s) = status {
      match s {
        "draft" => { cond = cond.add(acceptance_document::Column::Status.eq(DocumentStatus::Draft)); }
        "executed" => { cond = cond.add(acceptance_document::Column::Status.eq(DocumentStatus::Posted)); }
        "pending" => return Ok(()), // pending waybills handled separately
        _ => {}
      }
    }

    let docs = acceptance_document::Entity::find()
      .filter(cond)
      .order_by_desc(acceptance_document::Column::DateAccepted)
      .all(self.db.as_ref())
      .await?;

    // Determine contractor from linked waybills
    let twb_ids: Vec<Uuid> = docs.iter().filter_map(|d| d.truck_waybill_id).collect();
    let rwb_ids: Vec<Uuid> = docs.iter().filter_map(|d| d.rail_waybill_id).collect();

    let truck_waybills: HashMap<Uuid, truck_waybill::Model> = if twb_ids.is_empty() { HashMap::new() } else {
      truck_waybill::Entity::find()
        .filter(truck_waybill::Column::Id.is_in(twb_ids))
        .all(self.db.as_ref()).await?
        .into_iter().map(|w| (w.id, w)).collect()
    };
    let rail_waybills: HashMap<Uuid, rail_waybill::Model> = if rwb_ids.is_empty() { HashMap::new() } else {
      rail_waybill::Entity::find()
        .filter(rail_waybill::Column::Id.is_in(rwb_ids))
        .all(self.db.as_ref()).await?
        .into_iter().map(|w| (w.id, w)).collect()
    };

    // Collect all contractor IDs from waybills
    let mut all_cids: Vec<Uuid> = Vec::new();
    for d in &docs {
      if let Some(twb_id) = d.truck_waybill_id {
        if let Some(tw) = truck_waybills.get(&twb_id) { all_cids.push(tw.sender_id); }
      } else if let Some(rwb_id) = d.rail_waybill_id {
        if let Some(rw) = rail_waybills.get(&rwb_id) { all_cids.push(rw.sender_id); }
      }
    }
    let company_map = self.resolve_companies(&all_cids).await?;

    // Resolve first acceptance item product
    let doc_ids: Vec<Uuid> = docs.iter().map(|d| d.id).collect();
    let items = if doc_ids.is_empty() { vec![] } else {
      acceptance_item::Entity::find()
        .filter(Condition::all()
          .add(acceptance_item::Column::AcceptanceDocId.is_in(doc_ids))
          .add(acceptance_item::Column::DeletedAt.is_null()))
        .all(self.db.as_ref()).await?
    };
    let mut first_item: HashMap<Uuid, &acceptance_item::Model> = HashMap::new();
    for i in &items { first_item.entry(i.acceptance_doc_id).or_insert(i); }

    let pids: Vec<Uuid> = first_item.values().map(|i| i.product_id).collect();
    let product_map = self.resolve_products(&pids).await?;

    for d in &docs {
      let (op, route, cid) = if d.truck_waybill_id.is_some() {
        let c = d.truck_waybill_id.and_then(|id| truck_waybills.get(&id).map(|w| w.sender_id));
        ("Truck Receipt", "/incoming/truck", c)
      } else if d.rail_waybill_id.is_some() {
        let c = d.rail_waybill_id.and_then(|id| rail_waybills.get(&id).map(|w| w.sender_id));
        ("Rail Receipt", "/incoming/rail", c)
      } else if d.transit_dispatch_id.is_some() {
        ("Transit Receipt", "/incoming/external", None)
      } else {
        ("External Acceptance", "/incoming/external", None)
      };

      if let Some(filter_op) = operation {
        if filter_op != op { continue; }
      }
      if let Some(filter_cid) = contractor_id {
        if cid != Some(filter_cid) { continue; }
      }

      let item = first_item.get(&d.id);
      rows.push(CargoFlowRow {
        id: d.id,
        document_number: d.document_number.clone(),
        date: d.date_accepted.to_string(),
        flow_type: "Incoming".to_owned(),
        operation: op.to_owned(),
        contractor_id: cid,
        contractor_name: cid.and_then(|c| company_map.get(&c).map(|n| n.to_string())),
        product_name: item.and_then(|i| product_map.get(&i.product_id).map(|n| n.to_string())),
        quantity: item.map(|i| i.accepted_amount),
        status: status_str(&d.status).to_owned(),
        entity_type: "acceptance".to_owned(),
        flow_route: route.to_owned(),
      });
    }
    Ok(())
  }

  // ── Pending waybills (no acceptance yet) ───────────────────────────

  async fn collect_pending_waybills(
    &self,
    rows: &mut Vec<CargoFlowRow>,
    operation: Option<&str>,
    contractor_id: Option<Uuid>,
  ) -> Result<(), ApiError> {
    // Truck waybills without acceptance
    if operation.is_none() || operation == Some("Truck Receipt") {
      let mut cond = Condition::all().add(truck_waybill::Column::DeletedAt.is_null());
      if let Some(cid) = contractor_id {
        cond = cond.add(truck_waybill::Column::SenderId.eq(cid));
      }
      let waybills = truck_waybill::Entity::find()
        .filter(cond)
        .all(self.db.as_ref()).await?;

      // Find which have acceptances
      let wb_ids: Vec<Uuid> = waybills.iter().map(|w| w.id).collect();
      let linked: Vec<Uuid> = if wb_ids.is_empty() { vec![] } else {
        acceptance_document::Entity::find()
          .filter(Condition::all()
            .add(acceptance_document::Column::TruckWaybillId.is_in(wb_ids))
            .add(acceptance_document::Column::DeletedAt.is_null()))
          .all(self.db.as_ref()).await?
          .iter().filter_map(|a| a.truck_waybill_id).collect()
      };
      let linked_set: std::collections::HashSet<Uuid> = linked.into_iter().collect();

      let cids: Vec<Uuid> = waybills.iter().map(|w| w.sender_id).collect();
      let company_map = self.resolve_companies(&cids).await?;

      for w in &waybills {
        if linked_set.contains(&w.id) { continue; }
        rows.push(CargoFlowRow {
          id: w.id,
          document_number: w.document_number.clone(),
          date: w.date.to_string(),
          flow_type: "Incoming".to_owned(),
          operation: "Truck Receipt".to_owned(),
          contractor_id: Some(w.sender_id),
          contractor_name: Some(company_map.get(&w.sender_id).cloned().unwrap_or_else(|| "Unknown".to_owned())),
          product_name: None,
          quantity: None,
          status: "pending".to_owned(),
          entity_type: "truck_waybill".to_owned(),
          flow_route: "/incoming/truck".to_owned(),
        });
      }
    }

    // Rail waybills without acceptance (same pattern)
    if operation.is_none() || operation == Some("Rail Receipt") {
      let mut cond = Condition::all().add(rail_waybill::Column::DeletedAt.is_null());
      if let Some(cid) = contractor_id {
        cond = cond.add(rail_waybill::Column::SenderId.eq(cid));
      }
      let waybills = rail_waybill::Entity::find()
        .filter(cond)
        .all(self.db.as_ref()).await?;

      let wb_ids: Vec<Uuid> = waybills.iter().map(|w| w.id).collect();
      let linked: Vec<Uuid> = if wb_ids.is_empty() { vec![] } else {
        acceptance_document::Entity::find()
          .filter(Condition::all()
            .add(acceptance_document::Column::RailWaybillId.is_in(wb_ids))
            .add(acceptance_document::Column::DeletedAt.is_null()))
          .all(self.db.as_ref()).await?
          .iter().filter_map(|a| a.rail_waybill_id).collect()
      };
      let linked_set: std::collections::HashSet<Uuid> = linked.into_iter().collect();

      let cids: Vec<Uuid> = waybills.iter().map(|w| w.sender_id).collect();
      let company_map = self.resolve_companies(&cids).await?;

      for w in &waybills {
        if linked_set.contains(&w.id) { continue; }
        rows.push(CargoFlowRow {
          id: w.id,
          document_number: w.document_number.clone(),
          date: w.date.to_string(),
          flow_type: "Incoming".to_owned(),
          operation: "Rail Receipt".to_owned(),
          contractor_id: Some(w.sender_id),
          contractor_name: Some(company_map.get(&w.sender_id).cloned().unwrap_or_else(|| "Unknown".to_owned())),
          product_name: None,
          quantity: None,
          status: "pending".to_owned(),
          entity_type: "rail_waybill".to_owned(),
          flow_route: "/incoming/rail".to_owned(),
        });
      }
    }

    Ok(())
  }

  // ── Blending documents ─────────────────────────────────────────────

  async fn collect_blending_rows(
    &self,
    rows: &mut Vec<CargoFlowRow>,
    status: Option<&str>,
    contractor_id: Option<Uuid>,
  ) -> Result<(), ApiError> {
    let mut cond = Condition::all().add(blending_document::Column::DeletedAt.is_null());
    if let Some(cid) = contractor_id {
      cond = cond.add(blending_document::Column::ContractorId.eq(cid));
    }
    if let Some(s) = status {
      match s {
        "draft" => { cond = cond.add(blending_document::Column::Status.eq(DocumentStatus::Draft)); }
        "executed" => { cond = cond.add(blending_document::Column::Status.eq(DocumentStatus::Posted)); }
        _ => return Ok(()),
      }
    }

    let docs = blending_document::Entity::find()
      .filter(cond)
      .all(self.db.as_ref()).await?;

    let cids: Vec<Uuid> = docs.iter().map(|d| d.contractor_id).collect();
    let company_map = self.resolve_companies(&cids).await?;
    let pids: Vec<Uuid> = docs.iter().map(|d| d.target_product_id).collect();
    let product_map = self.resolve_products(&pids).await?;

    for d in &docs {
      rows.push(CargoFlowRow {
        id: d.id,
        document_number: d.document_number.clone(),
        date: d.date.to_string(),
        flow_type: "Internal".to_owned(),
        operation: "Blending".to_owned(),
        contractor_id: Some(d.contractor_id),
        contractor_name: Some(company_map.get(&d.contractor_id).cloned().unwrap_or_else(|| "Unknown".to_owned())),
        product_name: product_map.get(&d.target_product_id).map(|n| n.to_string()),
        quantity: None, // blending has components/results, not a single quantity
        status: status_str(&d.status).to_owned(),
        entity_type: "blending".to_owned(),
        flow_route: "/internal/blending".to_owned(),
      });
    }
    Ok(())
  }

  // ── Physical storage transfers ─────────────────────────────────────

  async fn collect_physical_transfer_rows(
    &self,
    rows: &mut Vec<CargoFlowRow>,
    status: Option<&str>,
    _contractor_id: Option<Uuid>,
  ) -> Result<(), ApiError> {
    let mut cond = Condition::all().add(physical_storage_transfer::Column::DeletedAt.is_null());
    if let Some(s) = status {
      match s {
        "draft" => { cond = cond.add(physical_storage_transfer::Column::Status.eq(DocumentStatus::Draft)); }
        "executed" => { cond = cond.add(physical_storage_transfer::Column::Status.eq(DocumentStatus::Posted)); }
        _ => return Ok(()),
      }
    }

    let docs = physical_storage_transfer::Entity::find()
      .filter(cond)
      .all(self.db.as_ref()).await?;

    // Resolve contractor from first transfer item
    let doc_ids: Vec<Uuid> = docs.iter().map(|d| d.id).collect();
    let items = if doc_ids.is_empty() { vec![] } else {
      physical_transfer_item::Entity::find()
        .filter(Condition::all()
          .add(physical_transfer_item::Column::PhysicalTransferId.is_in(doc_ids))
          .add(physical_transfer_item::Column::DeletedAt.is_null()))
        .all(self.db.as_ref()).await?
    };
    let mut first_item: HashMap<Uuid, &physical_transfer_item::Model> = HashMap::new();
    for i in &items { first_item.entry(i.physical_transfer_id).or_insert(i); }

    let cids: Vec<Uuid> = first_item.values().map(|i| i.contractor_id).collect();
    let company_map = self.resolve_companies(&cids).await?;
    let pids: Vec<Uuid> = first_item.values().map(|i| i.product_id).collect();
    let product_map = self.resolve_products(&pids).await?;

    for d in &docs {
      let item = first_item.get(&d.id);
      let cid = item.map(|i| i.contractor_id);

      if let Some(filter_cid) = _contractor_id {
        if cid != Some(filter_cid) { continue; }
      }

      rows.push(CargoFlowRow {
        id: d.id,
        document_number: d.document_number.clone(),
        date: d.date.to_string(),
        flow_type: "Internal".to_owned(),
        operation: "Physical Transfer".to_owned(),
        contractor_id: cid,
        contractor_name: cid.and_then(|c| company_map.get(&c).map(|n| n.to_string())),
        product_name: item.and_then(|i| product_map.get(&i.product_id).map(|n| n.to_string())),
        quantity: item.map(|i| i.amount),
        status: status_str(&d.status).to_owned(),
        entity_type: "physical_transfer".to_owned(),
        flow_route: "/internal/physical-transfer".to_owned(),
      });
    }
    Ok(())
  }

  // ── Ownership transfers ────────────────────────────────────────────

  async fn collect_ownership_transfer_rows(
    &self,
    rows: &mut Vec<CargoFlowRow>,
    status: Option<&str>,
    _contractor_id: Option<Uuid>,
  ) -> Result<(), ApiError> {
    let mut cond = Condition::all().add(ownership_transfer::Column::DeletedAt.is_null());
    if let Some(s) = status {
      match s {
        "draft" => { cond = cond.add(ownership_transfer::Column::Status.eq(DocumentStatus::Draft)); }
        "executed" => { cond = cond.add(ownership_transfer::Column::Status.eq(DocumentStatus::Posted)); }
        _ => return Ok(()),
      }
    }

    let docs = ownership_transfer::Entity::find()
      .filter(cond)
      .all(self.db.as_ref()).await?;

    // Resolve contractor from first item's from_contractor_id
    let doc_ids: Vec<Uuid> = docs.iter().map(|d| d.id).collect();
    let items = if doc_ids.is_empty() { vec![] } else {
      ownership_transfer_item::Entity::find()
        .filter(Condition::all()
          .add(ownership_transfer_item::Column::OwnershipTransferId.is_in(doc_ids))
          .add(ownership_transfer_item::Column::DeletedAt.is_null()))
        .all(self.db.as_ref()).await?
    };
    let mut first_item: HashMap<Uuid, &ownership_transfer_item::Model> = HashMap::new();
    for i in &items { first_item.entry(i.ownership_transfer_id).or_insert(i); }

    let cids: Vec<Uuid> = first_item.values().map(|i| i.from_contractor_id).collect();
    let company_map = self.resolve_companies(&cids).await?;
    let pids: Vec<Uuid> = first_item.values().map(|i| i.product_id).collect();
    let product_map = self.resolve_products(&pids).await?;

    for d in &docs {
      let item = first_item.get(&d.id);
      let cid = item.map(|i| i.from_contractor_id);

      if let Some(filter_cid) = _contractor_id {
        if cid != Some(filter_cid) { continue; }
      }

      // ownership_transfer has no document_number field — use id as display
      let doc_num = d.id.to_string();

      rows.push(CargoFlowRow {
        id: d.id,
        document_number: doc_num,
        date: d.date.to_string(),
        flow_type: "Internal".to_owned(),
        operation: "Ownership Transfer".to_owned(),
        contractor_id: cid,
        contractor_name: cid.and_then(|c| company_map.get(&c).map(|n| n.to_string())),
        product_name: item.and_then(|i| product_map.get(&i.product_id).map(|n| n.to_string())),
        quantity: item.map(|i| i.amount),
        status: status_str(&d.status).to_owned(),
        entity_type: "ownership_transfer".to_owned(),
        flow_route: "/internal/ownership-transfer".to_owned(),
      });
    }
    Ok(())
  }

  // ── Inventory reconciliations ──────────────────────────────────────

  async fn collect_reconciliation_rows(
    &self,
    rows: &mut Vec<CargoFlowRow>,
    status: Option<&str>,
    _contractor_id: Option<Uuid>,
  ) -> Result<(), ApiError> {
    let mut cond = Condition::all().add(inventory_reconciliation::Column::DeletedAt.is_null());
    if let Some(s) = status {
      match s {
        "draft" => { cond = cond.add(inventory_reconciliation::Column::Status.eq(DocumentStatus::Draft)); }
        "executed" => { cond = cond.add(inventory_reconciliation::Column::Status.eq(DocumentStatus::Posted)); }
        _ => return Ok(()),
      }
    }

    let docs = inventory_reconciliation::Entity::find()
      .filter(cond)
      .all(self.db.as_ref()).await?;

    // Resolve contractor from first adjustment
    let doc_ids: Vec<Uuid> = docs.iter().map(|d| d.id).collect();
    let items = if doc_ids.is_empty() { vec![] } else {
      inventory_adjustment::Entity::find()
        .filter(Condition::all()
          .add(inventory_adjustment::Column::ReconciliationId.is_in(doc_ids))
          .add(inventory_adjustment::Column::DeletedAt.is_null()))
        .all(self.db.as_ref()).await?
    };
    let mut first_item: HashMap<Uuid, &inventory_adjustment::Model> = HashMap::new();
    for i in &items { first_item.entry(i.reconciliation_id).or_insert(i); }

    let cids: Vec<Uuid> = first_item.values().map(|i| i.contractor_id).collect();
    let company_map = self.resolve_companies(&cids).await?;
    let pids: Vec<Uuid> = first_item.values().map(|i| i.product_id).collect();
    let product_map = self.resolve_products(&pids).await?;

    for d in &docs {
      let item = first_item.get(&d.id);
      let cid = item.map(|i| i.contractor_id);

      if let Some(filter_cid) = _contractor_id {
        if cid != Some(filter_cid) { continue; }
      }

      rows.push(CargoFlowRow {
        id: d.id,
        document_number: d.document_number.clone(),
        date: d.date.to_string(),
        flow_type: "Internal".to_owned(),
        operation: "Inventory Reconciliation".to_owned(),
        contractor_id: cid,
        contractor_name: cid.and_then(|c| company_map.get(&c).map(|n| n.to_string())),
        product_name: item.and_then(|i| product_map.get(&i.product_id).map(|n| n.to_string())),
        quantity: None,
        status: status_str(&d.status).to_owned(),
        entity_type: "reconciliation".to_owned(),
        flow_route: "/internal/reconciliation".to_owned(),
      });
    }
    Ok(())
  }

  // ── Shared helpers ─────────────────────────────────────────────────

  async fn resolve_companies(&self, ids: &[Uuid]) -> Result<HashMap<Uuid, String>, ApiError> {
    if ids.is_empty() { return Ok(HashMap::new()); }
    let mut unique: Vec<Uuid> = ids.to_vec();
    unique.sort();
    unique.dedup();
    let companies = company::Entity::find()
      .filter(company::Column::Id.is_in(unique))
      .all(self.db.as_ref()).await?;
    Ok(companies.into_iter().map(|c| (c.id, c.common_name)).collect())
  }

  async fn resolve_products(&self, ids: &[Uuid]) -> Result<HashMap<Uuid, String>, ApiError> {
    if ids.is_empty() { return Ok(HashMap::new()); }
    let mut unique: Vec<Uuid> = ids.to_vec();
    unique.sort();
    unique.dedup();
    let products = product::Entity::find()
      .filter(product::Column::Id.is_in(unique))
      .all(self.db.as_ref()).await?;
    Ok(products.into_iter().map(|p| (p.id, p.common_name)).collect())
  }
}
