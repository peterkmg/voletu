import { ProductTypeDeleteDialog } from './product-type-delete-dialog'
import { ProductTypeMutateDrawer } from './product-type-mutate-drawer'
import { useProductTypes } from './product-types-provider'

export function ProductTypesDialogs() {
  const { open, setOpen, currentRow } = useProductTypes()

  return (
    <>
      <ProductTypeMutateDrawer
        open={open === 'create' || open === 'update'}
        onOpenChange={() => setOpen(null)}
        currentRow={open === 'update' ? currentRow : null}
      />
      <ProductTypeDeleteDialog
        open={open === 'delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="soft"
      />
      <ProductTypeDeleteDialog
        open={open === 'hard-delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="hard"
      />
    </>
  )
}
