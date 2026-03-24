import type { OwnershipTransferResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { LifecycleDialog } from '~/components/lifecycle-dialog'
import { ownershipTransferExecute, ownershipTransferRevert } from '~/generated/client'
import { ownershipTransferListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/useOwnershipTransferList'

interface OwnershipTransferLifecycleDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: OwnershipTransferResponse | null
  action: 'execute' | 'revert'
}

export function OwnershipTransferLifecycleDialog({ open, onOpenChange, currentRow, action }: OwnershipTransferLifecycleDialogProps) {
  const { t } = useTranslation(['documents'])
  return (
    <LifecycleDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      action={action}
      executeFn={ownershipTransferExecute}
      revertFn={ownershipTransferRevert}
      queryKey={ownershipTransferListQueryKey()}
      entityLabel={t('documents:ownershipTransfer.singular')}
    />
  )
}
