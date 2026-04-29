use uuid::{uuid, Uuid};
use voletu_core_macros::enum_type;

mod audit_table;
pub use audit_table::AuditTable;

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
  Executed,
}

#[enum_type]
pub enum LedgerEntrySourceKind {
  OpeningBalance,
  AcceptanceDocument,
  DispatchDocument,
  PhysicalStorageTransfer,
  OwnershipTransfer,
  BlendingDocument,
  InventoryReconciliation,
  ManualAdjustment,
}

#[enum_type]
pub enum LedgerEntrySourceEvent {
  OpeningBalance,
  Execution,
  Reversion,
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
  /// Otherwise, `Draft` maps to `Draft` and `Executed` maps to `Executed`.
  pub fn from_doc_status(status: Option<&DocumentStatus>) -> Self {
    match status {
      None => Self::Pending,
      Some(DocumentStatus::Draft) => Self::Draft,
      Some(DocumentStatus::Executed) => Self::Executed,
    }
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
