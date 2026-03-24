import type { WarehouseResponse } from '~/generated/types'
import { createRowActions } from '~/lib/create-row-actions'
import { useWarehouses } from './warehouses-provider'

export const DataTableRowActions = createRowActions<WarehouseResponse>({
  useEntity: useWarehouses,
})
