import type { BlendingResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { LifecycleDialog } from '~/components/lifecycle-dialog'
import { blendingDocumentExecute, blendingDocumentRevert } from '~/generated/client'
import { blendingDocumentListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/useBlendingDocumentList'

interface BlendingLifecycleDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: BlendingResponse | null
  variant: 'execute' | 'revert'
}

export function BlendingLifecycleDialog({ open, onOpenChange, currentRow, variant }: BlendingLifecycleDialogProps) {
  const { t } = useTranslation(['documents'])
  return (
    <LifecycleDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      action={variant}
      executeFn={blendingDocumentExecute}
      revertFn={blendingDocumentRevert}
      queryKey={blendingDocumentListQueryKey()}
      entityLabel={t('documents:blending.singular')}
    />
  )
}
