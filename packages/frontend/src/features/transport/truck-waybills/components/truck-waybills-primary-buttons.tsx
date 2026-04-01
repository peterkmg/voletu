import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { useTruckWaybills } from './truck-waybills-provider'

export const TruckWaybillsPrimaryButtons = createPrimaryButtons({
  useEntity: useTruckWaybills,
  createLabel: 'transport:truck.create',
  i18nNamespaces: ['transport'],
})
