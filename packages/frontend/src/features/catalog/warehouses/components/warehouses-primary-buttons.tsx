import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { useWarehouses } from './warehouses-provider'

export const WarehousesPrimaryButtons = createPrimaryButtons({
  useEntity: useWarehouses,
  createLabel: 'catalog:warehouse.create',
  i18nNamespaces: ['catalog'],
})
