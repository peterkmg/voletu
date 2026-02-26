use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "arrival_type")]
pub enum ArrivalType {
  #[sea_orm(string_value = "TRUCK")]
  Truck,
  #[sea_orm(string_value = "RAIL")]
  Rail,
  #[sea_orm(string_value = "EXTERNAL_VESSEL")]
  ExternalVessel,
  #[sea_orm(string_value = "EXTERNAL_TERMINAL")]
  ExternalTerminal,
  #[sea_orm(string_value = "INITIAL_BALANCE")]
  InitialBalance,
}
