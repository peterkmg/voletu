import { BaseDeleteDialog } from './base-delete-dialog'
import { BaseMutateDialog } from './base-mutate-dialog'
import { useBases } from './bases-provider'

export function BasesDialogs() {
  const { open, setOpen, currentRow } = useBases()

  return (
    <>
      <BaseMutateDialog
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
