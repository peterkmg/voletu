import { createFileRoute } from '@tanstack/react-router'
import { defineListRoute } from '~/router/define-list-route'
import { createEnabledListSearchSchema } from '~/router/search-schemas'
import { TruckReceiptPage } from '~/views/incoming/truck-receipt'

export const Route = defineListRoute(createFileRoute, '/_authenticated/incoming/truck/')({
  validateSearch: createEnabledListSearchSchema,
  component: TruckReceiptPage,
})
