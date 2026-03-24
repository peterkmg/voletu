import type { DispatchResponse } from '~/generated/types'
import { createEntityProvider } from '~/lib/create-entity-provider'

type DispatchDialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

export const { Provider: DispatchProvider, useEntity: useDispatch }
  = createEntityProvider<DispatchResponse, DispatchDialogType>('Dispatch')
