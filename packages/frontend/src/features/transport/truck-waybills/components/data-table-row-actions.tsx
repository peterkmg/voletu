import type { TruckWaybillResponse } from '~/generated/types'
import { createRowActions } from '~/lib/create-row-actions'
import { useTruckWaybills } from './truck-waybills-provider'

export const DataTableRowActions = createRowActions<TruckWaybillResponse>({
  useEntity: useTruckWaybills,
})
