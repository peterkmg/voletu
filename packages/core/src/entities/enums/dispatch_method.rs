use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "dispatch_method")]
pub enum DispatchMethod {
  #[sea_orm(string_value = "TRUCK")]
  Truck,
  #[sea_orm(string_value = "VESSEL_TERMINAL")]
  VesselTerminal,
  #[sea_orm(string_value = "BUNKERING")]
  Bunkering,
}
