import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { useBases } from './bases-provider'

export const BasesPrimaryButtons = createPrimaryButtons({
  useEntity: useBases,
  createLabel: 'catalog:base.create',
  i18nNamespaces: ['catalog'],
})
