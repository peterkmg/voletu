import type { AcceptanceResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { LifecycleDialog } from '~/components/lifecycle-dialog'
import { acceptanceDocumentExecute, acceptanceDocumentRevert } from '~/generated/client'
import { acceptanceDocumentListQueryKey } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceDocumentList'

interface AcceptanceLifecycleDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: AcceptanceResponse | null
  action: 'execute' | 'revert'
}

export function AcceptanceLifecycleDialog({ open, onOpenChange, currentRow, action }: AcceptanceLifecycleDialogProps) {
  const { t } = useTranslation(['documents'])
  return (
    <LifecycleDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      action={action}
      executeFn={acceptanceDocumentExecute}
      revertFn={acceptanceDocumentRevert}
      queryKey={acceptanceDocumentListQueryKey()}
      entityLabel={t('documents:acceptance.singular')}
    />
  )
}
