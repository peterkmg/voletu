import type { PortResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { EntityDeleteDialog } from '~/components/entity-delete-dialog'
import { catalogPortHardDelete, catalogPortSoftDelete } from '~/generated/client'
import { catalogPortListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogPortList'

interface PortDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: PortResponse | null
  variant: 'soft' | 'hard'
}

export function PortDeleteDialog({ open, onOpenChange, currentRow, variant }: PortDeleteDialogProps) {
  const { t } = useTranslation(['common', 'catalog'])
  return (
    <EntityDeleteDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      variant={variant}
      hardDeleteFn={catalogPortHardDelete}
      softDeleteFn={catalogPortSoftDelete}
      queryKey={catalogPortListQueryKey()}
      entityLabel={t('catalog:port.singular')}
    />
  )
}
