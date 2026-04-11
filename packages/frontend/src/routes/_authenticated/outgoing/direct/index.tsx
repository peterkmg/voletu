import { createFileRoute } from '@tanstack/react-router'
import { defineListRoute } from '~/router/define-list-route'
import { createEnabledListSearchSchema } from '~/router/search-schemas'
import { DirectDispatchPage } from '~/views/outgoing/direct-dispatch'

export const Route = defineListRoute(createFileRoute, '/_authenticated/outgoing/direct/')({
  validateSearch: createEnabledListSearchSchema,
  component: DirectDispatchPage,
})
