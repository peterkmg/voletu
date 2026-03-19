import type { BlendingResponse } from '~/generated/types'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { ConfirmDialog } from '~/components/confirm-dialog'
import { blendingDocumentExecute, blendingDocumentRevert } from '~/generated/client'
import { blendingDocumentListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/useBlendingDocumentList'
import { queryClient } from '~/shared/api/query-client'

interface BlendingLifecycleDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: BlendingResponse | null
  variant: 'execute' | 'revert'
}

export function BlendingLifecycleDialog({
  open,
  onOpenChange,
  currentRow,
  variant,
}: BlendingLifecycleDialogProps) {
  const { t } = useTranslation(['common', 'documents'])
  const [loading, setLoading] = useState(false)

  const handleConfirm = async () => {
    if (!currentRow)
      return
    setLoading(true)

    try {
      if (variant === 'execute') {
        await blendingDocumentExecute(currentRow.id)
        toast.success(
          t('common:toast.executeSuccess', {
            entity: t('documents:blending.singular'),
          }),
        )
      }
      else {
        await blendingDocumentRevert(currentRow.id)
        toast.success(
          t('common:toast.revertSuccess', {
            entity: t('documents:blending.singular'),
          }),
        )
      }

      await queryClient.invalidateQueries({ queryKey: blendingDocumentListQueryKey() })
      onOpenChange(false)
    }
    catch (err) {
      toast.error(err instanceof Error ? err.message : t('common:toast.error'))
    }
    finally {
      setLoading(false)
    }
  }

  return (
    <ConfirmDialog
      open={open}
      onOpenChange={onOpenChange}
      title={
        variant === 'execute'
          ? t('documents:lifecycle.execute')
          : t('documents:lifecycle.revert')
      }
      description={
        variant === 'execute'
          ? t('documents:lifecycle.executeConfirm')
          : t('documents:lifecycle.revertConfirm')
      }
      confirmLabel={
        variant === 'execute'
          ? t('common:actions.execute')
          : t('common:actions.revert')
      }
      cancelLabel={t('common:actions.cancel')}
      onConfirm={handleConfirm}
      loading={loading}
    />
  )
}
