import { catalogWarehouseHardDelete, catalogWarehouseSoftDelete } from '~/generated/client'
import { catalogWarehouseListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogWarehouseList'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { WarehouseMutateDialog } from './warehouse-mutate-dialog'
import { useWarehouses } from './warehouses-provider'

const WarehouseDeleteDialog = createDeleteDialog({
  useEntity: useWarehouses,
  hardDeleteFn: catalogWarehouseHardDelete,
  softDeleteFn: catalogWarehouseSoftDelete,
  queryKey: catalogWarehouseListQueryKey,
  entityLabel: 'catalog:warehouse.singular',
  i18nNamespaces: ['common', 'catalog'],
})

export const WarehousesDialogs = createEntityDialogs({
  useEntity: useWarehouses,
  MutateDialog: WarehouseMutateDialog,
  DeleteDialog: WarehouseDeleteDialog,
})
