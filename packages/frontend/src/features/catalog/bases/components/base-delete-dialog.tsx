import type { BaseResponse } from '~/generated/types/BaseResponse'
import { useTranslation } from 'react-i18next'
import { EntityDeleteDialog } from '~/components/entity-delete-dialog'
import { catalogBaseHardDelete, catalogBaseSoftDelete } from '~/generated/client'
import { catalogBaseListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogBaseList'

interface BaseDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: BaseResponse | null
  variant: 'soft' | 'hard'
}

export function BaseDeleteDialog({ open, onOpenChange, currentRow, variant }: BaseDeleteDialogProps) {
  const { t } = useTranslation(['common', 'catalog'])
  return (
    <EntityDeleteDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      variant={variant}
      hardDeleteFn={catalogBaseHardDelete}
      softDeleteFn={catalogBaseSoftDelete}
      queryKey={catalogBaseListQueryKey()}
      entityLabel={t('catalog:base.singular')}
    />
  )
}
