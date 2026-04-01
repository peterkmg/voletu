import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { PhysicalTransferLifecycleDialog } from './physical-transfer-lifecycle-dialog'
import { PhysicalTransferMutateDialog } from './physical-transfer-mutate-dialog'
import { usePhysicalTransfer } from './physical-transfer-provider'

export const PhysicalTransferDialogs = createEntityDialogs({
  useEntity: usePhysicalTransfer,
  MutateDialog: PhysicalTransferMutateDialog,
  supportsUpdate: false,
  LifecycleDialog: PhysicalTransferLifecycleDialog,
  lifecyclePropName: 'action',
})
