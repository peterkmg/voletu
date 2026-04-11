import { createFileRoute } from '@tanstack/react-router'
import { defineListRoute } from '~/router/define-list-route'
import { paginatedListSearchSchema } from '~/router/search-schemas'
import { Bases } from '~/views/catalog/bases.tsx'

export const Route = defineListRoute(createFileRoute, '/_authenticated/catalog/bases/')({
  validateSearch: paginatedListSearchSchema,
  component: Bases,
})
