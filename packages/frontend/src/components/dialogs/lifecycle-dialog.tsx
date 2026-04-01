import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { ConfirmDialog } from '~/components/dialogs/confirm-dialog'
import { extractErrorMessage } from '~/lib/error'
import { queryClient } from '~/api/query-client'

interface LifecycleDialogProps<TRow extends { id: string }> {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: TRow | null
  action: 'execute' | 'revert'
  executeFn: (id: string) => Promise<unknown>
  revertFn: (id: string) => Promise<unknown>
  queryKey: readonly unknown[]
  entityLabel: string
}

export function LifecycleDialog<TRow extends { id: string }>({
  open,
  onOpenChange,
  currentRow,
  action,
  executeFn,
  revertFn,
  queryKey,
  entityLabel,
}: LifecycleDialogProps<TRow>) {
  const { t } = useTranslation(['common', 'documents'])
  const [loading, setLoading] = useState(false)

  const handleConfirm = async () => {
    if (!currentRow)
      return
    setLoading(true)

    try {
      const fn = action === 'execute' ? executeFn : revertFn
      await fn(currentRow.id)

      toast.success(
        t(
          action === 'execute'
            ? 'common:toast.executeSuccess'
            : 'common:toast.revertSuccess',
          { entity: entityLabel },
        ),
      )
      await queryClient.invalidateQueries({ queryKey })
      onOpenChange(false)
    }
    catch (err) {
      toast.error(extractErrorMessage(err, t('common:toast.error')))
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
      variant={action === 'revert' ? 'destructive' : 'default'}
      onConfirm={handleConfirm}
      loading={loading}
    />
  )
}
