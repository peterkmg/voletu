import { catalogProductGroupHardDelete, catalogProductGroupSoftDelete } from '~/generated/client'
import { catalogProductGroupListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogProductGroupList'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { ProductGroupMutateDialog } from './product-group-mutate-dialog'
import { useProductGroups } from './product-groups-provider'

const ProductGroupDeleteDialog = createDeleteDialog({
  useEntity: useProductGroups,
  hardDeleteFn: catalogProductGroupHardDelete,
  softDeleteFn: catalogProductGroupSoftDelete,
  queryKey: catalogProductGroupListQueryKey,
  entityLabel: 'catalog:productGroup.singular',
  i18nNamespaces: ['common', 'catalog'],
})

export const ProductGroupsDialogs = createEntityDialogs({
  useEntity: useProductGroups,
  MutateDialog: ProductGroupMutateDialog,
  DeleteDialog: ProductGroupDeleteDialog,
})
