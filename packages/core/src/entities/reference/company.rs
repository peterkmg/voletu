use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

use crate::{dtos::CreateCompanyRequest, entities::product};

#[voletu_core_macros::handle_audit]
#[voletu_core_macros::handle_service_fields]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "companies")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub common_name: String,
  pub legal_name: Option<String>,
  pub is_contractor: bool,
  pub is_exporter: bool,
  pub is_manufacturer: bool,
  pub is_sender: bool,
  #[sea_orm(has_many)]
  pub products: HasMany<product::Entity>,
}

impl From<&CreateCompanyRequest> for ActiveModel {
  fn from(dto: &CreateCompanyRequest) -> Self {
    Self {
      common_name: Set(dto.common_name.clone()),
      legal_name: Set(dto.legal_name.clone()),
      is_contractor: Set(dto.is_contractor),
      is_exporter: Set(dto.is_exporter),
      is_manufacturer: Set(dto.is_manufacturer),
      is_sender: Set(dto.is_sender),
      ..Default::default()
    }
  }
}
