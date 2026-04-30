import type {
  EntityDeleteMode,
  EntityDialogState,
  EntityLifecycleAction,
} from './entity-dialog-state'
import * as React from 'react'
import { useState } from 'react'

interface EntityContextType<
  TRow,
> {
  dialog: EntityDialogState<TRow> | null
  closeDialog: () => void
  openCreate: () => void
  openUpdate: (row: TRow) => void
  openDelete: (row: TRow, mode?: EntityDeleteMode) => void
  openLifecycle: (row: TRow, action: EntityLifecycleAction) => void
  openIssueAcceptance: (row: TRow) => void
}

export function createEntityProvider<
  TRow extends { id: string },
>(displayName: string) {
  const Context = React.createContext<EntityContextType<TRow> | null>(null)

  function Provider({ children }: { children: React.ReactNode }) {
    const [dialog, setDialog] = useState<EntityDialogState<TRow> | null>(null)

    const closeDialog = () => setDialog(null)
    const openCreate = () => setDialog({ kind: 'create' })
    const openUpdate = (row: TRow) => setDialog({ kind: 'update', row })
    const openDelete = (row: TRow, mode: EntityDeleteMode = 'soft') =>
      setDialog({ kind: 'delete', row, mode })
    const openLifecycle = (row: TRow, action: EntityLifecycleAction) =>
      setDialog({ kind: 'lifecycle', row, action })
    const openIssueAcceptance = (row: TRow) =>
      setDialog({ kind: 'issueAcceptance', row })

    return (
      <Context value={{ dialog, closeDialog, openCreate, openUpdate, openDelete, openLifecycle, openIssueAcceptance }}>
        {children}
      </Context>
    )
  }
  Provider.displayName = `${displayName}Provider`

  function useEntity() {
    const ctx = React.use(Context)
    if (!ctx) {
      throw new Error(`use${displayName} must be used within <${displayName}Provider>`)
    }
    return ctx
  }

  return { Provider, useEntity } as const
}
