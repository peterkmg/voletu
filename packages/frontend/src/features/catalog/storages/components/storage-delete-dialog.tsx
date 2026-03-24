import type { StorageResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { EntityDeleteDialog } from '~/components/entity-delete-dialog'
import { catalogStorageHardDelete, catalogStorageSoftDelete } from '~/generated/client'
import { catalogStorageListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogStorageList'

interface StorageDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: StorageResponse | null
  variant: 'soft' | 'hard'
}

export function StorageDeleteDialog({ open, onOpenChange, currentRow, variant }: StorageDeleteDialogProps) {
  const { t } = useTranslation(['common', 'catalog'])
  return (
    <EntityDeleteDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      variant={variant}
      hardDeleteFn={catalogStorageHardDelete}
      softDeleteFn={catalogStorageSoftDelete}
      queryKey={catalogStorageListQueryKey()}
      entityLabel={t('catalog:storage.singular')}
    />
  )
}
