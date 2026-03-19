/**
 * Bases API adapter — thin wrapper over kubb-generated client functions.
 * Mutation functions keep their original names so callers need no changes.
 * The list query is handled via useCatalogBaseList() in feature index files.
 */
import {
  catalogBaseCreate,
  catalogBaseHardDelete,
  catalogBaseSoftDelete,
  catalogBaseUpdate,
} from '~/generated/client'
import { catalogBaseListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogBaseList'
import { queryClient } from '~/shared/api/query-client'

export type { BaseResponse } from '~/generated/types/BaseResponse'
export type { CreateBaseRequest } from '~/generated/types/CreateBaseRequest'
export type { UpdateBaseRequest } from '~/generated/types/UpdateBaseRequest'

export const createBase = catalogBaseCreate
export const updateBase = catalogBaseUpdate
export const softDeleteBase = catalogBaseSoftDelete
export const hardDeleteBase = catalogBaseHardDelete

export function invalidateBases() {
  return queryClient.invalidateQueries({ queryKey: catalogBaseListQueryKey() })
}
