import type { CompanyResponse } from '~/generated/types'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { ConfirmDialog } from '~/components/confirm-dialog'
import { catalogCompanyHardDelete, catalogCompanySoftDelete } from '~/generated/client'
import { catalogCompanyListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { queryClient } from '~/shared/api/query-client'

interface CompanyDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: CompanyResponse | null
  variant: 'soft' | 'hard'
}

export function CompanyDeleteDialog({
  open,
  onOpenChange,
  currentRow,
  variant,
}: CompanyDeleteDialogProps) {
  const { t } = useTranslation(['common', 'catalog'])
  const [loading, setLoading] = useState(false)

  const handleConfirm = async () => {
    if (!currentRow)
      return
    setLoading(true)

    try {
      if (variant === 'hard') {
        await catalogCompanyHardDelete(currentRow.id)
      }
      else {
        await catalogCompanySoftDelete(currentRow.id)
      }

      toast.success(
        t('common:toast.deleteSuccess', {
          entity: t('catalog:company.singular'),
        }),
      )
      await queryClient.invalidateQueries({ queryKey: catalogCompanyListQueryKey() })
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
