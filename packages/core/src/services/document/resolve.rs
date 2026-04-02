use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos,
  enums,
  services::{common::resolve_names, DocumentService},
};

impl DocumentService {
  // ── Dispatch ──────────────────────────────

  pub async fn dispatch_document_list_with_names(
    &self,
  ) -> Result<Vec<dtos::DispatchResponse>, ApiError> {
    let mut items = self.dispatch_document_list(None).await?;
    resolve_names(self.db.as_ref(), &mut items).await?;
    Ok(items)
  }

  pub async fn dispatch_document_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::DispatchResponse, ApiError> {
    let mut item = self.dispatch_document_get(id).await?;
    resolve_names(self.db.as_ref(), std::slice::from_mut(&mut item)).await?;
    Ok(item)
  }

  #[allow(clippy::too_many_arguments)]
  pub async fn dispatch_document_query_with_names(
    &self,
    document_number: Option<&str>,
    status: Option<enums::DocumentStatus>,
    contractor_id: Option<Uuid>,
    dispatch_method: Option<enums::DispatchMethod>,
    dispatch_purpose: Option<enums::DispatchPurpose>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<dtos::DispatchResponse>, ApiError> {
    let mut items = self
      .dispatch_document_query(document_number, status, contractor_id, dispatch_method, dispatch_purpose, page, per_page)
      .await?;
    resolve_names(self.db.as_ref(), &mut items).await?;
    Ok(items)
  }

  pub async fn dispatch_composite_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::DispatchCompositeResponse, ApiError> {
    let mut composite = self.dispatch_composite_get(id).await?;
    resolve_names(
      self.db.as_ref(),
      std::slice::from_mut(&mut composite.document),
    )
    .await?;
    resolve_names(self.db.as_ref(), &mut composite.items).await?;
    resolve_names(self.db.as_ref(), &mut composite.storage_measurements).await?;
    Ok(composite)
  }

  // ── Acceptance ────────────────────────────

  pub async fn acceptance_document_list_with_names(
    &self,
  ) -> Result<Vec<dtos::AcceptanceResponse>, ApiError> {
    let mut items = self.acceptance_document_list(None).await?;
    resolve_names(self.db.as_ref(), &mut items).await?;
    Ok(items)
  }

  pub async fn acceptance_document_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::AcceptanceResponse, ApiError> {
    let mut item = self.acceptance_document_get(id).await?;
    resolve_names(self.db.as_ref(), std::slice::from_mut(&mut item)).await?;
    Ok(item)
  }

  pub async fn acceptance_document_query_with_names(
    &self,
    document_number: Option<&str>,
    status: Option<enums::DocumentStatus>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<dtos::AcceptanceResponse>, ApiError> {
    let mut items = self
      .acceptance_document_query(document_number, status, page, per_page)
      .await?;
    resolve_names(self.db.as_ref(), &mut items).await?;
    Ok(items)
  }

  pub async fn acceptance_composite_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::AcceptanceCompositeResponse, ApiError> {
    let mut composite = self.acceptance_composite_get(id).await?;
    resolve_names(
      self.db.as_ref(),
      std::slice::from_mut(&mut composite.document),
    )
    .await?;
    resolve_names(self.db.as_ref(), &mut composite.items).await?;
    Ok(composite)
  }

  // ── Blending ──────────────────────────────

  pub async fn blending_document_list_with_names(
    &self,
  ) -> Result<Vec<dtos::BlendingResponse>, ApiError> {
    let mut items = self.blending_document_list(None).await?;
    resolve_names(self.db.as_ref(), &mut items).await?;
    Ok(items)
  }

  pub async fn blending_document_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::BlendingResponse, ApiError> {
    let mut item = self.blending_document_get(id).await?;
    resolve_names(self.db.as_ref(), std::slice::from_mut(&mut item)).await?;
    Ok(item)
  }

  pub async fn blending_document_query_with_names(
    &self,
    doc_num: Option<&str>,
    status: Option<enums::DocumentStatus>,
    contractor_id: Option<Uuid>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<dtos::BlendingResponse>, ApiError> {
    let mut items = self
      .blending_document_query(doc_num, status, contractor_id, page, per_page)
      .await?;
    resolve_names(self.db.as_ref(), &mut items).await?;
    Ok(items)
  }

  pub async fn blending_composite_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::BlendingCompositeResponse, ApiError> {
    let mut composite = self.blending_composite_get(id).await?;
    resolve_names(
      self.db.as_ref(),
      std::slice::from_mut(&mut composite.document),
    )
    .await?;
    resolve_names(self.db.as_ref(), &mut composite.components).await?;
    resolve_names(self.db.as_ref(), &mut composite.results).await?;
    Ok(composite)
  }

  // ── Reconciliation ────────────────────────

  pub async fn reconciliation_list_with_names(
    &self,
  ) -> Result<Vec<dtos::InventoryReconciliationResponse>, ApiError> {
    let mut items = self.reconciliation_list(None).await?;
    resolve_names(self.db.as_ref(), &mut items).await?;
    Ok(items)
  }

  pub async fn reconciliation_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::InventoryReconciliationResponse, ApiError> {
    let mut item = self.reconciliation_get(id).await?;
    resolve_names(self.db.as_ref(), std::slice::from_mut(&mut item)).await?;
    Ok(item)
  }

  pub async fn reconciliation_query_with_names(
    &self,
    document_number: Option<&str>,
    status: Option<enums::DocumentStatus>,
    warehouse_id: Option<Uuid>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<dtos::InventoryReconciliationResponse>, ApiError> {
    let mut items = self
      .reconciliation_query(document_number, status, warehouse_id, page, per_page)
      .await?;
    resolve_names(self.db.as_ref(), &mut items).await?;
    Ok(items)
  }

  // ── Truck waybill ─────────────────────────

  pub async fn truck_waybill_list_with_names(
    &self,
  ) -> Result<Vec<dtos::TruckWaybillResponse>, ApiError> {
    let mut items = self.truck_waybill_list(None).await?;
    resolve_names(self.db.as_ref(), &mut items).await?;
    Ok(items)
  }

  pub async fn truck_waybill_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::TruckWaybillResponse, ApiError> {
    let mut item = self.truck_waybill_get(id).await?;
    resolve_names(self.db.as_ref(), std::slice::from_mut(&mut item)).await?;
    Ok(item)
  }

  // ── Rail waybill ──────────────────────────

  pub async fn rail_waybill_list_with_names(
    &self,
  ) -> Result<Vec<dtos::RailWaybillResponse>, ApiError> {
    let mut items = self.rail_waybill_list(None).await?;
    resolve_names(self.db.as_ref(), &mut items).await?;
    Ok(items)
  }

  pub async fn rail_waybill_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::RailWaybillResponse, ApiError> {
    let mut item = self.rail_waybill_get(id).await?;
    resolve_names(self.db.as_ref(), std::slice::from_mut(&mut item)).await?;
    Ok(item)
  }

  // ── Physical transfer ─────────────────────

  pub async fn physical_transfer_list_with_names(
    &self,
  ) -> Result<Vec<dtos::PhysicalTransferResponse>, ApiError> {
    let mut responses = self.physical_transfer_list(None).await?;
    for response in &mut responses {
      resolve_names(self.db.as_ref(), &mut response.items).await?;
    }
    Ok(responses)
  }

  pub async fn physical_transfer_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::PhysicalTransferResponse, ApiError> {
    let mut response = self.physical_transfer_get(id).await?;
    resolve_names(self.db.as_ref(), &mut response.items).await?;
    Ok(response)
  }

  pub async fn physical_transfer_composite_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::PhysicalTransferResponse, ApiError> {
    let mut response = self.physical_transfer_composite_get(id).await?;
    resolve_names(self.db.as_ref(), &mut response.items).await?;
    Ok(response)
  }

  pub async fn physical_transfer_composite_list_with_names(
    &self,
  ) -> Result<Vec<dtos::PhysicalTransferResponse>, ApiError> {
    let mut responses = self.physical_transfer_composite_list().await?;
    for response in &mut responses {
      resolve_names(self.db.as_ref(), &mut response.items).await?;
    }
    Ok(responses)
  }

  pub async fn physical_transfer_composite_query_with_names(
    &self,
    document_number: Option<&str>,
    status: Option<enums::DocumentStatus>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<dtos::PhysicalTransferResponse>, ApiError> {
    let mut responses = self
      .physical_transfer_composite_query(document_number, status, page, per_page)
      .await?;
    for response in &mut responses {
      resolve_names(self.db.as_ref(), &mut response.items).await?;
    }
    Ok(responses)
  }

  // ── Ownership transfer ────────────────────

  pub async fn ownership_transfer_list_with_names(
    &self,
  ) -> Result<Vec<dtos::OwnershipTransferResponse>, ApiError> {
    let mut responses = self.ownership_transfer_list(None).await?;
    for response in &mut responses {
      resolve_names(self.db.as_ref(), &mut response.items).await?;
    }
    Ok(responses)
  }

  pub async fn ownership_transfer_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::OwnershipTransferResponse, ApiError> {
    let mut response = self.ownership_transfer_get(id).await?;
    resolve_names(self.db.as_ref(), &mut response.items).await?;
    Ok(response)
  }

  pub async fn ownership_transfer_composite_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::OwnershipTransferResponse, ApiError> {
    let mut response = self.ownership_transfer_composite_get(id).await?;
    resolve_names(self.db.as_ref(), &mut response.items).await?;
    Ok(response)
  }

  pub async fn ownership_transfer_composite_list_with_names(
    &self,
  ) -> Result<Vec<dtos::OwnershipTransferResponse>, ApiError> {
    let mut responses = self.ownership_transfer_composite_list().await?;
    for response in &mut responses {
      resolve_names(self.db.as_ref(), &mut response.items).await?;
    }
    Ok(responses)
  }

  pub async fn ownership_transfer_composite_query_with_names(
    &self,
    status: Option<enums::DocumentStatus>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<dtos::OwnershipTransferResponse>, ApiError> {
    let mut responses = self
      .ownership_transfer_composite_query(status, page, per_page)
      .await?;
    for response in &mut responses {
      resolve_names(self.db.as_ref(), &mut response.items).await?;
    }
    Ok(responses)
  }
}
