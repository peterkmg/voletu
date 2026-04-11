import { createFileRoute } from '@tanstack/react-router'
import { defineListRoute } from '~/router/define-list-route'
import { createEnabledListSearchSchema } from '~/router/search-schemas'
import { TruckDispatchPage } from '~/views/outgoing/truck-dispatch'

export const Route = defineListRoute(createFileRoute, '/_authenticated/outgoing/truck/')({
  validateSearch: createEnabledListSearchSchema,
  component: TruckDispatchPage,
})
