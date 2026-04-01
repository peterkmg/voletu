import type * as React from 'react'

interface EntityHook<TRow> {
  open: string | null
  setOpen: (v: null) => void
  currentRow: TRow | null
}

interface MutateDialogWithRowProps<TRow> {
  open: boolean
  onOpenChange: () => void
  currentRow: TRow | null
}

interface ConfigWithUpdate<TRow> {
  useEntity: () => EntityHook<TRow>
  MutateDialog: React.ComponentType<MutateDialogWithRowProps<TRow>>
  DeleteDialog?: React.ComponentType
  supportsUpdate?: true
}

interface ConfigWithoutUpdate<TRow> {
  useEntity: () => EntityHook<TRow>
  MutateDialog: React.ComponentType<{ open: boolean, onOpenChange: () => void }>
  DeleteDialog?: React.ComponentType
  supportsUpdate: false
}

interface WithVariantLifecycle<TRow> {
  LifecycleDialog: React.ComponentType<{
    open: boolean
    onOpenChange: () => void
    currentRow: TRow | null
    variant: 'execute' | 'revert'
  }>
  lifecyclePropName?: 'variant'
}

interface WithActionLifecycle<TRow> {
  LifecycleDialog: React.ComponentType<{
    open: boolean
    onOpenChange: () => void
    currentRow: TRow | null
    action: 'execute' | 'revert'
  }>
  lifecyclePropName: 'action'
}

interface WithoutLifecycle {
  LifecycleDialog?: undefined
  lifecyclePropName?: undefined
}

type EntityDialogsConfig<TRow>
  = (ConfigWithUpdate<TRow> | ConfigWithoutUpdate<TRow>)
    & (WithVariantLifecycle<TRow> | WithActionLifecycle<TRow> | WithoutLifecycle)

export function createEntityDialogs<TRow>(config: EntityDialogsConfig<TRow>) {
  const {
    useEntity,
    DeleteDialog,
    supportsUpdate,
  } = config

  function EntityDialogs() {
    const { open, setOpen, currentRow } = useEntity()
    const onClose = () => setOpen(null)
    const Mutate = config.MutateDialog as React.ComponentType<MutateDialogWithRowProps<TRow>>

    const hasLifecycle = 'LifecycleDialog' in config && config.LifecycleDialog
    const LifecycleDialog = hasLifecycle
      ? config.LifecycleDialog as React.ComponentType<{
        open: boolean
        onOpenChange: () => void
        currentRow: TRow | null
        variant?: 'execute' | 'revert'
        action?: 'execute' | 'revert'
      }>
      : null
    const propName = ('lifecyclePropName' in config ? config.lifecyclePropName : undefined) ?? 'variant'

    return (
      <>
        <Mutate
          open={supportsUpdate === false ? open === 'create' : (open === 'create' || open === 'update')}
          onOpenChange={onClose}
          currentRow={supportsUpdate === false ? null : (open === 'update' ? currentRow : null)}
        />
        {DeleteDialog && <DeleteDialog />}
        {LifecycleDialog && (
          <>
            <LifecycleDialog
              open={open === 'execute'}
              onOpenChange={onClose}
              currentRow={currentRow}
              {...(propName === 'action'
                ? { action: 'execute' as const }
                : { variant: 'execute' as const })}
            />
            <LifecycleDialog
              open={open === 'revert'}
              onOpenChange={onClose}
              currentRow={currentRow}
              {...(propName === 'action'
                ? { action: 'revert' as const }
                : { variant: 'revert' as const })}
            />
          </>
        )}
      </>
    )
  }

  return EntityDialogs
}
