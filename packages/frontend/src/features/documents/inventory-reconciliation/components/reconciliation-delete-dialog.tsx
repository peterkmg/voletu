import type { InventoryReconciliationResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { EntityDeleteDialog } from '~/components/entity-delete-dialog'
import { reconciliationHardDelete, reconciliationSoftDelete } from '~/generated/client'
import { reconciliationListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/useReconciliationList'

interface ReconciliationDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: InventoryReconciliationResponse | null
  variant: 'soft' | 'hard'
}

export function ReconciliationDeleteDialog({ open, onOpenChange, currentRow, variant }: ReconciliationDeleteDialogProps) {
  const { t } = useTranslation(['common', 'documents'])
  return (
    <EntityDeleteDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      variant={variant}
      hardDeleteFn={reconciliationHardDelete}
      softDeleteFn={reconciliationSoftDelete}
      queryKey={reconciliationListQueryKey()}
      entityLabel={t('documents:reconciliation.singular')}
    />
  )
}
