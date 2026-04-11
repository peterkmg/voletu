import { createFileRoute } from '@tanstack/react-router'
import { defineListRoute } from '~/router/define-list-route'
import { createEnabledListSearchSchema } from '~/router/search-schemas'
import { ExternalAcceptancePage } from '~/views/incoming/external-acceptance'

export const Route = defineListRoute(createFileRoute, '/_authenticated/incoming/external/')({
  validateSearch: createEnabledListSearchSchema,
  component: ExternalAcceptancePage,
})
