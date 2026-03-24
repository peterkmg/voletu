import type { PhysicalTransferResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { LifecycleDialog } from '~/components/lifecycle-dialog'
import { physicalTransferExecute, physicalTransferRevert } from '~/generated/client'
import { physicalTransferListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/usePhysicalTransferList'

interface PhysicalTransferLifecycleDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: PhysicalTransferResponse | null
  action: 'execute' | 'revert'
}

export function PhysicalTransferLifecycleDialog({ open, onOpenChange, currentRow, action }: PhysicalTransferLifecycleDialogProps) {
  const { t } = useTranslation(['documents'])
  return (
    <LifecycleDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      action={action}
      executeFn={physicalTransferExecute}
      revertFn={physicalTransferRevert}
      queryKey={physicalTransferListQueryKey()}
      entityLabel={t('documents:physicalTransfer.singular')}
    />
  )
}
