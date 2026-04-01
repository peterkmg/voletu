import { blendingDocumentHardDelete, blendingDocumentSoftDelete } from '~/generated/client'
import { blendingDocumentListQueryKey } from '~/generated/hooks/DocumentOperationsHooks/useBlendingDocumentList'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { BlendingLifecycleDialog } from './blending-lifecycle-dialog'
import { BlendingMutateDialog } from './blending-mutate-dialog'
import { useBlending } from './blending-provider'

const BlendingDeleteDialog = createDeleteDialog({
  useEntity: useBlending,
  hardDeleteFn: blendingDocumentHardDelete,
  softDeleteFn: blendingDocumentSoftDelete,
  queryKey: blendingDocumentListQueryKey,
  entityLabel: 'documents:blending.singular',
  i18nNamespaces: ['common', 'documents'],
})

export const BlendingDialogs = createEntityDialogs({
  useEntity: useBlending,
  MutateDialog: BlendingMutateDialog,
  DeleteDialog: BlendingDeleteDialog,
  LifecycleDialog: BlendingLifecycleDialog,
  lifecyclePropName: 'variant',
})
