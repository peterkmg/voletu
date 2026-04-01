import { acceptanceDocumentHardDelete, acceptanceDocumentSoftDelete } from '~/generated/client'
import { acceptanceDocumentListQueryKey } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceDocumentList'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { AcceptanceLifecycleDialog } from './acceptance-lifecycle-dialog'
import { AcceptanceMutateDialog } from './acceptance-mutate-dialog'
import { useAcceptance } from './acceptance-provider'

const AcceptanceDeleteDialog = createDeleteDialog({
  useEntity: useAcceptance,
  hardDeleteFn: acceptanceDocumentHardDelete,
  softDeleteFn: acceptanceDocumentSoftDelete,
  queryKey: acceptanceDocumentListQueryKey,
  entityLabel: 'documents:acceptance.singular',
  i18nNamespaces: ['common', 'documents'],
})

export const AcceptanceDialogs = createEntityDialogs({
  useEntity: useAcceptance,
  MutateDialog: AcceptanceMutateDialog,
  DeleteDialog: AcceptanceDeleteDialog,
  LifecycleDialog: AcceptanceLifecycleDialog,
  lifecyclePropName: 'action',
})
