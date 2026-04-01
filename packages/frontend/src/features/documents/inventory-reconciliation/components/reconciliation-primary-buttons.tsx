import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { useReconciliation } from './reconciliation-provider'

export const ReconciliationPrimaryButtons = createPrimaryButtons({
  useEntity: useReconciliation,
  createLabel: 'documents:reconciliation.create',
  i18nNamespaces: ['documents'],
})
