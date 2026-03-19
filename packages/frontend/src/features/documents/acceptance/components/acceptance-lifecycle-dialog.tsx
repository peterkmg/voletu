import type { AcceptanceResponse } from '~/generated/types'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { ConfirmDialog } from '~/components/confirm-dialog'
import {
  executeAcceptanceDocument,
  invalidateAcceptanceDocuments,
  revertAcceptanceDocument,
} from '../data/acceptance-api'

interface AcceptanceLifecycleDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: AcceptanceResponse | null
  action: 'execute' | 'revert'
}

export function AcceptanceLifecycleDialog({
  open,
  onOpenChange,
  currentRow,
  action,
}: AcceptanceLifecycleDialogProps) {
  const { t } = useTranslation(['common', 'documents'])
  const [loading, setLoading] = useState(false)

  const handleConfirm = async () => {
    if (!currentRow)
      return
    setLoading(true)

    try {
      if (action === 'execute') {
        await executeAcceptanceDocument(currentRow.id)
        toast.success(
          t('common:toast.executeSuccess', {
            entity: t('documents:acceptance.singular'),
          }),
        )
      }
      else {
        await revertAcceptanceDocument(currentRow.id)
        toast.success(
          t('common:toast.revertSuccess', {
            entity: t('documents:acceptance.singular'),
          }),
        )
      }

      await invalidateAcceptanceDocuments()
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
        action === 'execute'
          ? t('documents:lifecycle.execute')
          : t('documents:lifecycle.revert')
      }
      description={
        action === 'execute'
          ? t('documents:lifecycle.executeConfirm')
          : t('documents:lifecycle.revertConfirm')
      }
      confirmLabel={
        action === 'execute'
          ? t('common:actions.execute')
          : t('common:actions.revert')
      }
      cancelLabel={t('common:actions.cancel')}
      onConfirm={handleConfirm}
      loading={loading}
    />
  )
}
