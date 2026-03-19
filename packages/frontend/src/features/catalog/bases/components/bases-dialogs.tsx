import { BaseDeleteDialog } from './base-delete-dialog'
import { BaseMutateDrawer } from './base-mutate-drawer'
import { useBases } from './bases-provider'

export function BasesDialogs() {
  const { open, setOpen, currentRow } = useBases()

  return (
    <>
      <BaseMutateDrawer
        open={open === 'create' || open === 'update'}
        onOpenChange={() => setOpen(null)}
        currentRow={open === 'update' ? currentRow : null}
      />
      <BaseDeleteDialog
        open={open === 'delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="soft"
      />
      <BaseDeleteDialog
        open={open === 'hard-delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="hard"
      />
    </>
  )
}
