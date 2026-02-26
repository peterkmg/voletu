use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "sync_direction")]
pub enum SyncDirection {
  #[sea_orm(string_value = "PUSH")]
  Push,
  #[sea_orm(string_value = "PULL")]
  Pull,
}
