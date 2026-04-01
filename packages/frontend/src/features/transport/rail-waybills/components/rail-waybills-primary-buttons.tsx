import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { useRailWaybills } from './rail-waybills-provider'

export const RailWaybillsPrimaryButtons = createPrimaryButtons({
  useEntity: useRailWaybills,
  createLabel: 'transport:rail.create',
  i18nNamespaces: ['transport'],
})
