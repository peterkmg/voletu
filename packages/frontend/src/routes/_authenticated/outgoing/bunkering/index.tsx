import { createFileRoute } from '@tanstack/react-router'
import { defineListRoute } from '~/router/define-list-route'
import { createEnabledListSearchSchema } from '~/router/search-schemas'
import { BunkeringPage } from '~/views/outgoing/bunkering'

export const Route = defineListRoute(createFileRoute, '/_authenticated/outgoing/bunkering/')({
  validateSearch: createEnabledListSearchSchema,
  component: BunkeringPage,
})
