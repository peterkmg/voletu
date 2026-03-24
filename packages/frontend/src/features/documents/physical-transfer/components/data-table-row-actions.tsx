import type { PhysicalTransferResponse } from '~/generated/types'
import { createRowActions } from '~/lib/create-row-actions'
import { usePhysicalTransfer } from './physical-transfer-provider'

export const DataTableRowActions = createRowActions<PhysicalTransferResponse>({
  useEntity: usePhysicalTransfer,
  lifecycle: true,
})
