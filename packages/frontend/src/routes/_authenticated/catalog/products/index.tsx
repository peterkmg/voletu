import { createFileRoute } from '@tanstack/react-router'
import { defineListRoute } from '~/router/define-list-route'
import { paginatedListSearchSchema } from '~/router/search-schemas'
import { Products } from '~/views/catalog/products.tsx'

export const Route = defineListRoute(createFileRoute, '/_authenticated/catalog/products/')({
  validateSearch: paginatedListSearchSchema,
  component: Products,
})
