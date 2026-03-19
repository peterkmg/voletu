import { PhysicalTransferLifecycleDialog } from './physical-transfer-lifecycle-dialog'
import { PhysicalTransferMutateDrawer } from './physical-transfer-mutate-drawer'
import { usePhysicalTransfer } from './physical-transfer-provider'

export function PhysicalTransferDialogs() {
  const { open, setOpen, currentRow } = usePhysicalTransfer()

  return (
    <>
      <PhysicalTransferMutateDrawer
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
