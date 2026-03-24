import type { ProductTypeResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { EntityDeleteDialog } from '~/components/entity-delete-dialog'
import { catalogProductTypeHardDelete, catalogProductTypeSoftDelete } from '~/generated/client'
import { catalogProductTypeListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogProductTypeList'

interface ProductTypeDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: ProductTypeResponse | null
  variant: 'soft' | 'hard'
}

export function ProductTypeDeleteDialog({ open, onOpenChange, currentRow, variant }: ProductTypeDeleteDialogProps) {
  const { t } = useTranslation(['common', 'catalog'])
  return (
    <EntityDeleteDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      variant={variant}
      hardDeleteFn={catalogProductTypeHardDelete}
      softDeleteFn={catalogProductTypeSoftDelete}
      queryKey={catalogProductTypeListQueryKey()}
      entityLabel={t('catalog:productType.singular')}
    />
  )
}
