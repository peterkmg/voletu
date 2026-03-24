import type { BlendingResponse } from '~/generated/types'
import { createEntityProvider } from '~/lib/create-entity-provider'

type BlendingDialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

export const { Provider: BlendingProvider, useEntity: useBlending }
  = createEntityProvider<BlendingResponse, BlendingDialogType>('Blending')
