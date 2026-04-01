import { catalogProductHardDelete, catalogProductSoftDelete } from '~/generated/client'
import { catalogProductListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogProductList'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { ProductMutateDialog } from './product-mutate-dialog'
import { useProducts } from './products-provider'

const ProductDeleteDialog = createDeleteDialog({
  useEntity: useProducts,
  hardDeleteFn: catalogProductHardDelete,
  softDeleteFn: catalogProductSoftDelete,
  queryKey: catalogProductListQueryKey,
  entityLabel: 'catalog:product.singular',
  i18nNamespaces: ['common', 'catalog'],
})

export const ProductsDialogs = createEntityDialogs({
  useEntity: useProducts,
  MutateDialog: ProductMutateDialog,
  DeleteDialog: ProductDeleteDialog,
})
