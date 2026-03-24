import type { ProductResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { EntityDeleteDialog } from '~/components/entity-delete-dialog'
import { catalogProductHardDelete, catalogProductSoftDelete } from '~/generated/client'
import { catalogProductListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogProductList'

interface ProductDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: ProductResponse | null
  variant: 'soft' | 'hard'
}

export function ProductDeleteDialog({ open, onOpenChange, currentRow, variant }: ProductDeleteDialogProps) {
  const { t } = useTranslation(['common', 'catalog'])
  return (
    <EntityDeleteDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      variant={variant}
      hardDeleteFn={catalogProductHardDelete}
      softDeleteFn={catalogProductSoftDelete}
      queryKey={catalogProductListQueryKey()}
      entityLabel={t('catalog:product.singular')}
    />
  )
}
