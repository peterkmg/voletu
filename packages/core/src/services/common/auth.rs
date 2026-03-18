use std::str::FromStr;

use crate::{api::ApiError, enums::RoleType};

pub fn ensure_supervisor_or_higher(role: &str) -> Result<(), ApiError> {
  let parsed = RoleType::from_str(role)
    .map_err(|_| ApiError::Forbidden("Invalid role in token".to_string()))?;
  if !matches!(
    parsed,
    RoleType::Supervisor | RoleType::SeniorSupervisor | RoleType::Admin
  ) {
    return Err(ApiError::Forbidden(
      "Only supervisors and above can execute documents".to_string(),
    ));
  }
  Ok(())
}

pub fn ensure_senior_supervisor_or_higher(role: &str) -> Result<(), ApiError> {
  let parsed = RoleType::from_str(role)
    .map_err(|_| ApiError::Forbidden("Invalid role in token".to_string()))?;
  if !matches!(parsed, RoleType::SeniorSupervisor | RoleType::Admin) {
    return Err(ApiError::Forbidden(
      "Only senior supervisors and admins can revert posted documents".to_string(),
    ));
  }
  Ok(())
}
