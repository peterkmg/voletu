import type { AcceptanceResponse } from '~/generated/types'
import { createRowActions } from '~/lib/create-row-actions'
import { useAcceptance } from './acceptance-provider'

export const DataTableRowActions = createRowActions<AcceptanceResponse>({
  useEntity: useAcceptance,
  lifecycle: true,
})
