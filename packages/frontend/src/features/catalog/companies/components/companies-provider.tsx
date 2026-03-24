import type { CompanyResponse } from '~/generated/types'
import { createEntityProvider } from '~/lib/create-entity-provider'

type CompaniesDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

export const { Provider: CompaniesProvider, useEntity: useCompanies }
  = createEntityProvider<CompanyResponse, CompaniesDialogType>('Companies')
