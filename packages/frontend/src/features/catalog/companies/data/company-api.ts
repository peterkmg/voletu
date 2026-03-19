import {
  catalogCompanyCreate,
  catalogCompanyHardDelete,
  catalogCompanySoftDelete,
  catalogCompanyUpdate,
} from '~/generated/client'
import { catalogCompanyListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { queryClient } from '~/shared/api/query-client'

export type { CompanyResponse } from '~/generated/types/CompanyResponse'
export type { CreateCompanyRequest } from '~/generated/types/CreateCompanyRequest'
export type { UpdateCompanyRequest } from '~/generated/types/UpdateCompanyRequest'

export const createCompany = catalogCompanyCreate
export const updateCompany = catalogCompanyUpdate
export const softDeleteCompany = catalogCompanySoftDelete
export const hardDeleteCompany = catalogCompanyHardDelete

export function invalidateCompanies() {
  return queryClient.invalidateQueries({ queryKey: catalogCompanyListQueryKey() })
}
