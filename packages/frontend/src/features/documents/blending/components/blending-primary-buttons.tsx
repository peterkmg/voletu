import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { useBlending } from './blending-provider'

export const BlendingPrimaryButtons = createPrimaryButtons({
  useEntity: useBlending,
  createLabel: 'documents:blending.create',
  i18nNamespaces: ['documents'],
})
