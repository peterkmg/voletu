import { catalogBaseHardDelete, catalogBaseSoftDelete } from '~/generated/client'
import { catalogBaseListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogBaseList'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { BaseMutateDialog } from './base-mutate-dialog'
import { useBases } from './bases-provider'

const BaseDeleteDialog = createDeleteDialog({
  useEntity: useBases,
  hardDeleteFn: catalogBaseHardDelete,
  softDeleteFn: catalogBaseSoftDelete,
  queryKey: catalogBaseListQueryKey,
  entityLabel: 'catalog:base.singular',
  i18nNamespaces: ['common', 'catalog'],
})

export const BasesDialogs = createEntityDialogs({
  useEntity: useBases,
  MutateDialog: BaseMutateDialog,
  DeleteDialog: BaseDeleteDialog,
})
