import {
  catalogPortCreate,
  catalogPortHardDelete,
  catalogPortSoftDelete,
  catalogPortUpdate,
} from '~/generated/client'
import { catalogPortListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogPortList'
import { queryClient } from '~/shared/api/query-client'

export type { CreatePortRequest } from '~/generated/types/CreatePortRequest'
export type { PortResponse } from '~/generated/types/PortResponse'
export type { UpdatePortRequest } from '~/generated/types/UpdatePortRequest'

export const createPort = catalogPortCreate
export const updatePort = catalogPortUpdate
export const softDeletePort = catalogPortSoftDelete
export const hardDeletePort = catalogPortHardDelete

export function invalidatePorts() {
  return queryClient.invalidateQueries({ queryKey: catalogPortListQueryKey() })
}
