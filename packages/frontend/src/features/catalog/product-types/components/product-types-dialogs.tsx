import { catalogProductTypeHardDelete, catalogProductTypeSoftDelete } from '~/generated/client'
import { catalogProductTypeListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogProductTypeList'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { ProductTypeMutateDialog } from './product-type-mutate-dialog'
import { useProductTypes } from './product-types-provider'

const ProductTypeDeleteDialog = createDeleteDialog({
  useEntity: useProductTypes,
  hardDeleteFn: catalogProductTypeHardDelete,
  softDeleteFn: catalogProductTypeSoftDelete,
  queryKey: catalogProductTypeListQueryKey,
  entityLabel: 'catalog:productType.singular',
  i18nNamespaces: ['common', 'catalog'],
})

export const ProductTypesDialogs = createEntityDialogs({
  useEntity: useProductTypes,
  MutateDialog: ProductTypeMutateDialog,
  DeleteDialog: ProductTypeDeleteDialog,
})
