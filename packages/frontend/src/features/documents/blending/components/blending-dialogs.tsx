import { BlendingDeleteDialog } from './blending-delete-dialog'
import { BlendingLifecycleDialog } from './blending-lifecycle-dialog'
import { BlendingMutateDrawer } from './blending-mutate-drawer'
import { useBlending } from './blending-provider'

export function BlendingDialogs() {
  const { open, setOpen, currentRow } = useBlending()

  return (
    <>
      <BlendingMutateDrawer
        open={open === 'create' || open === 'update'}
        onOpenChange={() => setOpen(null)}
        currentRow={open === 'update' ? currentRow : null}
      />
      <BlendingDeleteDialog
        open={open === 'delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="soft"
      />
      <BlendingDeleteDialog
        open={open === 'hard-delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="hard"
      />
      <BlendingLifecycleDialog
        open={open === 'execute'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="execute"
      />
      <BlendingLifecycleDialog
        open={open === 'revert'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="revert"
      />
    </>
  )
}
