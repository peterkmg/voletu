import type { BaseResponse } from '~/generated/types/BaseResponse'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { ConfirmDialog } from '~/components/confirm-dialog'
import { catalogBaseHardDelete, catalogBaseSoftDelete } from '~/generated/client'
import { catalogBaseListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogBaseList'
import { queryClient } from '~/shared/api/query-client'

interface BaseDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: BaseResponse | null
  variant: 'soft' | 'hard'
}

export function BaseDeleteDialog({
  open,
  onOpenChange,
  currentRow,
  variant,
}: BaseDeleteDialogProps) {
  const { t } = useTranslation(['common', 'catalog'])
  const [loading, setLoading] = useState(false)

  const handleConfirm = async () => {
    if (!currentRow)
      return
    setLoading(true)

    try {
      if (variant === 'hard') {
        await catalogBaseHardDelete(currentRow.id)
      }
      else {
        await catalogBaseSoftDelete(currentRow.id)
      }

      toast.success(
        t('common:toast.deleteSuccess', {
          entity: t('catalog:base.singular'),
        }),
      )
      await queryClient.invalidateQueries({ queryKey: catalogBaseListQueryKey() })
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
