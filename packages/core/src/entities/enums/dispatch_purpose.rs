use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "dispatch_purpose")]
pub enum DispatchPurpose {
  #[sea_orm(string_value = "EXTERNAL_COMMERCIAL")]
  ExternalCommercial,
  #[sea_orm(string_value = "INTERNAL_TRANSIT")]
  InternalTransit,
}
