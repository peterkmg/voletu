use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "audit_action")]
pub enum AuditAction {
  #[sea_orm(string_value = "INSERT")]
  Insert,
  #[sea_orm(string_value = "UPDATE")]
  Update,
  #[sea_orm(string_value = "SOFT_DELETE")]
  SoftDelete,
  #[sea_orm(string_value = "HARD_DELETE")]
  HardDelete,
}
