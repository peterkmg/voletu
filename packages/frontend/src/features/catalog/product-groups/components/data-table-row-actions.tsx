import type { ProductGroupResponse } from '~/generated/types'
import { createRowActions } from '~/lib/create-row-actions'
import { useProductGroups } from './product-groups-provider'

export const DataTableRowActions = createRowActions<ProductGroupResponse>({
  useEntity: useProductGroups,
})
