pub mod audit;
pub mod catalog;
pub mod common;
pub mod document;
pub mod ledger;
pub mod sync;
pub mod system;

/*
Service macro contracts:

- #[entity_service(...)]
  Generates service methods on the annotated impl target using the entity prefix.
  Internal mutable helpers follow *_no_tx naming and are pub(crate) implementation details.
  Public mutable methods open/commit transactions and delegate to internal no-tx helpers.
  Read methods keep direct service-level DB access (no wrapper/no-tx split).
  CRUD response mapping uses direct Into conversion (no mapper argument).

  Required parameters:
  - entity = <ident>
  - entity_mod = <path>
  - entity_name = <"Label used in not-found/conflict messages">
  - ops(<op_a>, <op_b>, ...)

  Optional parameters (required only when relevant ops are enabled):
  - create_req = <path> (required by create)
  - update_req = <path> (required by update)
  - response = <path> (required by create/list/get/update)
  - apply_update = <path> (required by update)
  - apply_soft_delete = <path> (optional soft-delete mutator)
  - before_update = <path> (optional async pre-update hook)
  - before_soft_delete = <path> (optional async pre-soft-delete hook)
  - before_execute = <path> (optional async execute hook)
  - before_revert = <path> (optional async revert hook)

  Supported ops:
  - create, list, get, update, soft_delete, hard_delete, create_and_execute, execute, revert

  Generated naming matrix:
  - create: <entity>_create + <entity>_create_no_tx
  - update: <entity>_update + <entity>_update_no_tx
  - create+execute: <entity>_create_and_execute
  - delete state: <entity>_soft_delete + <entity>_soft_delete_undo + internal state setter
  - hard delete: <entity>_hard_delete
  - execute/revert: <entity>_execute + <entity>_execute_no_tx + <entity>_revert + <entity>_revert_no_tx
*/

pub use audit::AuditService;
pub use catalog::CatalogService;
pub use document::DocumentService;
pub use ledger::LedgerService;
pub use sync::SyncService;
pub use system::SystemService;
