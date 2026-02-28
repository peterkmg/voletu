use uuid::Uuid;

#[derive(Clone)]
pub struct NodeConfig {
  pub database_id: Uuid,
  pub node_type: String,
  pub jwt_secret: String,
  pub central_api_url: Option<String>,
}

impl NodeConfig {
  pub fn new(
    database_id: Uuid,
    node_type: String,
    jwt_secret: String,
    central_api_url: Option<String>,
  ) -> Self {
    Self {
      database_id,
      node_type,
      jwt_secret,
      central_api_url,
    }
  }
}
