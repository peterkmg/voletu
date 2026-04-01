import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { usePhysicalTransfer } from './physical-transfer-provider'

export const PhysicalTransferPrimaryButtons = createPrimaryButtons({
  useEntity: usePhysicalTransfer,
  createLabel: 'documents:physicalTransfer.create',
  i18nNamespaces: ['documents'],
})
