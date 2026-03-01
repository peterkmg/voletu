use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

use crate::dtos::CreatePortRequest;

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "ports")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub common_name: String,
  pub country: Option<String>,
}

impl From<&CreatePortRequest> for ActiveModel {
  fn from(dto: &CreatePortRequest) -> Self {
    Self {
      common_name: Set(dto.common_name.clone()),
      country: Set(dto.country.clone()),
      ..Default::default()
    }
  }
}
