import {
  blendingDocumentCreate,
  blendingDocumentExecute,
  blendingDocumentHardDelete,
  blendingDocumentRevert,
  blendingDocumentSoftDelete,
  blendingDocumentUpdate,
} from '~/generated/client'
import { blendingDocumentListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/useBlendingDocumentList'
import { queryClient } from '~/shared/api/query-client'

export type { BlendingResponse } from '~/generated/types/BlendingResponse'
export type { CreateBlendingRequest } from '~/generated/types/CreateBlendingRequest'
export type { UpdateBlendingRequest } from '~/generated/types/UpdateBlendingRequest'

export const createBlendingDocument = blendingDocumentCreate
export const updateBlendingDocument = blendingDocumentUpdate
export const softDeleteBlendingDocument = blendingDocumentSoftDelete
export const hardDeleteBlendingDocument = blendingDocumentHardDelete
export const executeBlendingDocument = blendingDocumentExecute
export const revertBlendingDocument = blendingDocumentRevert

export function invalidateBlendingDocuments() {
  return queryClient.invalidateQueries({ queryKey: blendingDocumentListQueryKey() })
}
