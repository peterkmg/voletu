import {
  catalogProductCreate,
  catalogProductHardDelete,
  catalogProductSoftDelete,
  catalogProductUpdate,
} from '~/generated/client'
import { catalogProductListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogProductList'
import { queryClient } from '~/shared/api/query-client'

export type { CreateProductRequest } from '~/generated/types/CreateProductRequest'
export type { ProductResponse } from '~/generated/types/ProductResponse'
export type { UpdateProductRequest } from '~/generated/types/UpdateProductRequest'

export const createProduct = catalogProductCreate
export const updateProduct = catalogProductUpdate
export const softDeleteProduct = catalogProductSoftDelete
export const hardDeleteProduct = catalogProductHardDelete

export function invalidateProducts() {
  return queryClient.invalidateQueries({ queryKey: catalogProductListQueryKey() })
}
