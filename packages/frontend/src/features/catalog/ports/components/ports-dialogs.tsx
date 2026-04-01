import { catalogPortHardDelete, catalogPortSoftDelete } from '~/generated/client'
import { catalogPortListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogPortList'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { PortMutateDialog } from './port-mutate-dialog'
import { usePorts } from './ports-provider'

const PortDeleteDialog = createDeleteDialog({
  useEntity: usePorts,
  hardDeleteFn: catalogPortHardDelete,
  softDeleteFn: catalogPortSoftDelete,
  queryKey: catalogPortListQueryKey,
  entityLabel: 'catalog:port.singular',
  i18nNamespaces: ['common', 'catalog'],
})

export const PortsDialogs = createEntityDialogs({
  useEntity: usePorts,
  MutateDialog: PortMutateDialog,
  DeleteDialog: PortDeleteDialog,
})
