import { PortDeleteDialog } from './port-delete-dialog'
import { PortMutateDrawer } from './port-mutate-drawer'
import { usePorts } from './ports-provider'

export function PortsDialogs() {
  const { open, setOpen, currentRow } = usePorts()

  return (
    <>
      <PortMutateDrawer
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
