use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "bunker_type")]
pub enum BunkerType {
  #[sea_orm(string_value = "EXPORT")]
  Export,
  #[sea_orm(string_value = "DOMESTIC")]
  Domestic,
}
