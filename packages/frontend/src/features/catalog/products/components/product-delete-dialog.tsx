import type { ProductResponse } from '~/generated/types'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { ConfirmDialog } from '~/components/confirm-dialog'
import { catalogProductHardDelete, catalogProductSoftDelete } from '~/generated/client'
import { catalogProductListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogProductList'
import { queryClient } from '~/shared/api/query-client'

interface ProductDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: ProductResponse | null
  variant: 'soft' | 'hard'
}

export function ProductDeleteDialog({
  open,
  onOpenChange,
  currentRow,
  variant,
}: ProductDeleteDialogProps) {
  const { t } = useTranslation(['common', 'catalog'])
  const [loading, setLoading] = useState(false)

  const handleConfirm = async () => {
    if (!currentRow)
      return
    setLoading(true)

    try {
      if (variant === 'hard') {
        await catalogProductHardDelete(currentRow.id)
      }
      else {
        await catalogProductSoftDelete(currentRow.id)
      }

      toast.success(
        t('common:toast.deleteSuccess', {
          entity: t('catalog:product.singular'),
        }),
      )
      await queryClient.invalidateQueries({ queryKey: catalogProductListQueryKey() })
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
