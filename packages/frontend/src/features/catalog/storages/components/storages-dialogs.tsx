import { catalogStorageHardDelete, catalogStorageSoftDelete } from '~/generated/client'
import { catalogStorageListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogStorageList'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { StorageMutateDialog } from './storage-mutate-dialog'
import { useStorages } from './storages-provider'

const StorageDeleteDialog = createDeleteDialog({
  useEntity: useStorages,
  hardDeleteFn: catalogStorageHardDelete,
  softDeleteFn: catalogStorageSoftDelete,
  queryKey: catalogStorageListQueryKey,
  entityLabel: 'catalog:storage.singular',
  i18nNamespaces: ['common', 'catalog'],
})

export const StoragesDialogs = createEntityDialogs({
  useEntity: useStorages,
  MutateDialog: StorageMutateDialog,
  DeleteDialog: StorageDeleteDialog,
})
