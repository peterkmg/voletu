import { StorageDeleteDialog } from './storage-delete-dialog'
import { StorageMutateDrawer } from './storage-mutate-drawer'
import { useStorages } from './storages-provider'

export function StoragesDialogs() {
  const { open, setOpen, currentRow } = useStorages()

  return (
    <>
      <StorageMutateDrawer
        open={open === 'create' || open === 'update'}
        onOpenChange={() => setOpen(null)}
        currentRow={open === 'update' ? currentRow : null}
      />
      <StorageDeleteDialog
        open={open === 'delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="soft"
      />
      <StorageDeleteDialog
        open={open === 'hard-delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="hard"
      />
    </>
  )
}
