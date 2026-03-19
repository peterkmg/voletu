import {
  reconciliationCreate,
  reconciliationExecute,
  reconciliationHardDelete,
  reconciliationRevert,
  reconciliationSoftDelete,
  reconciliationUpdate,
} from '~/generated/client'
import { reconciliationListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/useReconciliationList'
import { queryClient } from '~/shared/api/query-client'

export type { CreateInventoryReconciliationRequest } from '~/generated/types/CreateInventoryReconciliationRequest'
export type { InventoryReconciliationResponse } from '~/generated/types/InventoryReconciliationResponse'
export type { UpdateInventoryReconciliationRequest } from '~/generated/types/UpdateInventoryReconciliationRequest'

export const createReconciliationDocument = reconciliationCreate
export const updateReconciliationDocument = reconciliationUpdate
export const softDeleteReconciliationDocument = reconciliationSoftDelete
export const hardDeleteReconciliationDocument = reconciliationHardDelete
export const executeReconciliationDocument = reconciliationExecute
export const revertReconciliationDocument = reconciliationRevert

export function invalidateReconciliations() {
  return queryClient.invalidateQueries({ queryKey: reconciliationListQueryKey() })
}
