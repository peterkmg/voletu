use anyhow::anyhow;

use crate::{dtos::UserResponse, entities::system::user::ModelEx};

pub fn map_user_response(model: &ModelEx) -> anyhow::Result<UserResponse> {
  let role = model.role.as_ref().ok_or(anyhow!("User role not found"))?;

  Ok(UserResponse {
    id: model.id,
    username: model.username.clone(),
    fullname: model.fullname.clone(),
    role: role.common_name.to_string(),
  })
}
