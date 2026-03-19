import {
  physicalTransferCreate,
  physicalTransferExecute,
  physicalTransferRevert,
} from '~/generated/client'
import { physicalTransferListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/usePhysicalTransferList'
import { queryClient } from '~/shared/api/query-client'

export type { CreatePhysicalTransferRequest } from '~/generated/types/CreatePhysicalTransferRequest'
export type { PhysicalTransferResponse } from '~/generated/types/PhysicalTransferResponse'

export const createPhysicalTransfer = physicalTransferCreate
export const executePhysicalTransfer = physicalTransferExecute
export const revertPhysicalTransfer = physicalTransferRevert

export function invalidatePhysicalTransfers() {
  return queryClient.invalidateQueries({ queryKey: physicalTransferListQueryKey() })
}
