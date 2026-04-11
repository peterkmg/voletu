import { createFileRoute } from '@tanstack/react-router'
import { defineListRoute } from '~/router/define-list-route'
import { createEnabledListSearchSchema } from '~/router/search-schemas'
import { RailReceiptPage } from '~/views/incoming/rail-receipt'

export const Route = defineListRoute(createFileRoute, '/_authenticated/incoming/rail/')({
  validateSearch: createEnabledListSearchSchema,
  component: RailReceiptPage,
})
