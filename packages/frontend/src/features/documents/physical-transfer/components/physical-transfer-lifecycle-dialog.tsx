import type { PhysicalTransferResponse } from '~/generated/types'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { ConfirmDialog } from '~/components/confirm-dialog'
import {
  executePhysicalTransfer,
  invalidatePhysicalTransfers,
  revertPhysicalTransfer,
} from '../data/physical-transfer-api'

interface PhysicalTransferLifecycleDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: PhysicalTransferResponse | null
  action: 'execute' | 'revert'
}

export function PhysicalTransferLifecycleDialog({
  open,
  onOpenChange,
  currentRow,
  action,
}: PhysicalTransferLifecycleDialogProps) {
  const { t } = useTranslation(['common', 'documents'])
  const [loading, setLoading] = useState(false)

  const handleConfirm = async () => {
    if (!currentRow)
      return
    setLoading(true)

    try {
      if (action === 'execute') {
        await executePhysicalTransfer(currentRow.id)
        toast.success(
          t('common:toast.executeSuccess', {
            entity: t('documents:physicalTransfer.singular'),
          }),
        )
      }
      else {
        await revertPhysicalTransfer(currentRow.id)
        toast.success(
          t('common:toast.revertSuccess', {
            entity: t('documents:physicalTransfer.singular'),
          }),
        )
      }

      await invalidatePhysicalTransfers()
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
