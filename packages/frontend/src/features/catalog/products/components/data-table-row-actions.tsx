import type { ProductResponse } from '~/generated/types'
import { createRowActions } from '~/lib/create-row-actions'
import { useProducts } from './products-provider'

export const DataTableRowActions = createRowActions<ProductResponse>({
  useEntity: useProducts,
})
