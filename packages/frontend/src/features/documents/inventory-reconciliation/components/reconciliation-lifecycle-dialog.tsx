import type { InventoryReconciliationResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { LifecycleDialog } from '~/components/lifecycle-dialog'
import { reconciliationExecute, reconciliationRevert } from '~/generated/client'
import { reconciliationListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/useReconciliationList'

interface ReconciliationLifecycleDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: InventoryReconciliationResponse | null
  variant: 'execute' | 'revert'
}

export function ReconciliationLifecycleDialog({ open, onOpenChange, currentRow, variant }: ReconciliationLifecycleDialogProps) {
  const { t } = useTranslation(['documents'])
  return (
    <LifecycleDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      action={variant}
      executeFn={reconciliationExecute}
      revertFn={reconciliationRevert}
      queryKey={reconciliationListQueryKey()}
      entityLabel={t('documents:reconciliation.singular')}
    />
  )
}
