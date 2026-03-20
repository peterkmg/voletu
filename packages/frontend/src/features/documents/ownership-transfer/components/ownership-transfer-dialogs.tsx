import { OwnershipTransferLifecycleDialog } from './ownership-transfer-lifecycle-dialog'
import { OwnershipTransferMutateDialog } from './ownership-transfer-mutate-dialog'
import { useOwnershipTransfer } from './ownership-transfer-provider'

export function OwnershipTransferDialogs() {
  const { open, setOpen, currentRow } = useOwnershipTransfer()

  return (
    <>
      <OwnershipTransferMutateDialog
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
