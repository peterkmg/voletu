import { RailWaybillDeleteDialog } from './rail-waybill-delete-dialog'
import { RailWaybillMutateDialog } from './rail-waybill-mutate-dialog'
import { useRailWaybills } from './rail-waybills-provider'

export function RailWaybillsDialogs() {
  const { open, setOpen, currentRow } = useRailWaybills()

  return (
    <>
      <RailWaybillMutateDialog
        open={open === 'create' || open === 'update'}
        onOpenChange={() => setOpen(null)}
        currentRow={open === 'update' ? currentRow : null}
      />
      <RailWaybillDeleteDialog
        open={open === 'delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="soft"
      />
      <RailWaybillDeleteDialog
        open={open === 'hard-delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="hard"
      />
    </>
  )
}
