import { ReconciliationDeleteDialog } from './reconciliation-delete-dialog'
import { ReconciliationLifecycleDialog } from './reconciliation-lifecycle-dialog'
import { ReconciliationMutateDialog } from './reconciliation-mutate-dialog'
import { useReconciliation } from './reconciliation-provider'

export function ReconciliationDialogs() {
  const { open, setOpen, currentRow } = useReconciliation()

  return (
    <>
      <ReconciliationMutateDialog
        open={open === 'create' || open === 'update'}
        onOpenChange={() => setOpen(null)}
        currentRow={open === 'update' ? currentRow : null}
      />
      <ReconciliationDeleteDialog
        open={open === 'delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="soft"
      />
      <ReconciliationDeleteDialog
        open={open === 'hard-delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="hard"
      />
      <ReconciliationLifecycleDialog
        open={open === 'execute'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="execute"
      />
      <ReconciliationLifecycleDialog
        open={open === 'revert'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="revert"
      />
    </>
  )
}
