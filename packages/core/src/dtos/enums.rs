use voletu_core_macros::enum_dto;

#[enum_dto]
pub enum ArrivalType {
  Truck,
  Rail,
  External,
  InitialBalance,
}

#[enum_dto]
pub enum DispatchPurpose {
  External,
  Internal,
}

#[enum_dto]
pub enum DispatchMethod {
  Truck,
  VesselTerminal,
  Bunkering,
}

#[enum_dto]
pub enum BunkerType {
  Export,
  Domestic,
}

#[enum_dto]
pub enum AdjustmentType {
  Surplus,
  Loss,
}

#[enum_dto]
pub enum SyncDirection {
  Push,
  Pull,
}

#[enum_dto]
pub enum InitializeAdminAction {
  Replace,
  Delete,
}

#[enum_dto]
pub enum NodeType {
  Central,
  Peripheral,
}
