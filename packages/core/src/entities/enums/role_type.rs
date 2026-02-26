use std::str::FromStr;

use anyhow::anyhow;
use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "role_type")]
pub enum RoleType {
  #[sea_orm(string_value = "admin")]
  Admin,
  #[sea_orm(string_value = "senior_supervisor")]
  SeniorSupervisor,
  #[sea_orm(string_value = "supervisor")]
  Supervisor,
  #[sea_orm(string_value = "operator")]
  Operator,
}

impl RoleType {
  pub fn all() -> &'static [RoleType] {
    &[
      RoleType::Admin,
      RoleType::SeniorSupervisor,
      RoleType::Supervisor,
      RoleType::Operator,
    ]
  }

  pub fn as_str(&self) -> &str {
    match self {
      RoleType::Admin => "admin",
      RoleType::SeniorSupervisor => "senior_supervisor",
      RoleType::Supervisor => "supervisor",
      RoleType::Operator => "operator",
    }
  }

  pub fn from_str(s: &str) -> anyhow::Result<Self> {
    match s {
      "admin" => Ok(RoleType::Admin),
      "senior_supervisor" => Ok(RoleType::SeniorSupervisor),
      "supervisor" => Ok(RoleType::Supervisor),
      "operator" => Ok(RoleType::Operator),
      _ => Err(anyhow!("Invalid role type: {}", s)),
    }
  }

  pub fn uuid(&self) -> Uuid {
    match self {
      RoleType::Admin => Uuid::from_str("019c8cc2-8913-774a-a432-4dee8eb3f194").unwrap(),
      RoleType::SeniorSupervisor => Uuid::from_str("019c8cc4-3538-7b66-8ce5-6faad856b217").unwrap(),
      RoleType::Supervisor => Uuid::from_str("019c8cc4-9048-7b61-9443-52858a953a17").unwrap(),
      RoleType::Operator => Uuid::from_str("019c8cc4-d965-7f4a-9f9d-c8d299180c6e").unwrap(),
    }
  }
}
