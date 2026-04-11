import { createFileRoute } from '@tanstack/react-router'
import { defineListRoute } from '~/router/define-list-route'
import { paginatedListSearchSchema } from '~/router/search-schemas'
import { Warehouses } from '~/views/catalog/warehouses.tsx'

export const Route = defineListRoute(createFileRoute, '/_authenticated/catalog/warehouses/')({
  validateSearch: paginatedListSearchSchema,
  component: Warehouses,
})
