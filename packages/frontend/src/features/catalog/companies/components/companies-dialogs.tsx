import { catalogCompanyHardDelete, catalogCompanySoftDelete } from '~/generated/client'
import { catalogCompanyListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { useCompanies } from './companies-provider'
import { CompanyMutateDialog } from './company-mutate-dialog'

const CompanyDeleteDialog = createDeleteDialog({
  useEntity: useCompanies,
  hardDeleteFn: catalogCompanyHardDelete,
  softDeleteFn: catalogCompanySoftDelete,
  queryKey: catalogCompanyListQueryKey,
  entityLabel: 'catalog:company.singular',
  i18nNamespaces: ['common', 'catalog'],
})

export const CompaniesDialogs = createEntityDialogs({
  useEntity: useCompanies,
  MutateDialog: CompanyMutateDialog,
  DeleteDialog: CompanyDeleteDialog,
})
