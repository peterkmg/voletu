import { createFileRoute } from '@tanstack/react-router'
import { defineListRoute } from '~/router/define-list-route'
import { paginatedListSearchSchema } from '~/router/search-schemas'
import { Users } from '~/views/system/users'

export const Route = defineListRoute(createFileRoute, '/_authenticated/system/users/')({
  validateSearch: paginatedListSearchSchema,
  component: Users,
})
