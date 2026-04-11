import { createFileRoute } from '@tanstack/react-router'
import { defineListRoute } from '~/router/define-list-route'
import { paginatedListSearchSchema } from '~/router/search-schemas'
import { Companies } from '~/views/catalog/companies.tsx'

export const Route = defineListRoute(createFileRoute, '/_authenticated/catalog/companies/')({
  validateSearch: paginatedListSearchSchema,
  component: Companies,
})
