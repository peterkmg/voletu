import { createFileRoute } from '@tanstack/react-router'
import { defineListRoute } from '~/router/define-list-route'
import { paginatedListSearchSchema } from '~/router/search-schemas'
import { ProductGroups } from '~/views/catalog/product-groups.tsx'

export const Route = defineListRoute(createFileRoute, '/_authenticated/catalog/product-groups/')({
  validateSearch: paginatedListSearchSchema,
  component: ProductGroups,
})
