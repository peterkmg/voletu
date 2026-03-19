import type { OwnershipTransferResponse } from '~/generated/types'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { ConfirmDialog } from '~/components/confirm-dialog'
import { ownershipTransferExecute, ownershipTransferRevert } from '~/generated/client'
import { ownershipTransferListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/useOwnershipTransferList'
import { queryClient } from '~/shared/api/query-client'

interface OwnershipTransferLifecycleDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: OwnershipTransferResponse | null
  action: 'execute' | 'revert'
}

export function OwnershipTransferLifecycleDialog({
  open,
  onOpenChange,
  currentRow,
  action,
}: OwnershipTransferLifecycleDialogProps) {
  const { t } = useTranslation(['common', 'documents'])
  const [loading, setLoading] = useState(false)

  const handleConfirm = async () => {
    if (!currentRow)
      return
    setLoading(true)

    try {
      if (action === 'execute') {
        await ownershipTransferExecute(currentRow.id)
        toast.success(
          t('common:toast.executeSuccess', {
            entity: t('documents:ownershipTransfer.singular'),
          }),
        )
      }
      else {
        await ownershipTransferRevert(currentRow.id)
        toast.success(
          t('common:toast.revertSuccess', {
            entity: t('documents:ownershipTransfer.singular'),
          }),
        )
      }

      await queryClient.invalidateQueries({ queryKey: ownershipTransferListQueryKey() })
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
