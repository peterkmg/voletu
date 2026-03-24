import type { OwnershipTransferResponse } from '~/generated/types'
import { createRowActions } from '~/lib/create-row-actions'
import { useOwnershipTransfer } from './ownership-transfer-provider'

export const DataTableRowActions = createRowActions<OwnershipTransferResponse>({
  useEntity: useOwnershipTransfer,
  lifecycle: true,
})
