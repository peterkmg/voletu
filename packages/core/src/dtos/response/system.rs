use uuid::Uuid;
use voletu_core_macros::response_dto;

use crate::entities::{
  node_base_assignment,
  system::{database_instance, local, refresh_token, role, user::ModelEx},
};

/// Response DTO for the `user` entity.
#[response_dto(service_fields(common))]
pub struct UserResponse {
  #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
  pub id: Uuid,
  #[schema(example = "johndoe")]
  pub username: String,
  #[schema(example = "John Doe")]
  pub fullname: Option<String>,
  #[schema(example = "ADMIN")]
  pub role: String,
}

/// Functional DTO returned by authentication login endpoints.
#[response_dto]
pub struct LoginResponse {
  pub access_token: String,
  pub refresh_token: String,
  pub user: UserResponse,
}

/// Response DTO for the `role` entity.
#[response_dto]
pub struct RoleResponse {
  pub id: Uuid,
  pub common_name: String,
}

/// Response DTO for the `local` entity.
#[response_dto]
pub struct LocalResponse {
  pub id: i32,
  pub is_initialized: bool,
  pub local_db_id: Uuid,
  pub central_api_url: Option<String>,
}

/// Response DTO for the `database_instance` entity.
#[response_dto(service_fields(common))]
pub struct DatabaseInstanceResponse {
  pub id: Uuid,
  pub common_name: String,
  pub node_type: String,
  pub base_id: Option<Uuid>,
}

/// Response DTO for the `refresh_token` entity.
#[response_dto(service_fields(created_at, updated_at))]
pub struct RefreshTokenResponse {
  pub id: Uuid,
  pub user_id: Uuid,
  pub expires_at: String,
  pub is_revoked: bool,
  pub device_info: Option<String>,
}

/// Functional DTO describing node/base assignment configuration.
#[response_dto]
pub struct BaseAssignmentResponse {
  pub id: Uuid,
  pub node_id: Uuid,
  pub base_id: Uuid,
}

/// Functional DTO used by command-style endpoints that only return a status
/// message.
#[response_dto]
pub struct OperationMessageResponse {
  pub message: String,
}

/// Functional DTO returned by the health endpoint.
#[response_dto]
pub struct HealthData {
  pub status: String,
  pub is_initialized: bool,
  pub node_type: String,
  pub node_name: String,
}

/// Functional DTO returned by the node status endpoint.
#[response_dto]
pub struct NodeStatusResponse {
  pub is_initialized: bool,
  pub node_type: String,
  pub node_name: String,
  pub worker_state: String,
  pub last_sync_at: Option<String>,
  pub central_api_url: Option<String>,
}

impl TryFrom<&ModelEx> for UserResponse {
  type Error = anyhow::Error;

  fn try_from(model: &ModelEx) -> Result<Self, Self::Error> {
    let role = model
      .role
      .as_ref()
      .ok_or_else(|| anyhow::anyhow!("User role not found"))?;

    Ok(Self {
      id: model.id,
      username: model.username.clone(),
      fullname: model.fullname.clone(),
      role: role.common_name.to_string(),
      created_at: model.created_at.to_rfc3339(),
      updated_at: model.updated_at.to_rfc3339(),
      deleted_at: model.deleted_at.map(|v| v.to_rfc3339()),
      created_by: model.created_by,
      updated_by: model.updated_by,
      deleted_by: model.deleted_by,
      origin_db_id: model.origin_db_id,
    })
  }
}

impl From<&role::Model> for RoleResponse {
  fn from(model: &role::Model) -> Self {
    Self {
      id: model.id,
      common_name: model.common_name.to_string(),
    }
  }
}

impl From<&role::ModelEx> for RoleResponse {
  fn from(model: &role::ModelEx) -> Self {
    Self {
      id: model.id,
      common_name: model.common_name.to_string(),
    }
  }
}

impl From<&local::Model> for LocalResponse {
  fn from(model: &local::Model) -> Self {
    Self {
      id: model.id,
      is_initialized: model.is_initialized,
      local_db_id: model.local_db_id,
      central_api_url: model.central_api_url.clone(),
    }
  }
}

impl From<&local::ModelEx> for LocalResponse {
  fn from(model: &local::ModelEx) -> Self {
    Self {
      id: model.id,
      is_initialized: model.is_initialized,
      local_db_id: model.local_db_id,
      central_api_url: model.central_api_url.clone(),
    }
  }
}

impl From<&database_instance::Model> for DatabaseInstanceResponse {
  fn from(model: &database_instance::Model) -> Self {
    Self {
      id: model.id,
      common_name: model.common_name.clone(),
      node_type: model.node_type.to_string(),
      base_id: model.base_id,
      created_at: model.created_at.to_rfc3339(),
      updated_at: model.updated_at.to_rfc3339(),
      deleted_at: model.deleted_at.map(|v| v.to_rfc3339()),
      created_by: model.created_by,
      updated_by: model.updated_by,
      deleted_by: model.deleted_by,
      origin_db_id: model.origin_db_id,
    }
  }
}

impl From<&database_instance::ModelEx> for DatabaseInstanceResponse {
  fn from(model: &database_instance::ModelEx) -> Self {
    Self {
      id: model.id,
      common_name: model.common_name.clone(),
      node_type: model.node_type.to_string(),
      base_id: model.base_id,
      created_at: model.created_at.to_rfc3339(),
      updated_at: model.updated_at.to_rfc3339(),
      deleted_at: model.deleted_at.map(|v| v.to_rfc3339()),
      created_by: model.created_by,
      updated_by: model.updated_by,
      deleted_by: model.deleted_by,
      origin_db_id: model.origin_db_id,
    }
  }
}

impl From<&refresh_token::Model> for RefreshTokenResponse {
  fn from(model: &refresh_token::Model) -> Self {
    Self {
      id: model.id,
      user_id: model.user_id,
      expires_at: model.expires_at.to_rfc3339(),
      is_revoked: model.is_revoked,
      device_info: model.device_info.clone(),
      created_at: model.created_at.to_rfc3339(),
      updated_at: model.updated_at.to_rfc3339(),
    }
  }
}

impl From<&refresh_token::ModelEx> for RefreshTokenResponse {
  fn from(model: &refresh_token::ModelEx) -> Self {
    Self {
      id: model.id,
      user_id: model.user_id,
      expires_at: model.expires_at.to_rfc3339(),
      is_revoked: model.is_revoked,
      device_info: model.device_info.clone(),
      created_at: model.created_at.to_rfc3339(),
      updated_at: model.updated_at.to_rfc3339(),
    }
  }
}

impl From<node_base_assignment::Model> for BaseAssignmentResponse {
  fn from(model: node_base_assignment::Model) -> Self {
    Self {
      id: model.id,
      node_id: model.node_id,
      base_id: model.base_id,
    }
  }
}

impl From<&node_base_assignment::ModelEx> for BaseAssignmentResponse {
  fn from(model: &node_base_assignment::ModelEx) -> Self {
    Self {
      id: model.id,
      node_id: model.node_id,
      base_id: model.base_id,
    }
  }
}
