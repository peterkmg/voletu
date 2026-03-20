import { WarehouseDeleteDialog } from './warehouse-delete-dialog'
import { WarehouseMutateDialog } from './warehouse-mutate-dialog'
import { useWarehouses } from './warehouses-provider'

export function WarehousesDialogs() {
  const { open, setOpen, currentRow } = useWarehouses()

  return (
    <>
      <WarehouseMutateDialog
        open={open === 'create' || open === 'update'}
        onOpenChange={() => setOpen(null)}
        currentRow={open === 'update' ? currentRow : null}
      />
      <WarehouseDeleteDialog
        open={open === 'delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="soft"
      />
      <WarehouseDeleteDialog
        open={open === 'hard-delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="hard"
      />
    </>
  )
}
