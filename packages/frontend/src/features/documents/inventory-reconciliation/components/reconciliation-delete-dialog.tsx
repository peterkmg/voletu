import type { InventoryReconciliationResponse } from '~/generated/types'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { ConfirmDialog } from '~/components/confirm-dialog'
import { reconciliationHardDelete, reconciliationSoftDelete } from '~/generated/client'
import { reconciliationListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/useReconciliationList'
import { queryClient } from '~/shared/api/query-client'

interface ReconciliationDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: InventoryReconciliationResponse | null
  variant: 'soft' | 'hard'
}

export function ReconciliationDeleteDialog({
  open,
  onOpenChange,
  currentRow,
  variant,
}: ReconciliationDeleteDialogProps) {
  const { t } = useTranslation(['common', 'documents'])
  const [loading, setLoading] = useState(false)

  const handleConfirm = async () => {
    if (!currentRow)
      return
    setLoading(true)

    try {
      if (variant === 'hard') {
        await reconciliationHardDelete(currentRow.id)
      }
      else {
        await reconciliationSoftDelete(currentRow.id)
      }

      toast.success(
        t('common:toast.deleteSuccess', {
          entity: t('documents:reconciliation.singular'),
        }),
      )
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
