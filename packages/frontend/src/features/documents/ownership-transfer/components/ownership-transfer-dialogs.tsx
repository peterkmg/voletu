import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { OwnershipTransferLifecycleDialog } from './ownership-transfer-lifecycle-dialog'
import { OwnershipTransferMutateDialog } from './ownership-transfer-mutate-dialog'
import { useOwnershipTransfer } from './ownership-transfer-provider'

export const OwnershipTransferDialogs = createEntityDialogs({
  useEntity: useOwnershipTransfer,
  MutateDialog: OwnershipTransferMutateDialog,
  supportsUpdate: false,
  LifecycleDialog: OwnershipTransferLifecycleDialog,
  lifecyclePropName: 'action',
})
