import type { PhysicalTransferResponse } from '~/generated/types'
import { createEntityProvider } from '~/lib/create-entity-provider'

type PhysicalTransferDialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

export const { Provider: PhysicalTransferProvider, useEntity: usePhysicalTransfer }
  = createEntityProvider<PhysicalTransferResponse, PhysicalTransferDialogType>('PhysicalTransfer')
