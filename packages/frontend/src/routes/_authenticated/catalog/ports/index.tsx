import { createFileRoute } from '@tanstack/react-router'
import { defineListRoute } from '~/router/define-list-route'
import { paginatedListSearchSchema } from '~/router/search-schemas'
import { Ports } from '~/views/catalog/ports.tsx'

export const Route = defineListRoute(createFileRoute, '/_authenticated/catalog/ports/')({
  validateSearch: paginatedListSearchSchema,
  component: Ports,
})
