import type { AcceptanceResponse } from '~/generated/types'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { ConfirmDialog } from '~/components/confirm-dialog'
import { acceptanceDocumentHardDelete, acceptanceDocumentSoftDelete } from '~/generated/client'
import { acceptanceDocumentListQueryKey } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceDocumentList'
import { queryClient } from '~/shared/api/query-client'

interface AcceptanceDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: AcceptanceResponse | null
  variant: 'soft' | 'hard'
}

export function AcceptanceDeleteDialog({
  open,
  onOpenChange,
  currentRow,
  variant,
}: AcceptanceDeleteDialogProps) {
  const { t } = useTranslation(['common', 'documents'])
  const [loading, setLoading] = useState(false)

  const handleConfirm = async () => {
    if (!currentRow)
      return
    setLoading(true)

    try {
      if (variant === 'hard') {
        await acceptanceDocumentHardDelete(currentRow.id)
      }
      else {
        await acceptanceDocumentSoftDelete(currentRow.id)
      }

      toast.success(
        t('common:toast.deleteSuccess', {
          entity: t('documents:acceptance.singular'),
        }),
      )
      await queryClient.invalidateQueries({ queryKey: acceptanceDocumentListQueryKey() })
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
