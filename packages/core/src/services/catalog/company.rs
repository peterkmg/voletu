use crate::{
  dtos,
  entities::company,
  services::{
    common::{set_if_some, set_if_some_mapped},
    CatalogService,
  },
};
fn apply_company_update(model: &mut company::ActiveModel, req: &dtos::UpdateCompanyRequest) {
  set_if_some(&mut model.common_name, req.common_name.clone());
  set_if_some_mapped(&mut model.legal_name, req.legal_name.clone(), Some);
  set_if_some(&mut model.is_contractor, req.is_contractor);
  set_if_some(&mut model.is_exporter, req.is_exporter);
  set_if_some(&mut model.is_manufacturer, req.is_manufacturer);
  set_if_some(&mut model.is_sender, req.is_sender);
}

#[voletu_core_macros::entity_service(
  entity_name = "Company",
  entity = company,
  entity_mod = company,
  create_req = dtos::CreateCompanyRequest,
  update_req = dtos::UpdateCompanyRequest,
  response = dtos::CompanyResponse,
  apply_update = apply_company_update,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl CatalogService {}
