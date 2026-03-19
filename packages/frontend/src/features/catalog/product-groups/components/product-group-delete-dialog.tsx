import type { ProductGroupResponse } from '~/generated/types'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { ConfirmDialog } from '~/components/confirm-dialog'
import { catalogProductGroupHardDelete, catalogProductGroupSoftDelete } from '~/generated/client'
import { catalogProductGroupListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogProductGroupList'
import { queryClient } from '~/shared/api/query-client'

interface ProductGroupDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: ProductGroupResponse | null
  variant: 'soft' | 'hard'
}

export function ProductGroupDeleteDialog({
  open,
  onOpenChange,
  currentRow,
  variant,
}: ProductGroupDeleteDialogProps) {
  const { t } = useTranslation(['common', 'catalog'])
  const [loading, setLoading] = useState(false)

  const handleConfirm = async () => {
    if (!currentRow)
      return
    setLoading(true)

    try {
      if (variant === 'hard') {
        await catalogProductGroupHardDelete(currentRow.id)
      }
      else {
        await catalogProductGroupSoftDelete(currentRow.id)
      }

      toast.success(
        t('common:toast.deleteSuccess', {
          entity: t('catalog:productGroup.singular'),
        }),
      )
      await queryClient.invalidateQueries({ queryKey: catalogProductGroupListQueryKey() })
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
