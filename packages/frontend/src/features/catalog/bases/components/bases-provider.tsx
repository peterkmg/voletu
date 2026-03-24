import type { BaseResponse } from '~/generated/types/BaseResponse'
import { createEntityProvider } from '~/lib/create-entity-provider'

type BasesDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

export const { Provider: BasesProvider, useEntity: useBases }
  = createEntityProvider<BaseResponse, BasesDialogType>('Bases')
