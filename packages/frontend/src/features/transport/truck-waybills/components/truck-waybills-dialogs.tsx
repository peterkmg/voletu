import { TruckWaybillDeleteDialog } from './truck-waybill-delete-dialog'
import { TruckWaybillMutateDialog } from './truck-waybill-mutate-dialog'
import { useTruckWaybills } from './truck-waybills-provider'

export function TruckWaybillsDialogs() {
  const { open, setOpen, currentRow } = useTruckWaybills()

  return (
    <>
      <TruckWaybillMutateDialog
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
