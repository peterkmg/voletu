import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { useProductTypes } from './product-types-provider'

export const ProductTypesPrimaryButtons = createPrimaryButtons({
  useEntity: useProductTypes,
  createLabel: 'catalog:productType.create',
  i18nNamespaces: ['catalog'],
})
