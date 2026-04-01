import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { useOwnershipTransfer } from './ownership-transfer-provider'

export const OwnershipTransferPrimaryButtons = createPrimaryButtons({
  useEntity: useOwnershipTransfer,
  createLabel: 'documents:ownershipTransfer.create',
  i18nNamespaces: ['documents'],
})
