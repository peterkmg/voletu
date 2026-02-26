use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "adjustment_type")]
pub enum AdjustmentType {
  #[sea_orm(string_value = "SURPLUS")]
  Surplus,
  #[sea_orm(string_value = "LOSS")]
  Loss,
}
