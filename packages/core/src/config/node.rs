use uuid::Uuid;

#[derive(Clone)]
pub struct NodeConfig {
  pub database_id: Uuid,
  pub jwt_secret: String,
}

impl NodeConfig {
  pub fn new(database_id: Uuid, jwt_secret: String) -> Self {
    Self {
      database_id,
      jwt_secret,
    }
  }
}
