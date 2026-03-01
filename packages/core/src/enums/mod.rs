use std::str::FromStr;

use uuid::Uuid;
use voletu_core_macros::enum_type;

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
  SoftDelete,
  HardDelete,
}

#[enum_type]
pub enum InitializeAdminAction {
  Replace,
  Delete,
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
  SoftDeleted,
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
      RoleType::Admin => Uuid::from_str("019c8cc2-8913-774a-a432-4dee8eb3f194").unwrap(),
      RoleType::SeniorSupervisor => Uuid::from_str("019c8cc4-3538-7b66-8ce5-6faad856b217").unwrap(),
      RoleType::Supervisor => Uuid::from_str("019c8cc4-9048-7b61-9443-52858a953a17").unwrap(),
      RoleType::Operator => Uuid::from_str("019c8cc4-d965-7f4a-9f9d-c8d299180c6e").unwrap(),
    }
  }
}
