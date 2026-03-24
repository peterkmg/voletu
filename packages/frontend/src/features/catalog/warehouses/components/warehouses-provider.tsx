import type { WarehouseResponse } from '~/generated/types'
import { createEntityProvider } from '~/lib/create-entity-provider'

type WarehousesDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

export const { Provider: WarehousesProvider, useEntity: useWarehouses }
  = createEntityProvider<WarehouseResponse, WarehousesDialogType>('Warehouses')
