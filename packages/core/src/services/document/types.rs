use crate::{
  dtos::response::document::{
    AcceptanceFlatRow,
    DispatchFlatRow,
    OwnershipTransferFlatRow,
    PhysicalTransferFlatRow,
  },
  entities::{
    acceptance_document,
    acceptance_item,
    dispatch_document,
    dispatch_item,
    ownership_transfer,
    ownership_transfer_item,
    physical_storage_transfer,
    physical_transfer_item,
  },
};

const EMPTY_LABEL: &str = "\u{2014}";

pub(crate) struct AcceptanceFlatRowRef<'a> {
  pub(crate) document: &'a acceptance_document::ModelEx,
  pub(crate) item: Option<&'a acceptance_item::ModelEx>,
}

impl From<AcceptanceFlatRowRef<'_>> for AcceptanceFlatRow {
  fn from(value: AcceptanceFlatRowRef<'_>) -> Self {
    let contractor_id_name = value
      .document
      .contractor
      .as_ref()
      .map(|contractor| contractor.common_name.clone())
      .unwrap_or_else(|| EMPTY_LABEL.to_string());

    match value.item {
      Some(item) => Self {
        id: value.document.id,
        document_id: value.document.id,
        document_number: value.document.document_number.clone(),
        date_accepted: value.document.date_accepted.to_string(),
        status: value.document.status,
        source_entity: value.document.source_entity.clone(),
        item_id: item.id,
        product_id_name: item
          .product
          .as_ref()
          .map(|product| product.common_name.clone())
          .unwrap_or_default(),
        storage_id_name: item
          .storage
          .as_ref()
          .map(|storage| storage.common_name.clone())
          .unwrap_or_default(),
        contractor_id_name,
        accepted_amount: item.accepted_amount,
      },
      None => Self {
        id: value.document.id,
        document_id: value.document.id,
        document_number: value.document.document_number.clone(),
        date_accepted: value.document.date_accepted.to_string(),
        status: value.document.status,
        source_entity: value.document.source_entity.clone(),
        item_id: value.document.id,
        product_id_name: EMPTY_LABEL.to_string(),
        storage_id_name: EMPTY_LABEL.to_string(),
        contractor_id_name,
        accepted_amount: Default::default(),
      },
    }
  }
}

pub(crate) struct DispatchFlatRowRef<'a> {
  pub(crate) document: &'a dispatch_document::ModelEx,
  pub(crate) item: Option<&'a dispatch_item::ModelEx>,
}

impl From<DispatchFlatRowRef<'_>> for DispatchFlatRow {
  fn from(value: DispatchFlatRowRef<'_>) -> Self {
    let contractor_id_name = value
      .document
      .contractor
      .as_ref()
      .map(|contractor| contractor.common_name.clone())
      .unwrap_or_else(|| EMPTY_LABEL.to_string());

    match value.item {
      Some(item) => Self {
        id: value.document.id,
        document_id: value.document.id,
        document_number: value.document.document_number.clone(),
        date: value.document.date.to_string(),
        status: value.document.status,
        dispatch_method: value.document.dispatch_method,
        dispatch_purpose: value.document.dispatch_purpose,
        contractor_id_name,
        item_id: item.id,
        product_id_name: item
          .product
          .as_ref()
          .map(|product| product.common_name.clone())
          .unwrap_or_default(),
        storage_id_name: item
          .storage
          .as_ref()
          .map(|storage| storage.common_name.clone())
          .unwrap_or_default(),
        dispatched_amount: item.dispatched_amount,
      },
      None => Self {
        id: value.document.id,
        document_id: value.document.id,
        document_number: value.document.document_number.clone(),
        date: value.document.date.to_string(),
        status: value.document.status,
        dispatch_method: value.document.dispatch_method,
        dispatch_purpose: value.document.dispatch_purpose,
        contractor_id_name,
        item_id: value.document.id,
        product_id_name: EMPTY_LABEL.to_string(),
        storage_id_name: EMPTY_LABEL.to_string(),
        dispatched_amount: Default::default(),
      },
    }
  }
}

pub(crate) struct PhysicalTransferFlatRowRef<'a> {
  pub(crate) document: &'a physical_storage_transfer::ModelEx,
  pub(crate) item: Option<&'a physical_transfer_item::ModelEx>,
  pub(crate) contractor_id_name: &'a str,
  pub(crate) to_storage_id_name: &'a str,
}

impl From<PhysicalTransferFlatRowRef<'_>> for PhysicalTransferFlatRow {
  fn from(value: PhysicalTransferFlatRowRef<'_>) -> Self {
    match value.item {
      Some(item) => Self {
        id: value.document.id,
        document_id: value.document.id,
        document_number: value.document.document_number.clone(),
        date: value.document.date.to_string(),
        status: value.document.status,
        contractor_id_name: value.contractor_id_name.to_string(),
        item_id: item.id,
        product_id_name: item
          .product
          .as_ref()
          .map(|product| product.common_name.clone())
          .unwrap_or_default(),
        from_storage_id_name: item
          .from_storage
          .as_ref()
          .map(|storage| storage.common_name.clone())
          .unwrap_or_default(),
        to_storage_id_name: value.to_storage_id_name.to_string(),
        amount: item.amount,
      },
      None => Self {
        id: value.document.id,
        document_id: value.document.id,
        document_number: value.document.document_number.clone(),
        date: value.document.date.to_string(),
        status: value.document.status,
        contractor_id_name: value.contractor_id_name.to_string(),
        item_id: value.document.id,
        product_id_name: EMPTY_LABEL.to_string(),
        from_storage_id_name: EMPTY_LABEL.to_string(),
        to_storage_id_name: EMPTY_LABEL.to_string(),
        amount: Default::default(),
      },
    }
  }
}

pub(crate) struct OwnershipTransferFlatRowRef<'a> {
  pub(crate) document: &'a ownership_transfer::ModelEx,
  pub(crate) item: Option<&'a ownership_transfer_item::ModelEx>,
  pub(crate) from_contractor_id_name: &'a str,
  pub(crate) to_contractor_id_name: &'a str,
}

impl From<OwnershipTransferFlatRowRef<'_>> for OwnershipTransferFlatRow {
  fn from(value: OwnershipTransferFlatRowRef<'_>) -> Self {
    match value.item {
      Some(item) => Self {
        id: value.document.id,
        document_id: value.document.id,
        date: value.document.date.to_string(),
        status: value.document.status,
        item_id: item.id,
        product_id_name: item
          .product
          .as_ref()
          .map(|product| product.common_name.clone())
          .unwrap_or_default(),
        storage_id_name: item
          .storage
          .as_ref()
          .map(|storage| storage.common_name.clone())
          .unwrap_or_default(),
        from_contractor_id_name: value.from_contractor_id_name.to_string(),
        to_contractor_id_name: value.to_contractor_id_name.to_string(),
        amount: item.amount,
      },
      None => Self {
        id: value.document.id,
        document_id: value.document.id,
        date: value.document.date.to_string(),
        status: value.document.status,
        item_id: value.document.id,
        product_id_name: EMPTY_LABEL.to_string(),
        storage_id_name: EMPTY_LABEL.to_string(),
        from_contractor_id_name: EMPTY_LABEL.to_string(),
        to_contractor_id_name: EMPTY_LABEL.to_string(),
        amount: Default::default(),
      },
    }
  }
}
