use sea_orm::entity::prelude::Decimal;
use validator::ValidationError;

use crate::{
  dtos::request::{
    document::{CreateDispatchRequest, CreatePhysicalTransferRequest},
    system::CompleteInitializationRequest,
  },
  enums::{DispatchMethod, DispatchPurpose, NodeType},
};

pub fn validate_non_negative_decimal(value: &Decimal) -> Result<(), ValidationError> {
  if value.is_sign_negative() {
    let mut error = ValidationError::new("non_negative_decimal");
    error.message = Some("must be non-negative".into());
    return Err(error);
  }

  Ok(())
}

pub fn validate_positive_decimal(value: &Decimal) -> Result<(), ValidationError> {
  if value.is_sign_negative() || value.is_zero() {
    let mut error = ValidationError::new("positive_decimal");
    error.message = Some("must be greater than zero".into());
    return Err(error);
  }

  Ok(())
}

pub fn validate_non_blank_string(value: &str) -> Result<(), ValidationError> {
  if value.trim().is_empty() {
    let mut error = ValidationError::new("non_blank_string");
    error.message = Some("must not be blank".into());
    return Err(error);
  }

  Ok(())
}

pub fn validate_dispatch_request(value: &CreateDispatchRequest) -> Result<(), ValidationError> {
  if let (Some(start), Some(end)) = (value.start_cargo_ops, value.end_cargo_ops) {
    if start > end {
      let mut error = ValidationError::new("dispatch_time_window");
      error.message = Some("startCargoOps must be earlier than or equal to endCargoOps".into());
      return Err(error);
    }
  }

  if matches!(value.dispatch_method, DispatchMethod::VesselTerminal) && value.port_id.is_none() {
    let mut error = ValidationError::new("dispatch_port_required");
    error.message = Some("portId is required when dispatchMethod is VESSEL_TERMINAL".into());
    return Err(error);
  }

  if matches!(value.dispatch_method, DispatchMethod::Bunkering) && value.bunker_type.is_none() {
    let mut error = ValidationError::new("dispatch_bunker_type_required");
    error.message = Some("bunkerType is required when dispatchMethod is BUNKERING".into());
    return Err(error);
  }

  if matches!(value.dispatch_purpose, DispatchPurpose::Internal)
    && value.destination_base_id.is_none()
  {
    let mut error = ValidationError::new("dispatch_destination_required");
    error.message = Some("destinationBaseId is required when dispatchPurpose is INTERNAL".into());
    return Err(error);
  }

  Ok(())
}

pub fn validate_physical_transfer_request(
  value: &CreatePhysicalTransferRequest,
) -> Result<(), ValidationError> {
  if value.start_cargo_ops > value.end_cargo_ops {
    let mut error = ValidationError::new("physical_transfer_time_window");
    error.message = Some("startCargoOps must be earlier than or equal to endCargoOps".into());
    return Err(error);
  }

  Ok(())
}

pub fn validate_complete_initialization_request(
  value: &CompleteInitializationRequest,
) -> Result<(), ValidationError> {
  if matches!(value.node_type, Some(NodeType::Peripheral)) && value.central_api_url.is_none() {
    let mut error = ValidationError::new("initialization_peripheral_central_url_required");
    error.message = Some("centralApiUrl is required when nodeType is PERIPHERAL".into());
    return Err(error);
  }

  Ok(())
}
