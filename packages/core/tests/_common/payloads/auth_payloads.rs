use serde_json::json;

pub fn auth_login(username: &str, password: &str) -> String {
  json!({
    "username": username,
    "password": password,
  })
  .to_string()
}

pub fn auth_refresh(refresh_token: &str) -> String {
  json!({
    "refreshToken": refresh_token,
  })
  .to_string()
}

pub fn auth_change_password(username: &str, current_password: &str, new_password: &str) -> String {
  json!({
    "username": username,
    "currentPassword": current_password,
    "newPassword": new_password,
  })
  .to_string()
}

pub fn user_create(username: &str, password: &str, fullname: &str, role_name: &str) -> String {
  json!({
    "username": username,
    "password": password,
    "fullname": fullname,
    "roleName": role_name,
  })
  .to_string()
}
