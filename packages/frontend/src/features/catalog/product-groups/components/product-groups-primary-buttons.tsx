import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { useProductGroups } from './product-groups-provider'

export const ProductGroupsPrimaryButtons = createPrimaryButtons({
  useEntity: useProductGroups,
  createLabel: 'catalog:productGroup.create',
  i18nNamespaces: ['catalog'],
})
