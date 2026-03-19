import { AcceptanceDeleteDialog } from './acceptance-delete-dialog'
import { AcceptanceLifecycleDialog } from './acceptance-lifecycle-dialog'
import { AcceptanceMutateDrawer } from './acceptance-mutate-drawer'
import { useAcceptance } from './acceptance-provider'

export function AcceptanceDialogs() {
  const { open, setOpen, currentRow } = useAcceptance()

  return (
    <>
      <AcceptanceMutateDrawer
        open={open === 'create' || open === 'update'}
        onOpenChange={() => setOpen(null)}
        currentRow={open === 'update' ? currentRow : null}
      />
      <AcceptanceDeleteDialog
        open={open === 'delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="soft"
      />
      <AcceptanceDeleteDialog
        open={open === 'hard-delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="hard"
      />
      <AcceptanceLifecycleDialog
        open={open === 'execute'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        action="execute"
      />
      <AcceptanceLifecycleDialog
        open={open === 'revert'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        action="revert"
      />
    </>
  )
}
