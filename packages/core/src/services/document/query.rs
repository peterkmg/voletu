use uuid::Uuid;

use crate::{dtos::request::query::NullableFilter, enums};

#[derive(Debug, Clone, Default)]
pub struct AcceptanceDocumentQuerySpec {
  pub document_number: Option<String>,
  pub status: Option<enums::DocumentStatus>,
  pub truck_waybill_id: Option<NullableFilter>,
  pub rail_waybill_id: Option<NullableFilter>,
  pub transit_dispatch_id: Option<NullableFilter>,
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

impl AcceptanceDocumentQuerySpec {
  pub fn list(page: Option<u64>, per_page: Option<u64>) -> Self {
    Self {
      page,
      per_page,
      ..Self::default()
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct AcceptanceFlatQuerySpec {
  pub status: Option<enums::DocumentStatus>,
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

impl AcceptanceFlatQuerySpec {
  pub fn list(page: Option<u64>, per_page: Option<u64>) -> Self {
    Self {
      page,
      per_page,
      ..Self::default()
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct DispatchDocumentQuerySpec {
  pub document_number: Option<String>,
  pub status: Option<enums::DocumentStatus>,
  pub contractor_id: Option<Uuid>,
  pub dispatch_method: Option<enums::DispatchMethod>,
  pub dispatch_purpose: Option<enums::DispatchPurpose>,
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

impl DispatchDocumentQuerySpec {
  pub fn list(page: Option<u64>, per_page: Option<u64>) -> Self {
    Self {
      page,
      per_page,
      ..Self::default()
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct DispatchFlatQuerySpec {
  pub status: Option<enums::DocumentStatus>,
  pub dispatch_method: Option<enums::DispatchMethod>,
  pub dispatch_purpose: Option<enums::DispatchPurpose>,
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

impl DispatchFlatQuerySpec {
  pub fn list(page: Option<u64>, per_page: Option<u64>) -> Self {
    Self {
      page,
      per_page,
      ..Self::default()
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct TruckDispatchPipelineQuerySpec {
  pub pipeline_status: Option<enums::PipelineStatus>,
  pub contractor_id: Option<Uuid>,
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

impl TruckDispatchPipelineQuerySpec {
  pub fn list(page: Option<u64>, per_page: Option<u64>) -> Self {
    Self {
      page,
      per_page,
      ..Self::default()
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct BlendingDocumentQuerySpec {
  pub document_number: Option<String>,
  pub status: Option<enums::DocumentStatus>,
  pub contractor_id: Option<Uuid>,
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

impl BlendingDocumentQuerySpec {
  pub fn list(page: Option<u64>, per_page: Option<u64>) -> Self {
    Self {
      page,
      per_page,
      ..Self::default()
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct BlendingFlatQuerySpec {
  pub status: Option<enums::DocumentStatus>,
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

impl BlendingFlatQuerySpec {
  pub fn list(page: Option<u64>, per_page: Option<u64>) -> Self {
    Self {
      page,
      per_page,
      ..Self::default()
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct ReconciliationQuerySpec {
  pub document_number: Option<String>,
  pub status: Option<enums::DocumentStatus>,
  pub warehouse_id: Option<Uuid>,
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

impl ReconciliationQuerySpec {
  pub fn list(page: Option<u64>, per_page: Option<u64>) -> Self {
    Self {
      page,
      per_page,
      ..Self::default()
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct ReconciliationFlatQuerySpec {
  pub status: Option<enums::DocumentStatus>,
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

impl ReconciliationFlatQuerySpec {
  pub fn list(page: Option<u64>, per_page: Option<u64>) -> Self {
    Self {
      page,
      per_page,
      ..Self::default()
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct TruckWaybillQuerySpec {
  pub document_number: Option<String>,
  pub sender_id: Option<Uuid>,
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

impl TruckWaybillQuerySpec {
  pub fn list(page: Option<u64>, per_page: Option<u64>) -> Self {
    Self {
      page,
      per_page,
      ..Self::default()
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct TruckReceiptPipelineQuerySpec {
  pub pipeline_status: Option<enums::PipelineStatus>,
  pub contractor_id: Option<Uuid>,
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

impl TruckReceiptPipelineQuerySpec {
  pub fn list(page: Option<u64>, per_page: Option<u64>) -> Self {
    Self {
      page,
      per_page,
      ..Self::default()
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct RailWaybillQuerySpec {
  pub document_number: Option<String>,
  pub sender_id: Option<Uuid>,
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

impl RailWaybillQuerySpec {
  pub fn list(page: Option<u64>, per_page: Option<u64>) -> Self {
    Self {
      page,
      per_page,
      ..Self::default()
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct RailReceiptPipelineQuerySpec {
  pub pipeline_status: Option<enums::PipelineStatus>,
  pub contractor_id: Option<Uuid>,
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

impl RailReceiptPipelineQuerySpec {
  pub fn list(page: Option<u64>, per_page: Option<u64>) -> Self {
    Self {
      page,
      per_page,
      ..Self::default()
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct PhysicalTransferQuerySpec {
  pub document_number: Option<String>,
  pub status: Option<enums::DocumentStatus>,
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

impl PhysicalTransferQuerySpec {
  pub fn list(page: Option<u64>, per_page: Option<u64>) -> Self {
    Self {
      page,
      per_page,
      ..Self::default()
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct PhysicalTransferFlatQuerySpec {
  pub status: Option<enums::DocumentStatus>,
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

impl PhysicalTransferFlatQuerySpec {
  pub fn list(page: Option<u64>, per_page: Option<u64>) -> Self {
    Self {
      page,
      per_page,
      ..Self::default()
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct OwnershipTransferQuerySpec {
  pub status: Option<enums::DocumentStatus>,
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

impl OwnershipTransferQuerySpec {
  pub fn list(page: Option<u64>, per_page: Option<u64>) -> Self {
    Self {
      page,
      per_page,
      ..Self::default()
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct OwnershipTransferFlatQuerySpec {
  pub status: Option<enums::DocumentStatus>,
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

impl OwnershipTransferFlatQuerySpec {
  pub fn list(page: Option<u64>, per_page: Option<u64>) -> Self {
    Self {
      page,
      per_page,
      ..Self::default()
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct CargoFlowQuerySpec {
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

impl CargoFlowQuerySpec {
  pub fn list(page: Option<u64>, per_page: Option<u64>) -> Self {
    Self { page, per_page }
  }
}
