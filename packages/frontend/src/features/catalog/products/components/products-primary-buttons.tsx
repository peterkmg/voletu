import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { useProducts } from './products-provider'

export const ProductsPrimaryButtons = createPrimaryButtons({
  useEntity: useProducts,
  createLabel: 'catalog:product.create',
  i18nNamespaces: ['catalog'],
})
