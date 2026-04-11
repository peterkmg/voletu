import { createFileRoute } from '@tanstack/react-router'
import { defineListRoute } from '~/router/define-list-route'
import { paginatedListSearchSchema } from '~/router/search-schemas'
import { ProductTypes } from '~/views/catalog/product-types.tsx'

export const Route = defineListRoute(createFileRoute, '/_authenticated/catalog/product-types/')({
  validateSearch: paginatedListSearchSchema,
  component: ProductTypes,
})
