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

interface EntityDialogsConfigBase<TRow> {
  useEntity: () => EntityHook<TRow>
  MutateDialog: React.ComponentType<MutateDialogProps<TRow>>
  DeleteDialog?: React.ComponentType
  supportsUpdate?: boolean
}

export type EntityDialogsLifecycleConfig<TRow>
  = | LifecycleDialogWithVariant<TRow>
    | LifecycleDialogWithAction<TRow>
    | WithoutLifecycle

export type EntityDialogsConfig<TRow>
  = EntityDialogsConfigBase<TRow> & EntityDialogsLifecycleConfig<TRow>

export function createEntityDialogs<TRow>(config: EntityDialogsConfig<TRow>) {
  const {
    useEntity,
    DeleteDialog,
    supportsUpdate,
  } = config

  function EntityDialogs() {
    const { dialog, closeDialog } = useEntity()
    const Mutate = config.MutateDialog
    const currentRow = dialog?.kind === 'update' || dialog?.kind === 'delete' || dialog?.kind === 'lifecycle'
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

    return (
      <>
        <Mutate
          open={supportsUpdate === false ? dialog?.kind === 'create' : (dialog?.kind === 'create' || dialog?.kind === 'update')}
          onOpenChange={closeDialog}
          currentRow={supportsUpdate === false ? null : (dialog?.kind === 'update' ? currentRow : null)}
        />
        {DeleteDialog && <DeleteDialog />}
        {lifecycleDialogs}
      </>
    )
  }

  return EntityDialogs
}
