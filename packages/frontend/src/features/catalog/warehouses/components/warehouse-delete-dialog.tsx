import type { WarehouseResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { EntityDeleteDialog } from '~/components/entity-delete-dialog'
import { catalogWarehouseHardDelete, catalogWarehouseSoftDelete } from '~/generated/client'
import { catalogWarehouseListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogWarehouseList'

interface WarehouseDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: WarehouseResponse | null
  variant: 'soft' | 'hard'
}

export function WarehouseDeleteDialog({ open, onOpenChange, currentRow, variant }: WarehouseDeleteDialogProps) {
  const { t } = useTranslation(['common', 'catalog'])
  return (
    <EntityDeleteDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      variant={variant}
      hardDeleteFn={catalogWarehouseHardDelete}
      softDeleteFn={catalogWarehouseSoftDelete}
      queryKey={catalogWarehouseListQueryKey()}
      entityLabel={t('catalog:warehouse.singular')}
    />
  )
}
