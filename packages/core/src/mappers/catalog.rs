use crate::{
  dtos::{
    BaseResponse,
    CompanyResponse,
    PortResponse,
    ProductGroupResponse,
    ProductResponse,
    ProductTypeResponse,
    StorageResponse,
    WarehouseResponse,
  },
  entities::{base, company, port, product, product_group, product_type, storage, warehouse},
};

pub fn map_company(row: company::Model) -> CompanyResponse {
  CompanyResponse {
    id: row.id,
    common_name: row.common_name,
    legal_name: row.legal_name,
    is_contractor: row.is_contractor,
    is_exporter: row.is_exporter,
    is_manufacturer: row.is_manufacturer,
    is_sender: row.is_sender,
  }
}

pub fn map_product_type(row: product_type::Model) -> ProductTypeResponse {
  ProductTypeResponse {
    id: row.id,
    common_name: row.common_name,
    long_name: row.long_name,
  }
}

pub fn map_product_group(row: product_group::Model) -> ProductGroupResponse {
  ProductGroupResponse {
    id: row.id,
    product_type_id: row.product_type_id,
    common_name: row.common_name,
    long_name: row.long_name,
  }
}

pub fn map_product(row: product::Model) -> ProductResponse {
  ProductResponse {
    id: row.id,
    product_group_id: row.product_group_id,
    manufacturer_id: row.manufacturer_id,
    common_name: row.common_name,
    long_name: row.long_name,
    add_identification: row.add_identification,
    is_component: row.is_component,
  }
}

pub fn map_base(row: base::Model) -> BaseResponse {
  BaseResponse {
    id: row.id,
    common_name: row.common_name,
    long_name: row.long_name,
  }
}

pub fn map_warehouse(row: warehouse::Model) -> WarehouseResponse {
  WarehouseResponse {
    id: row.id,
    base_id: row.base_id,
    common_name: row.common_name,
    long_name: row.long_name,
  }
}

pub fn map_storage(row: storage::Model) -> StorageResponse {
  StorageResponse {
    id: row.id,
    warehouse_id: row.warehouse_id,
    common_name: row.common_name,
    long_name: row.long_name,
    capacity: row.capacity,
    is_type_specific: row.is_type_specific,
    product_type_id: row.product_type_id,
  }
}

pub fn map_port(row: port::Model) -> PortResponse {
  PortResponse {
    id: row.id,
    common_name: row.common_name,
    country: row.country,
  }
}
