import type { TruckWaybillResponse } from '~/generated/types'
import { createEntityProvider } from '~/lib/create-entity-provider'

type TruckWaybillsDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

export const { Provider: TruckWaybillsProvider, useEntity: useTruckWaybills }
  = createEntityProvider<TruckWaybillResponse, TruckWaybillsDialogType>('TruckWaybills')
