use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "database_node_type")]
pub enum NodeType {
  #[sea_orm(string_value = "CENTRAL")]
  Central,
  #[sea_orm(string_value = "PERIPHERAL")]
  Peripheral,
}

impl NodeType {
  pub fn as_str(&self) -> &'static str {
    match self {
      Self::Central => "CENTRAL",
      Self::Peripheral => "PERIPHERAL",
    }
  }
}
