import type { RailWaybillResponse } from '~/generated/types'
import { createEntityProvider } from '~/lib/create-entity-provider'

type RailWaybillsDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

export const { Provider: RailWaybillsProvider, useEntity: useRailWaybills }
  = createEntityProvider<RailWaybillResponse, RailWaybillsDialogType>('RailWaybills')
