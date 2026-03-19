import {
  ownershipTransferCreate,
  ownershipTransferExecute,
  ownershipTransferRevert,
} from '~/generated/client'
import { ownershipTransferListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/useOwnershipTransferList'
import { queryClient } from '~/shared/api/query-client'

export type { CreateOwnershipTransferRequest } from '~/generated/types/CreateOwnershipTransferRequest'
export type { OwnershipTransferResponse } from '~/generated/types/OwnershipTransferResponse'

export const createOwnershipTransfer = ownershipTransferCreate
export const executeOwnershipTransfer = ownershipTransferExecute
export const revertOwnershipTransfer = ownershipTransferRevert

export function invalidateOwnershipTransfers() {
  return queryClient.invalidateQueries({ queryKey: ownershipTransferListQueryKey() })
}
