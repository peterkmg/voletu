import type { RailWaybillResponse } from '~/generated/types'
import { createRowActions } from '~/lib/create-row-actions'
import { useRailWaybills } from './rail-waybills-provider'

export const DataTableRowActions = createRowActions<RailWaybillResponse>({
  useEntity: useRailWaybills,
})
