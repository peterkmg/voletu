import type { ProductTypeResponse } from '~/generated/types'
import { createRowActions } from '~/lib/create-row-actions'
import { useProductTypes } from './product-types-provider'

export const DataTableRowActions = createRowActions<ProductTypeResponse>({
  useEntity: useProductTypes,
})
