import { dispatchDocumentHardDelete, dispatchDocumentSoftDelete } from '~/generated/client'
import { dispatchDocumentListQueryKey } from '~/generated/hooks/DocumentDispatchHooks/useDispatchDocumentList'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { DispatchLifecycleDialog } from './dispatch-lifecycle-dialog'
import { DispatchMutateDialog } from './dispatch-mutate-dialog'
import { useDispatch } from './dispatch-provider'

const DispatchDeleteDialog = createDeleteDialog({
  useEntity: useDispatch,
  hardDeleteFn: dispatchDocumentHardDelete,
  softDeleteFn: dispatchDocumentSoftDelete,
  queryKey: dispatchDocumentListQueryKey,
  entityLabel: 'documents:dispatch.singular',
  i18nNamespaces: ['common', 'documents'],
})

export const DispatchDialogs = createEntityDialogs({
  useEntity: useDispatch,
  MutateDialog: DispatchMutateDialog,
  DeleteDialog: DispatchDeleteDialog,
  LifecycleDialog: DispatchLifecycleDialog,
  lifecyclePropName: 'variant',
})
