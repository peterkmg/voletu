import { TruckWaybillDeleteDialog } from './truck-waybill-delete-dialog'
import { TruckWaybillMutateDrawer } from './truck-waybill-mutate-drawer'
import { useTruckWaybills } from './truck-waybills-provider'

export function TruckWaybillsDialogs() {
  const { open, setOpen, currentRow } = useTruckWaybills()

  return (
    <>
      <TruckWaybillMutateDrawer
        open={open === 'create' || open === 'update'}
        onOpenChange={() => setOpen(null)}
        currentRow={open === 'update' ? currentRow : null}
      />
      <TruckWaybillDeleteDialog
        open={open === 'delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="soft"
      />
      <TruckWaybillDeleteDialog
        open={open === 'hard-delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="hard"
      />
    </>
  )
}
