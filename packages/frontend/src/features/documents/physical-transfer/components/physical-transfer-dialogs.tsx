import { PhysicalTransferLifecycleDialog } from './physical-transfer-lifecycle-dialog'
import { PhysicalTransferMutateDialog } from './physical-transfer-mutate-dialog'
import { usePhysicalTransfer } from './physical-transfer-provider'

export function PhysicalTransferDialogs() {
  const { open, setOpen, currentRow } = usePhysicalTransfer()

  return (
    <>
      <PhysicalTransferMutateDialog
        open={open === 'create'}
        onOpenChange={() => setOpen(null)}
      />
      <PhysicalTransferLifecycleDialog
        open={open === 'execute'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        action="execute"
      />
      <PhysicalTransferLifecycleDialog
        open={open === 'revert'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        action="revert"
      />
    </>
  )
}
