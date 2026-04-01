import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { useDispatch } from './dispatch-provider'

export const DispatchPrimaryButtons = createPrimaryButtons({
  useEntity: useDispatch,
  createLabel: 'documents:dispatch.create',
  i18nNamespaces: ['documents'],
})
