import type { DispatchResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { LifecycleDialog } from '~/components/lifecycle-dialog'
import { dispatchDocumentExecute, dispatchDocumentRevert } from '~/generated/client'
import { dispatchDocumentListQueryKey } from '~/generated/hooks/DocumentDispatchHooks/useDispatchDocumentList'

interface DispatchLifecycleDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: DispatchResponse | null
  variant: 'execute' | 'revert'
}

export function DispatchLifecycleDialog({ open, onOpenChange, currentRow, variant }: DispatchLifecycleDialogProps) {
  const { t } = useTranslation(['documents'])
  return (
    <LifecycleDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      action={variant}
      executeFn={dispatchDocumentExecute}
      revertFn={dispatchDocumentRevert}
      queryKey={dispatchDocumentListQueryKey()}
      entityLabel={t('documents:dispatch.singular')}
    />
  )
}
