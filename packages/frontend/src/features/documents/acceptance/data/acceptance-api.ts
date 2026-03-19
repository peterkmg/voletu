import {
  acceptanceDocumentCreate,
  acceptanceDocumentExecute,
  acceptanceDocumentHardDelete,
  acceptanceDocumentRevert,
  acceptanceDocumentSoftDelete,
  acceptanceDocumentUpdate,
} from '~/generated/client'
import { acceptanceDocumentListQueryKey } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceDocumentList'
import { queryClient } from '~/shared/api/query-client'

export type { AcceptanceResponse } from '~/generated/types/AcceptanceResponse'
export type { CreateAcceptanceRequest } from '~/generated/types/CreateAcceptanceRequest'
export type { UpdateAcceptanceRequest } from '~/generated/types/UpdateAcceptanceRequest'

export const createAcceptanceDocument = acceptanceDocumentCreate
export const updateAcceptanceDocument = acceptanceDocumentUpdate
export const softDeleteAcceptanceDocument = acceptanceDocumentSoftDelete
export const hardDeleteAcceptanceDocument = acceptanceDocumentHardDelete
export const executeAcceptanceDocument = acceptanceDocumentExecute
export const revertAcceptanceDocument = acceptanceDocumentRevert

export function invalidateAcceptanceDocuments() {
  return queryClient.invalidateQueries({ queryKey: acceptanceDocumentListQueryKey() })
}
