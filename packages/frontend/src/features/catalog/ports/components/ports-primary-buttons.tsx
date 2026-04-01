import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { usePorts } from './ports-provider'

export const PortsPrimaryButtons = createPrimaryButtons({
  useEntity: usePorts,
  createLabel: 'catalog:port.create',
  i18nNamespaces: ['catalog'],
})
