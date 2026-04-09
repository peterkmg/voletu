use uuid::Uuid;
use voletu_core_macros::request_dto;

use crate::enums::SyncDirection;

#[request_dto]
pub struct UpsertWatermarkRequest {
  pub target_node_id: Uuid,
  pub direction: SyncDirection,
  pub last_audit_log_id: Uuid,
  /// Canonical base discriminant to store alongside the cursor. Optional;
  /// defaults to empty string ("catalog-only scope") when omitted. This
  /// endpoint is a manual override - the normal pull path goes through
  /// `apply_pulled_logs`, which sets the discriminant atomically from the
  /// peripheral's actual assignments.
  #[serde(default)]
  pub base_discriminant: Option<String>,
}
