import type { CompanyResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { EntityDeleteDialog } from '~/components/entity-delete-dialog'
import { catalogCompanyHardDelete, catalogCompanySoftDelete } from '~/generated/client'
import { catalogCompanyListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'

interface CompanyDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: CompanyResponse | null
  variant: 'soft' | 'hard'
}

export function CompanyDeleteDialog({ open, onOpenChange, currentRow, variant }: CompanyDeleteDialogProps) {
  const { t } = useTranslation(['common', 'catalog'])
  return (
    <EntityDeleteDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      variant={variant}
      hardDeleteFn={catalogCompanyHardDelete}
      softDeleteFn={catalogCompanySoftDelete}
      queryKey={catalogCompanyListQueryKey()}
      entityLabel={t('catalog:company.singular')}
    />
  )
}
