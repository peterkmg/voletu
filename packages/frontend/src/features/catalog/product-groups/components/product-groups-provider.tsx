import type { ProductGroupResponse } from '~/generated/types'
import { createEntityProvider } from '~/lib/create-entity-provider'

type ProductGroupsDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

export const { Provider: ProductGroupsProvider, useEntity: useProductGroups }
  = createEntityProvider<ProductGroupResponse, ProductGroupsDialogType>('ProductGroups')
