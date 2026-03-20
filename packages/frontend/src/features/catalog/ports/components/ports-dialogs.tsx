import { PortDeleteDialog } from './port-delete-dialog'
import { PortMutateDialog } from './port-mutate-dialog'
import { usePorts } from './ports-provider'

export function PortsDialogs() {
  const { open, setOpen, currentRow } = usePorts()

  return (
    <>
      <PortMutateDialog
        open={open === 'create' || open === 'update'}
        onOpenChange={() => setOpen(null)}
        currentRow={open === 'update' ? currentRow : null}
      />
      <PortDeleteDialog
        open={open === 'delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="soft"
      />
      <PortDeleteDialog
        open={open === 'hard-delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="hard"
      />
    </>
  )
}
