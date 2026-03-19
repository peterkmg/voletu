import {
  catalogProductGroupCreate,
  catalogProductGroupHardDelete,
  catalogProductGroupSoftDelete,
  catalogProductGroupUpdate,
} from '~/generated/client'
import { catalogProductGroupListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogProductGroupList'
import { queryClient } from '~/shared/api/query-client'

export type { CreateProductGroupRequest } from '~/generated/types/CreateProductGroupRequest'
export type { ProductGroupResponse } from '~/generated/types/ProductGroupResponse'
export type { UpdateProductGroupRequest } from '~/generated/types/UpdateProductGroupRequest'

export const createProductGroup = catalogProductGroupCreate
export const updateProductGroup = catalogProductGroupUpdate
export const softDeleteProductGroup = catalogProductGroupSoftDelete
export const hardDeleteProductGroup = catalogProductGroupHardDelete

export function invalidateProductGroups() {
  return queryClient.invalidateQueries({ queryKey: catalogProductGroupListQueryKey() })
}
