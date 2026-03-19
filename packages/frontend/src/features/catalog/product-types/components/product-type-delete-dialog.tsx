import type { ProductTypeResponse } from '~/generated/types'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { ConfirmDialog } from '~/components/confirm-dialog'
import { catalogProductTypeHardDelete, catalogProductTypeSoftDelete } from '~/generated/client'
import { catalogProductTypeListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogProductTypeList'
import { queryClient } from '~/shared/api/query-client'

interface ProductTypeDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: ProductTypeResponse | null
  variant: 'soft' | 'hard'
}

export function ProductTypeDeleteDialog({
  open,
  onOpenChange,
  currentRow,
  variant,
}: ProductTypeDeleteDialogProps) {
  const { t } = useTranslation(['common', 'catalog'])
  const [loading, setLoading] = useState(false)

  const handleConfirm = async () => {
    if (!currentRow)
      return
    setLoading(true)

    try {
      if (variant === 'hard') {
        await catalogProductTypeHardDelete(currentRow.id)
      }
      else {
        await catalogProductTypeSoftDelete(currentRow.id)
      }

      toast.success(
        t('common:toast.deleteSuccess', {
          entity: t('catalog:productType.singular'),
        }),
      )
      await queryClient.invalidateQueries({ queryKey: catalogProductTypeListQueryKey() })
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
