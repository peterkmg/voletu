import type { CompanyResponse } from '~/generated/types'
import { createRowActions } from '~/lib/create-row-actions'
import { useCompanies } from './companies-provider'

export const DataTableRowActions = createRowActions<CompanyResponse>({
  useEntity: useCompanies,
})
