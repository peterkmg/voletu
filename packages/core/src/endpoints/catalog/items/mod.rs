use std::sync::Arc;

use axum::{
  extract::{Path, Query, State},
  Json,
};
use axum_valid::Valid;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::{
    CompanyResponse, CreateCompanyRequest, CreateProductGroupRequest, CreateProductRequest,
    CreateProductTypeRequest, ProductGroupResponse, ProductResponse, ProductTypeResponse,
    UpdateCompanyRequest, UpdateProductGroupRequest, UpdateProductRequest,
    UpdateProductTypeRequest,
  },
  endpoints::{
    paths,
    query::{EmbedParams, PaginationParams},
  },
};

mod company;
mod product;
mod product_group;
mod product_type;

pub fn catalog_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .merge(company::company_routes(state.clone()))
    .merge(product_type::product_type_routes(state.clone()))
    .merge(product_group::product_group_routes(state.clone()))
    .merge(product::product_routes(state))
}
