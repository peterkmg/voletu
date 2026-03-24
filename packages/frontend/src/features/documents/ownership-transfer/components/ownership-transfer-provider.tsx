import type { OwnershipTransferResponse } from '~/generated/types'
import { createEntityProvider } from '~/lib/create-entity-provider'

type OwnershipTransferDialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

export const { Provider: OwnershipTransferProvider, useEntity: useOwnershipTransfer }
  = createEntityProvider<OwnershipTransferResponse, OwnershipTransferDialogType>('OwnershipTransfer')
