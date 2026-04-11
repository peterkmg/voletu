import { createFileRoute } from '@tanstack/react-router'
import { defineListRoute } from '~/router/define-list-route'
import { createEnabledListSearchSchema } from '~/router/search-schemas'
import { ReconciliationPage } from '~/views/internal/reconciliation'

export const Route = defineListRoute(createFileRoute, '/_authenticated/internal/reconciliation/')({
  validateSearch: createEnabledListSearchSchema,
  component: ReconciliationPage,
})
