import type { InventoryReconciliationResponse } from '~/generated/types'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { ConfirmDialog } from '~/components/confirm-dialog'
import {
  executeReconciliationDocument,
  invalidateReconciliations,
  revertReconciliationDocument,
} from '../data/reconciliation-api'

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
        await executeReconciliationDocument(currentRow.id)
        toast.success(
          t('common:toast.executeSuccess', {
            entity: t('documents:reconciliation.singular'),
          }),
        )
      }
      else {
        await revertReconciliationDocument(currentRow.id)
        toast.success(
          t('common:toast.revertSuccess', {
            entity: t('documents:reconciliation.singular'),
          }),
        )
      }

      await invalidateReconciliations()
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
