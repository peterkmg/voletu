import {
  dispatchDocumentCreate,
  dispatchDocumentExecute,
  dispatchDocumentHardDelete,
  dispatchDocumentRevert,
  dispatchDocumentSoftDelete,
  dispatchDocumentUpdate,
} from '~/generated/client'
import { dispatchDocumentListQueryKey } from '~/generated/hooks/DocumentDispatchHooks/useDispatchDocumentList'
import { queryClient } from '~/shared/api/query-client'

export type { CreateDispatchRequest } from '~/generated/types/CreateDispatchRequest'
export type { DispatchResponse } from '~/generated/types/DispatchResponse'
export type { UpdateDispatchRequest } from '~/generated/types/UpdateDispatchRequest'

export const createDispatchDocument = dispatchDocumentCreate
export const updateDispatchDocument = dispatchDocumentUpdate
export const softDeleteDispatchDocument = dispatchDocumentSoftDelete
export const hardDeleteDispatchDocument = dispatchDocumentHardDelete
export const executeDispatchDocument = dispatchDocumentExecute
export const revertDispatchDocument = dispatchDocumentRevert

export function invalidateDispatchDocuments() {
  return queryClient.invalidateQueries({ queryKey: dispatchDocumentListQueryKey() })
}
