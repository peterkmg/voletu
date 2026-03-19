import { OwnershipTransferLifecycleDialog } from './ownership-transfer-lifecycle-dialog'
import { OwnershipTransferMutateDrawer } from './ownership-transfer-mutate-drawer'
import { useOwnershipTransfer } from './ownership-transfer-provider'

export function OwnershipTransferDialogs() {
  const { open, setOpen, currentRow } = useOwnershipTransfer()

  return (
    <>
      <OwnershipTransferMutateDrawer
        open={open === 'create'}
        onOpenChange={() => setOpen(null)}
      />
      <OwnershipTransferLifecycleDialog
        open={open === 'execute'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        action="execute"
      />
      <OwnershipTransferLifecycleDialog
        open={open === 'revert'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        action="revert"
      />
    </>
  )
}
