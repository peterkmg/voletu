import { createFileRoute } from '@tanstack/react-router'
import { defineListRoute } from '~/router/define-list-route'
import { paginatedListSearchSchema } from '~/router/search-schemas'
import { Storages } from '~/views/catalog/storages.tsx'

export const Route = defineListRoute(createFileRoute, '/_authenticated/catalog/storages/')({
  validateSearch: paginatedListSearchSchema,
  component: Storages,
})
