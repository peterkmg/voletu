import type { ProductResponse } from '~/generated/types'
import { createEntityProvider } from '~/lib/create-entity-provider'

type ProductsDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

export const { Provider: ProductsProvider, useEntity: useProducts }
  = createEntityProvider<ProductResponse, ProductsDialogType>('Products')
