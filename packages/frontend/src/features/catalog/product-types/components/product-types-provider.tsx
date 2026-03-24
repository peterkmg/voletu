import type { ProductTypeResponse } from '~/generated/types'
import { createEntityProvider } from '~/lib/create-entity-provider'

type ProductTypesDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

export const { Provider: ProductTypesProvider, useEntity: useProductTypes }
  = createEntityProvider<ProductTypeResponse, ProductTypesDialogType>('ProductTypes')
