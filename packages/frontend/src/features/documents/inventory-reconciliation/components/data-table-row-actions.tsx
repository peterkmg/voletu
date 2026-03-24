import type { InventoryReconciliationResponse } from '~/generated/types'
import { createRowActions } from '~/lib/create-row-actions'
import { useReconciliation } from './reconciliation-provider'

export const DataTableRowActions = createRowActions<InventoryReconciliationResponse>({
  useEntity: useReconciliation,
  lifecycle: true,
})
