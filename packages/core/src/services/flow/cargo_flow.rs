use uuid::Uuid;

use super::FlowService;
use crate::{
  api::ApiError,
  dtos::response::flow::CargoFlowRow,
  enums::{FlowOperation, FlowType, PipelineStatus},
};

impl FlowService {
  pub async fn cargo_flow_query(
    &self,
    _flow_type: Option<FlowType>,
    _operation: Option<FlowOperation>,
    _status: Option<PipelineStatus>,
    _contractor_id: Option<Uuid>,
    _page: Option<u64>,
    _per_page: Option<u64>,
  ) -> Result<Vec<CargoFlowRow>, ApiError> {
    // TODO: Rewrite using sea_query UNION ALL with DB-level pagination.
    // This is a placeholder to unblock compilation while the linked flow
    // services are being tested. Will be implemented properly next.
    Ok(vec![])
  }
}
