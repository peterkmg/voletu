use uuid::{uuid, Uuid};
use voletu_core_macros::enum_type;

mod audit_table;
pub(crate) use audit_table::AuditTable;

#[enum_type]
pub enum ArrivalType {
  Truck,
  Rail,
  External,
  InitialBalance,
}

#[enum_type]
pub enum DispatchPurpose {
  External,
  Internal,
}

#[enum_type]
pub enum DispatchMethod {
  Truck,
  VesselTerminal,
  Bunkering,
}

#[enum_type]
pub enum BunkerType {
  Export,
  Domestic,
}

#[enum_type]
pub enum AdjustmentType {
  Surplus,
  Loss,
}

#[enum_type]
pub enum SyncDirection {
  Push,
  Pull,
}

#[enum_type]
pub enum AuditAction {
  Insert,
  Update,
  HardDelete,
}

#[enum_type]
pub enum NodeType {
  Central,
  Peripheral,
}

#[enum_type]
pub enum DocumentStatus {
  Draft,
  Posted,
}

/// Pipeline status for flow views.
///
/// Represents the computed lifecycle stage of a document in a flow pipeline:
/// - `Pending` -- basis document exists but no action document yet
/// - `Draft` -- action document exists in draft state
/// - `Executed` -- action document has been posted/executed
#[enum_type]
pub enum PipelineStatus {
  Pending,
  Draft,
  Executed,
}

impl PipelineStatus {
  /// Derive pipeline status from an optional linked document's status.
  ///
  /// If no document exists, the pipeline is `Pending`.
  /// Otherwise, `Draft` maps to `Draft` and `Posted` maps to `Executed`.
  pub fn from_doc_status(status: Option<&DocumentStatus>) -> Self {
    match status {
      None => Self::Pending,
      Some(DocumentStatus::Draft) => Self::Draft,
      Some(DocumentStatus::Posted) => Self::Executed,
    }
  }
}

/// Flow direction category for the cargo flow aggregate view.
#[enum_type]
pub enum FlowType {
  Incoming,
  Outgoing,
  Internal,
}

/// Entity type identifier for cargo flow rows.
#[enum_type]
pub enum FlowEntityType {
  Dispatch,
  Acceptance,
  TruckWaybill,
  RailWaybill,
  Blending,
  PhysicalTransfer,
  OwnershipTransfer,
  Reconciliation,
}

#[enum_type]
pub enum FlowOperation {
  TruckReceipt,
  RailReceipt,
  ExternalAcceptance,
  TransitReceipt,
  TruckDispatch,
  DirectDispatch,
  InternalDispatch,
  Bunkering,
  PhysicalTransfer,
  OwnershipTransfer,
  Blending,
  InventoryReconciliation,
}

impl FlowOperation {
  pub fn flow_type(&self) -> FlowType {
    match self {
      Self::TruckReceipt | Self::RailReceipt | Self::ExternalAcceptance | Self::TransitReceipt => FlowType::Incoming,
      Self::TruckDispatch | Self::DirectDispatch | Self::InternalDispatch | Self::Bunkering => FlowType::Outgoing,
      Self::PhysicalTransfer | Self::OwnershipTransfer | Self::Blending | Self::InventoryReconciliation => FlowType::Internal,
    }
  }

  pub fn entity_type(&self) -> FlowEntityType {
    match self {
      Self::TruckReceipt => FlowEntityType::TruckWaybill,
      Self::RailReceipt => FlowEntityType::RailWaybill,
      Self::ExternalAcceptance | Self::TransitReceipt => FlowEntityType::Acceptance,
      Self::TruckDispatch | Self::DirectDispatch | Self::InternalDispatch | Self::Bunkering => FlowEntityType::Dispatch,
      Self::PhysicalTransfer => FlowEntityType::PhysicalTransfer,
      Self::OwnershipTransfer => FlowEntityType::OwnershipTransfer,
      Self::Blending => FlowEntityType::Blending,
      Self::InventoryReconciliation => FlowEntityType::Reconciliation,
    }
  }

  pub fn route(&self) -> &'static str {
    match self {
      Self::TruckReceipt => "/incoming/truck",
      Self::RailReceipt => "/incoming/rail",
      Self::ExternalAcceptance | Self::TransitReceipt => "/incoming/external",
      Self::TruckDispatch => "/outgoing/truck",
      Self::DirectDispatch | Self::InternalDispatch => "/outgoing/direct",
      Self::Bunkering => "/outgoing/bunkering",
      Self::PhysicalTransfer => "/internal/physical-transfer",
      Self::OwnershipTransfer => "/internal/ownership-transfer",
      Self::Blending => "/internal/blending",
      Self::InventoryReconciliation => "/internal/reconciliation",
    }
  }

  pub fn from_dispatch(method: DispatchMethod, purpose: DispatchPurpose) -> Self {
    match (method, purpose) {
      (DispatchMethod::Bunkering, _) => Self::Bunkering,
      (DispatchMethod::VesselTerminal, _) => Self::DirectDispatch,
      (DispatchMethod::Truck, DispatchPurpose::Internal) => Self::InternalDispatch,
      (DispatchMethod::Truck, DispatchPurpose::External) => Self::TruckDispatch,
    }
  }

  pub fn matches_filter(&self, flow_type: Option<FlowType>, operation: Option<&Self>) -> bool {
    if let Some(ft) = flow_type {
      if self.flow_type() != ft { return false; }
    }
    if let Some(op) = operation {
      if self != op { return false; }
    }
    true
  }
}

#[enum_type]
pub enum RoleType {
  Admin,
  SeniorSupervisor,
  Supervisor,
  Operator,
}

impl RoleType {
  pub fn uuid(&self) -> Uuid {
    match self {
      RoleType::Admin => uuid!("019c8cc2-8913-774a-a432-4dee8eb3f194"),
      RoleType::SeniorSupervisor => uuid!("019c8cc4-3538-7b66-8ce5-6faad856b217"),
      RoleType::Supervisor => uuid!("019c8cc4-9048-7b61-9443-52858a953a17"),
      RoleType::Operator => uuid!("019c8cc4-d965-7f4a-9f9d-c8d299180c6e"),
    }
  }
}
