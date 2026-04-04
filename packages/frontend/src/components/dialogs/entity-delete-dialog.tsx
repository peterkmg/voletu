import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { queryClient } from '~/api/query-client'
import { ConfirmDialog } from '~/components/dialogs/confirm-dialog'
import { extractErrorMessage } from '~/lib/error'

interface EntityDeleteDialogProps<TRow extends { id: string }> {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: TRow | null
  hardDeleteFn: (id: string) => Promise<unknown>
  softDeleteFn?: (id: string) => Promise<unknown>
  variant?: 'soft' | 'hard'
  queryKey: readonly unknown[]
  entityLabel: string
}

export function EntityDeleteDialog<TRow extends { id: string }>({
  open,
  onOpenChange,
  currentRow,
  hardDeleteFn,
  softDeleteFn,
  variant = 'hard',
  queryKey,
  entityLabel,
}: EntityDeleteDialogProps<TRow>) {
  const { t } = useTranslation('common')
  const [loading, setLoading] = useState(false)

  const isHard = variant === 'hard' || !softDeleteFn

  const handleConfirm = async () => {
    if (!currentRow)
      return
    setLoading(true)

    try {
      const deleteFn = isHard || !softDeleteFn ? hardDeleteFn : softDeleteFn
      await deleteFn(currentRow.id)

      toast.success(
        t(isHard ? 'toast.deleteSuccess' : 'toast.archiveSuccess', {
          entity: entityLabel,
        }),
      )
      await queryClient.invalidateQueries({ queryKey })
      onOpenChange(false)
    }
    catch (err) {
      toast.error(extractErrorMessage(err, t('toast.error')))
    }
    finally {
      setLoading(false)
    }
  }

  const confirmLabel = softDeleteFn
    ? t(isHard ? 'actions.hardDelete' : 'actions.softDelete')
    : t('actions.delete')

  return (
    <ConfirmDialog
      open={open}
      onOpenChange={onOpenChange}
      title={t(isHard ? 'confirm.deleteTitle' : 'confirm.archiveTitle')}
      description={t(isHard ? 'confirm.deleteDescription' : 'confirm.archiveDescription')}
      confirmLabel={confirmLabel}
      cancelLabel={t('actions.cancel')}
      variant="destructive"
      onConfirm={handleConfirm}
      loading={loading}
    />
  )
}
