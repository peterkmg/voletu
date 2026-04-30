import type * as React from 'react'
import type {
  EntityDialogState,
  EntityLifecycleAction,
} from './entity-dialog-state'

interface EntityHook<TRow> {
  dialog: EntityDialogState<TRow> | null
  closeDialog: () => void
}

export interface MutateDialogProps<TRow> {
  open: boolean
  onOpenChange: () => void
  currentRow: TRow | null
}

interface LifecycleDialogBaseProps<TRow> {
  open: boolean
  onOpenChange: () => void
  currentRow: TRow | null
}

interface LifecycleDialogWithVariant<TRow> {
  LifecycleDialog: React.ComponentType<
    LifecycleDialogBaseProps<TRow> & { variant: EntityLifecycleAction }
  >
  lifecyclePropName?: 'variant'
}

interface LifecycleDialogWithAction<TRow> {
  LifecycleDialog: React.ComponentType<
    LifecycleDialogBaseProps<TRow> & { action: EntityLifecycleAction }
  >
  lifecyclePropName: 'action'
}

interface WithoutLifecycle {
  LifecycleDialog?: undefined
  lifecyclePropName?: undefined
}

// Secondary dialog slot used by the truck / rail pipeline lists when the
// "Issue acceptance" row trigger needs to spawn a *different* document
// (an Acceptance) seeded with the row as its basis. It deliberately lives
// alongside the existing `LifecycleDialog` slot rather than overloading it
// because the dialog's prop signature (`prefillBasis` instead of
// `currentRow + variant/action`) is too divergent for a discriminated
// union to narrow cleanly in object-literal contexts. See spec §3.2 / §6.3.
export interface IssueAcceptanceDialogConfig {
  IssueAcceptanceDialog?: React.ComponentType<{
    open: boolean
    onOpenChange: (open: boolean) => void
    prefillBasis: { kind: 'truck' | 'rail', basisId: string } | undefined
  }>
  /**
   * Discriminator passed through to the dialog's `prefillBasis.kind`. Required
   * whenever `IssueAcceptanceDialog` is set.
   */
  prefillBasisKind?: 'truck' | 'rail'
}

interface EntityDialogsConfigBase<TRow> extends IssueAcceptanceDialogConfig {
  useEntity: () => EntityHook<TRow>
  MutateDialog: React.ComponentType<MutateDialogProps<TRow>>
  DeleteDialog?: React.ComponentType
  supportsUpdate?: boolean
}

export type EntityDialogsLifecycleConfig<TRow>
  = | LifecycleDialogWithVariant<TRow>
    | LifecycleDialogWithAction<TRow>
    | WithoutLifecycle

export type EntityDialogsConfig<TRow extends { id: string }>
  = EntityDialogsConfigBase<TRow> & EntityDialogsLifecycleConfig<TRow>

export function createEntityDialogs<TRow extends { id: string }>(config: EntityDialogsConfig<TRow>) {
  const {
    useEntity,
    DeleteDialog,
    supportsUpdate,
    IssueAcceptanceDialog,
    prefillBasisKind,
  } = config

  function EntityDialogs() {
    const { dialog, closeDialog } = useEntity()
    const Mutate = config.MutateDialog
    const currentRow = dialog?.kind === 'update'
      || dialog?.kind === 'delete'
      || dialog?.kind === 'lifecycle'
      || dialog?.kind === 'issueAcceptance'
      ? dialog.row
      : null

    let lifecycleDialogs: React.ReactNode = null

    if ('LifecycleDialog' in config && config.LifecycleDialog) {
      if (config.lifecyclePropName === 'action') {
        const LifecycleDialog = config.LifecycleDialog
        lifecycleDialogs = (
          <>
            <LifecycleDialog
              open={dialog?.kind === 'lifecycle' && dialog.action === 'execute'}
              onOpenChange={closeDialog}
              currentRow={currentRow}
              action="execute"
            />
            <LifecycleDialog
              open={dialog?.kind === 'lifecycle' && dialog.action === 'revert'}
              onOpenChange={closeDialog}
              currentRow={currentRow}
              action="revert"
            />
          </>
        )
      }
      else {
        const LifecycleDialog = config.LifecycleDialog
        lifecycleDialogs = (
          <>
            <LifecycleDialog
              open={dialog?.kind === 'lifecycle' && dialog.action === 'execute'}
              onOpenChange={closeDialog}
              currentRow={currentRow}
              variant="execute"
            />
            <LifecycleDialog
              open={dialog?.kind === 'lifecycle' && dialog.action === 'revert'}
              onOpenChange={closeDialog}
              currentRow={currentRow}
              variant="revert"
            />
          </>
        )
      }
    }

    let issueAcceptanceDialog: React.ReactNode = null
    if (IssueAcceptanceDialog && prefillBasisKind) {
      const isIssueAcceptance = dialog?.kind === 'issueAcceptance'
      issueAcceptanceDialog = (
        <IssueAcceptanceDialog
          open={isIssueAcceptance}
          onOpenChange={() => closeDialog()}
          prefillBasis={isIssueAcceptance && currentRow
            ? { kind: prefillBasisKind, basisId: currentRow.id }
            : undefined}
        />
      )
    }

    return (
      <>
        <Mutate
          open={supportsUpdate === false ? dialog?.kind === 'create' : (dialog?.kind === 'create' || dialog?.kind === 'update')}
          onOpenChange={closeDialog}
          currentRow={supportsUpdate === false ? null : (dialog?.kind === 'update' ? currentRow : null)}
        />
        {DeleteDialog && <DeleteDialog />}
        {lifecycleDialogs}
        {issueAcceptanceDialog}
      </>
    )
  }

  return EntityDialogs
}
