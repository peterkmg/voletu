import {
  catalogStorageCreate,
  catalogStorageHardDelete,
  catalogStorageSoftDelete,
  catalogStorageUpdate,
} from '~/generated/client'
import { catalogStorageListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogStorageList'
import { queryClient } from '~/shared/api/query-client'

export type { CreateStorageRequest } from '~/generated/types/CreateStorageRequest'
export type { StorageResponse } from '~/generated/types/StorageResponse'
export type { UpdateStorageRequest } from '~/generated/types/UpdateStorageRequest'

export const createStorage = catalogStorageCreate
export const updateStorage = catalogStorageUpdate
export const softDeleteStorage = catalogStorageSoftDelete
export const hardDeleteStorage = catalogStorageHardDelete

export function invalidateStorages() {
  return queryClient.invalidateQueries({ queryKey: catalogStorageListQueryKey() })
}
