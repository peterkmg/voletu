export type EntityDeleteMode = 'soft' | 'hard'

export type EntityLifecycleAction = 'execute' | 'revert'

// Pipeline-list "Issue acceptance" trigger (truck/rail incoming flows).
// Carried in its own `EntityRowAction` slot rather than the lifecycle action
// because it spawns a *different* document and uses a divergent dialog
// signature — see spec §3.2 / §6.3 and `IssueAcceptanceDialogConfig` in
// `create-entity-dialogs.tsx`.
export type EntityRowAction = EntityLifecycleAction | 'issueAcceptance'

export type EntityDialogState<TRow>
  = | { kind: 'create' }
    | { kind: 'update', row: TRow }
    | { kind: 'delete', row: TRow, mode: EntityDeleteMode }
    | { kind: 'lifecycle', row: TRow, action: EntityLifecycleAction }
    | { kind: 'issueAcceptance', row: TRow }
