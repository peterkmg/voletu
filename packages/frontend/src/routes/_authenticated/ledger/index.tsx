import { createFileRoute } from '@tanstack/react-router'
import { defineListRoute } from '~/router/define-list-route'
import { paginatedListSearchSchema } from '~/router/search-schemas'
import { Ledger } from '~/views/system/ledger'

export const Route = defineListRoute(createFileRoute, '/_authenticated/ledger/')({
  validateSearch: paginatedListSearchSchema,
  component: Ledger,
})
