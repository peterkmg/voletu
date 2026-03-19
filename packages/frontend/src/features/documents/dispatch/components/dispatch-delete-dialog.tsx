import type { DispatchResponse } from '~/generated/types'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { ConfirmDialog } from '~/components/confirm-dialog'
import {
  hardDeleteDispatchDocument,
  invalidateDispatchDocuments,
  softDeleteDispatchDocument,
} from '../data/dispatch-api'

interface DispatchDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: DispatchResponse | null
  variant: 'soft' | 'hard'
}

export function DispatchDeleteDialog({
  open,
  onOpenChange,
  currentRow,
  variant,
}: DispatchDeleteDialogProps) {
  const { t } = useTranslation(['common', 'documents'])
  const [loading, setLoading] = useState(false)

  const handleConfirm = async () => {
    if (!currentRow)
      return
    setLoading(true)

    try {
      if (variant === 'hard') {
        await hardDeleteDispatchDocument(currentRow.id)
      }
      else {
        await softDeleteDispatchDocument(currentRow.id)
      }

      toast.success(
        t('common:toast.deleteSuccess', {
          entity: t('documents:dispatch.singular'),
        }),
      )
      await invalidateDispatchDocuments()
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
        variant === 'hard'
          ? t('common:confirm.deleteTitle')
          : t('common:confirm.archiveTitle')
      }
      description={
        variant === 'hard'
          ? t('common:confirm.deleteDescription')
          : t('common:confirm.archiveDescription')
      }
      confirmLabel={
        variant === 'hard'
          ? t('common:actions.hardDelete')
          : t('common:actions.softDelete')
      }
      cancelLabel={t('common:actions.cancel')}
      variant="destructive"
      onConfirm={handleConfirm}
      loading={loading}
    />
  )
}
