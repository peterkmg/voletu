import type { PortResponse } from '~/generated/types'
import { createEntityProvider } from '~/lib/create-entity-provider'

type PortsDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

export const { Provider: PortsProvider, useEntity: usePorts }
  = createEntityProvider<PortResponse, PortsDialogType>('Ports')
