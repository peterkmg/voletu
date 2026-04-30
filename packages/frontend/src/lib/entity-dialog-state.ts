export type EntityDeleteMode = 'soft' | 'hard'

export type EntityLifecycleAction = 'execute' | 'revert'

export type EntityRowAction = EntityLifecycleAction | 'issueAcceptance'

export type EntityDialogState<TRow>
  = | { kind: 'create' }
    | { kind: 'update', row: TRow }
    | { kind: 'delete', row: TRow, mode: EntityDeleteMode }
    | { kind: 'lifecycle', row: TRow, action: EntityLifecycleAction }
    | { kind: 'issueAcceptance', row: TRow }
