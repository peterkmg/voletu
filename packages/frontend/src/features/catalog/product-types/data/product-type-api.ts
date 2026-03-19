import {
  catalogProductTypeCreate,
  catalogProductTypeHardDelete,
  catalogProductTypeSoftDelete,
  catalogProductTypeUpdate,
} from '~/generated/client'
import { catalogProductTypeListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogProductTypeList'
import { queryClient } from '~/shared/api/query-client'

export type { CreateProductTypeRequest } from '~/generated/types/CreateProductTypeRequest'
export type { ProductTypeResponse } from '~/generated/types/ProductTypeResponse'
export type { UpdateProductTypeRequest } from '~/generated/types/UpdateProductTypeRequest'

export const createProductType = catalogProductTypeCreate
export const updateProductType = catalogProductTypeUpdate
export const softDeleteProductType = catalogProductTypeSoftDelete
export const hardDeleteProductType = catalogProductTypeHardDelete

export function invalidateProductTypes() {
  return queryClient.invalidateQueries({ queryKey: catalogProductTypeListQueryKey() })
}
