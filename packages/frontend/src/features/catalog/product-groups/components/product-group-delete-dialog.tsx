import type { ProductGroupResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { EntityDeleteDialog } from '~/components/entity-delete-dialog'
import { catalogProductGroupHardDelete, catalogProductGroupSoftDelete } from '~/generated/client'
import { catalogProductGroupListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogProductGroupList'

interface ProductGroupDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: ProductGroupResponse | null
  variant: 'soft' | 'hard'
}

export function ProductGroupDeleteDialog({ open, onOpenChange, currentRow, variant }: ProductGroupDeleteDialogProps) {
  const { t } = useTranslation(['common', 'catalog'])
  return (
    <EntityDeleteDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      variant={variant}
      hardDeleteFn={catalogProductGroupHardDelete}
      softDeleteFn={catalogProductGroupSoftDelete}
      queryKey={catalogProductGroupListQueryKey()}
      entityLabel={t('catalog:productGroup.singular')}
    />
  )
}
