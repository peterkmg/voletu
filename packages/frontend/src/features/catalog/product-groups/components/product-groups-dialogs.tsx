import { ProductGroupDeleteDialog } from './product-group-delete-dialog'
import { ProductGroupMutateDrawer } from './product-group-mutate-drawer'
import { useProductGroups } from './product-groups-provider'

export function ProductGroupsDialogs() {
  const { open, setOpen, currentRow } = useProductGroups()

  return (
    <>
      <ProductGroupMutateDrawer
        open={open === 'create' || open === 'update'}
        onOpenChange={() => setOpen(null)}
        currentRow={open === 'update' ? currentRow : null}
      />
      <ProductGroupDeleteDialog
        open={open === 'delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="soft"
      />
      <ProductGroupDeleteDialog
        open={open === 'hard-delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="hard"
      />
    </>
  )
}
