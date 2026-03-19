import type { RailWaybillResponse } from '~/generated/types'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { ConfirmDialog } from '~/components/confirm-dialog'
import { transportRailWaybillHardDelete, transportRailWaybillSoftDelete } from '~/generated/client'
import { transportRailWaybillListQueryKey } from '~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillList'
import { queryClient } from '~/shared/api/query-client'

interface RailWaybillDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: RailWaybillResponse | null
  variant: 'soft' | 'hard'
}

export function RailWaybillDeleteDialog({
  open,
  onOpenChange,
  currentRow,
  variant,
}: RailWaybillDeleteDialogProps) {
  const { t } = useTranslation(['common', 'transport'])
  const [loading, setLoading] = useState(false)

  const handleConfirm = async () => {
    if (!currentRow)
      return
    setLoading(true)

    try {
      if (variant === 'hard') {
        await transportRailWaybillHardDelete(currentRow.id)
      }
      else {
        await transportRailWaybillSoftDelete(currentRow.id)
      }

      toast.success(
        t('common:toast.deleteSuccess', {
          entity: t('transport:rail.singular'),
        }),
      )
      await queryClient.invalidateQueries({ queryKey: transportRailWaybillListQueryKey() })
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
