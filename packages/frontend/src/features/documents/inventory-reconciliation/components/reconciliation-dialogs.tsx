import { reconciliationHardDelete, reconciliationSoftDelete } from '~/generated/client'
import { reconciliationListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/useReconciliationList'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { ReconciliationLifecycleDialog } from './reconciliation-lifecycle-dialog'
import { ReconciliationMutateDialog } from './reconciliation-mutate-dialog'
import { useReconciliation } from './reconciliation-provider'

const ReconciliationDeleteDialog = createDeleteDialog({
  useEntity: useReconciliation,
  hardDeleteFn: reconciliationHardDelete,
  softDeleteFn: reconciliationSoftDelete,
  queryKey: reconciliationListQueryKey,
  entityLabel: 'documents:reconciliation.singular',
  i18nNamespaces: ['common', 'documents'],
})

export const ReconciliationDialogs = createEntityDialogs({
  useEntity: useReconciliation,
  MutateDialog: ReconciliationMutateDialog,
  DeleteDialog: ReconciliationDeleteDialog,
  LifecycleDialog: ReconciliationLifecycleDialog,
  lifecyclePropName: 'variant',
})
