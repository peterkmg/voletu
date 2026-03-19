import type { InventoryReconciliationResponse } from '~/generated/types'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { ConfirmDialog } from '~/components/confirm-dialog'
import { reconciliationExecute, reconciliationRevert } from '~/generated/client'
import { reconciliationListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/useReconciliationList'
import { queryClient } from '~/shared/api/query-client'

interface ReconciliationLifecycleDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: InventoryReconciliationResponse | null
  variant: 'execute' | 'revert'
}

export function ReconciliationLifecycleDialog({
  open,
  onOpenChange,
  currentRow,
  variant,
}: ReconciliationLifecycleDialogProps) {
  const { t } = useTranslation(['common', 'documents'])
  const [loading, setLoading] = useState(false)

  const handleConfirm = async () => {
    if (!currentRow)
      return
    setLoading(true)

    try {
      if (variant === 'execute') {
        await reconciliationExecute(currentRow.id)
        toast.success(
          t('common:toast.executeSuccess', {
            entity: t('documents:reconciliation.singular'),
          }),
        )
      }
      else {
        await reconciliationRevert(currentRow.id)
        toast.success(
          t('common:toast.revertSuccess', {
            entity: t('documents:reconciliation.singular'),
          }),
        )
      }

      await queryClient.invalidateQueries({ queryKey: reconciliationListQueryKey() })
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
      variant={variant === 'revert' ? 'destructive' : 'default'}
      onConfirm={handleConfirm}
      loading={loading}
    />
  )
}
