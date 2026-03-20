import { DispatchDeleteDialog } from './dispatch-delete-dialog'
import { DispatchLifecycleDialog } from './dispatch-lifecycle-dialog'
import { DispatchMutateDialog } from './dispatch-mutate-dialog'
import { useDispatch } from './dispatch-provider'

export function DispatchDialogs() {
  const { open, setOpen, currentRow } = useDispatch()

  return (
    <>
      <DispatchMutateDialog
        open={open === 'create' || open === 'update'}
        onOpenChange={() => setOpen(null)}
        currentRow={open === 'update' ? currentRow : null}
      />
      <DispatchDeleteDialog
        open={open === 'delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="soft"
      />
      <DispatchDeleteDialog
        open={open === 'hard-delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="hard"
      />
      <DispatchLifecycleDialog
        open={open === 'execute'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="execute"
      />
      <DispatchLifecycleDialog
        open={open === 'revert'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="revert"
      />
    </>
  )
}
