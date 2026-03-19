import {
  catalogWarehouseCreate,
  catalogWarehouseHardDelete,
  catalogWarehouseSoftDelete,
  catalogWarehouseUpdate,
} from '~/generated/client'
import { catalogWarehouseListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogWarehouseList'
import { queryClient } from '~/shared/api/query-client'

export type { CreateWarehouseRequest } from '~/generated/types/CreateWarehouseRequest'
export type { UpdateWarehouseRequest } from '~/generated/types/UpdateWarehouseRequest'
export type { WarehouseResponse } from '~/generated/types/WarehouseResponse'

export const createWarehouse = catalogWarehouseCreate
export const updateWarehouse = catalogWarehouseUpdate
export const softDeleteWarehouse = catalogWarehouseSoftDelete
export const hardDeleteWarehouse = catalogWarehouseHardDelete

export function invalidateWarehouses() {
  return queryClient.invalidateQueries({ queryKey: catalogWarehouseListQueryKey() })
}
