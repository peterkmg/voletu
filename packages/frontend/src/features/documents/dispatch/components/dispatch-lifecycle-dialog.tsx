import type { DispatchResponse } from '~/generated/types'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { ConfirmDialog } from '~/components/confirm-dialog'
import { dispatchDocumentExecute, dispatchDocumentRevert } from '~/generated/client'
import { dispatchDocumentListQueryKey } from '~/generated/hooks/DocumentDispatchHooks/useDispatchDocumentList'
import { queryClient } from '~/shared/api/query-client'

interface DispatchLifecycleDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: DispatchResponse | null
  variant: 'execute' | 'revert'
}

export function DispatchLifecycleDialog({
  open,
  onOpenChange,
  currentRow,
  variant,
}: DispatchLifecycleDialogProps) {
  const { t } = useTranslation(['common', 'documents'])
  const [loading, setLoading] = useState(false)

  const handleConfirm = async () => {
    if (!currentRow)
      return
    setLoading(true)

    try {
      if (variant === 'execute') {
        await dispatchDocumentExecute(currentRow.id)
        toast.success(
          t('common:toast.executeSuccess', {
            entity: t('documents:dispatch.singular'),
          }),
        )
      }
      else {
        await dispatchDocumentRevert(currentRow.id)
        toast.success(
          t('common:toast.revertSuccess', {
            entity: t('documents:dispatch.singular'),
          }),
        )
      }

      await queryClient.invalidateQueries({ queryKey: dispatchDocumentListQueryKey() })
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
      variant={variant === 'execute' ? 'default' : 'destructive'}
      onConfirm={handleConfirm}
      loading={loading}
    />
  )
}
