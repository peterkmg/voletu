use crate::{
  dtos::{
    BlendingComponentResponse,
    BlendingResponse,
    BlendingResultResponse,
    InventoryAdjustmentResponse,
    InventoryReconciliationResponse,
    OwnershipTransferResponse,
    PhysicalTransferResponse,
  },
  entities::{
    blending_component,
    blending_document,
    blending_result,
    inventory_adjustment,
    inventory_reconciliation,
    ownership_transfer,
    physical_storage_transfer,
  },
};

pub fn map_blending_document(row: blending_document::Model) -> BlendingResponse {
  BlendingResponse {
    id: row.id,
    document_number: row.document_number,
    date: row.date.to_rfc3339(),
    contractor_id: row.contractor_id,
    target_product_id: row.target_product_id,
  }
}

pub fn map_blending_component(row: blending_component::Model) -> BlendingComponentResponse {
  BlendingComponentResponse {
    id: row.id,
    blending_doc_id: row.blending_doc_id,
    storage_id: row.storage_id,
    source_product_id: row.source_product_id,
    amount_used: row.amount_used,
  }
}

pub fn map_blending_result(row: blending_result::Model) -> BlendingResultResponse {
  BlendingResultResponse {
    id: row.id,
    blending_doc_id: row.blending_doc_id,
    storage_id: row.storage_id,
    produced_amount: row.produced_amount,
  }
}

pub fn map_ownership_transfer(row: ownership_transfer::Model) -> OwnershipTransferResponse {
  OwnershipTransferResponse {
    id: row.id,
    date: row.date.to_rfc3339(),
    storage_id: row.storage_id,
    product_id: row.product_id,
    from_contractor_id: row.from_contractor_id,
    to_contractor_id: row.to_contractor_id,
    amount_transferred: row.amount_transferred,
  }
}

pub fn map_physical_transfer(row: physical_storage_transfer::Model) -> PhysicalTransferResponse {
  PhysicalTransferResponse {
    id: row.id,
    document_number: row.document_number,
    date: row.date.to_rfc3339(),
    start_cargo_ops: row.start_cargo_ops.to_rfc3339(),
    end_cargo_ops: row.end_cargo_ops.to_rfc3339(),
    contractor_id: row.contractor_id,
    product_id: row.product_id,
    from_storage_id: row.from_storage_id,
    to_storage_id: row.to_storage_id,
    amount_transferred: row.amount_transferred,
  }
}

pub fn map_reconciliation(row: inventory_reconciliation::Model) -> InventoryReconciliationResponse {
  InventoryReconciliationResponse {
    id: row.id,
    document_number: row.document_number,
    date: row.date.to_rfc3339(),
    warehouse_id: row.warehouse_id,
  }
}

pub fn map_adjustment(row: inventory_adjustment::Model) -> InventoryAdjustmentResponse {
  InventoryAdjustmentResponse {
    id: row.id,
    reconciliation_id: row.reconciliation_id,
    storage_id: row.storage_id,
    product_id: row.product_id,
    contractor_id: row.contractor_id,
    adjustment_type: row.adjustment_type,
    amount: row.amount,
    reason: row.reason,
  }
}
