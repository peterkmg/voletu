import type { InventoryReconciliationResponse } from '~/generated/types'
import { createEntityProvider } from '~/lib/create-entity-provider'

type ReconciliationDialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

export const { Provider: ReconciliationProvider, useEntity: useReconciliation }
  = createEntityProvider<InventoryReconciliationResponse, ReconciliationDialogType>('Reconciliation')
