import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { useAcceptance } from './acceptance-provider'

export const AcceptancePrimaryButtons = createPrimaryButtons({
  useEntity: useAcceptance,
  createLabel: 'documents:acceptance.create',
  i18nNamespaces: ['documents'],
})
