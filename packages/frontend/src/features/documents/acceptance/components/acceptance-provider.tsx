import type { AcceptanceResponse } from '~/generated/types'
import { createEntityProvider } from '~/lib/create-entity-provider'

type AcceptanceDialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

export const { Provider: AcceptanceProvider, useEntity: useAcceptance }
  = createEntityProvider<AcceptanceResponse, AcceptanceDialogType>('Acceptance')
